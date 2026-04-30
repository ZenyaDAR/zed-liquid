use std::{env, fs};
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result};

const PACKAGE_NAME: &str = "@shopify/theme-language-server-node";
const PACKAGE_ENTRY: &str =
    "node_modules/@shopify/theme-language-server-node/dist/index.js";
const SERVER_WRAPPER: &str = "run_server.js";

struct LiquidExtension {
    cached_binary_path: Option<String>,
}

impl LiquidExtension {
    fn server_installed(&self) -> bool {
        fs::metadata(PACKAGE_ENTRY).is_ok_and(|m| m.is_file())
    }

    fn ensure_server(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if self.cached_binary_path.is_some() && self.server_installed() {
            return Ok(SERVER_WRAPPER.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let needs_install = !self.server_installed()
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version);

        if needs_install {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_installed() {
                        Err(format!(
                            "Installed '{PACKAGE_NAME}' but '{PACKAGE_ENTRY}' not found"
                        ))?;
                    }
                }
                Err(e) => {
                    if !self.server_installed() {
                        Err(e)?;
                    }
                }
            }
        }

        fs::write(
            SERVER_WRAPPER,
            "require('@shopify/theme-language-server-node').startServer();",
        )
        .map_err(|e| format!("Could not write server wrapper: {e}"))?;

        self.cached_binary_path = Some(SERVER_WRAPPER.to_string());
        Ok(SERVER_WRAPPER.to_string())
    }
}

impl zed::Extension for LiquidExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Prefer Shopify CLI if installed globally — avoids npm install step
        if let Some(shopify_bin) = worktree.which("shopify") {
            return Ok(zed::Command {
                command: shopify_bin,
                args: vec!["theme".to_string(), "language-server".to_string()],
                env: Default::default(),
            });
        }

        // Fall back: install the npm package and run via a tiny wrapper script
        let wrapper = self.ensure_server(language_server_id)?;
        let abs_wrapper = env::current_dir()
            .map_err(|e| format!("Cannot determine working directory: {e}"))?
            .join(&wrapper)
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![abs_wrapper],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let user_options = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.initialization_options);

        Ok(user_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let user_settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.settings);

        Ok(Some(merge_workspace_config(user_settings)))
    }
}

fn merge_workspace_config(
    user_settings: Option<zed::serde_json::Value>,
) -> zed::serde_json::Value {
    let defaults = zed::serde_json::json!({
        "shopifyLiquid": {
            "onlySingleFileChecks": false,
            "disabledChecks": []
        }
    });

    match user_settings {
        Some(zed::serde_json::Value::Object(mut user_map)) => {
            if let zed::serde_json::Value::Object(default_map) = defaults {
                for (k, v) in default_map {
                    user_map.entry(k).or_insert(v);
                }
            }
            zed::serde_json::Value::Object(user_map)
        }
        Some(other) => other,
        None => defaults,
    }
}

zed::register_extension!(LiquidExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::serde_json::json;

    #[test]
    fn defaults_returned_when_no_user_settings() {
        let result = merge_workspace_config(None);
        assert_eq!(result["shopifyLiquid"]["onlySingleFileChecks"], false);
        assert!(result["shopifyLiquid"]["disabledChecks"].is_array());
        assert!(result["shopifyLiquid"]["disabledChecks"]
            .as_array()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn user_object_gets_defaults_inserted_for_missing_keys() {
        let user = json!({ "myExtension": { "enabled": true } });
        let result = merge_workspace_config(Some(user));
        assert_eq!(result["myExtension"]["enabled"], true);
        assert_eq!(result["shopifyLiquid"]["onlySingleFileChecks"], false);
    }

    #[test]
    fn user_shopify_liquid_key_is_not_overwritten_by_defaults() {
        let user = json!({
            "shopifyLiquid": {
                "onlySingleFileChecks": true,
                "disabledChecks": ["MissingTemplate"]
            }
        });
        let result = merge_workspace_config(Some(user));
        assert_eq!(result["shopifyLiquid"]["onlySingleFileChecks"], true);
        assert_eq!(result["shopifyLiquid"]["disabledChecks"][0], "MissingTemplate");
    }

    #[test]
    fn non_object_user_settings_returned_unchanged() {
        let result = merge_workspace_config(Some(json!(null)));
        assert!(result.is_null());
    }
}
