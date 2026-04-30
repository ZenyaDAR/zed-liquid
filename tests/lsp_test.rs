use std::fs;

fn read_lib() -> String {
    fs::read_to_string("src/lib.rs").expect("src/lib.rs not found")
}

#[test]
fn package_name_is_official_shopify_ls() {
    let src = read_lib();
    assert!(
        src.contains("@shopify/theme-language-server-node"),
        "PACKAGE_NAME must be the official Shopify Theme Language Server npm package"
    );
}

#[test]
fn package_entry_is_under_node_modules() {
    let src = read_lib();
    assert!(
        src.contains("node_modules/@shopify/theme-language-server-node"),
        "PACKAGE_ENTRY must point inside node_modules"
    );
    assert!(
        src.contains("dist/index.js"),
        "PACKAGE_ENTRY must point to dist/index.js"
    );
}

#[test]
fn server_wrapper_calls_start_server() {
    let src = read_lib();
    assert!(
        src.contains("startServer()"),
        "server wrapper JS must call startServer()"
    );
    assert!(
        src.contains("require('@shopify/theme-language-server-node')"),
        "server wrapper must require the Shopify LS package"
    );
}

#[test]
fn shopify_cli_used_as_primary_strategy() {
    let src = read_lib();
    assert!(
        src.contains(r#"worktree.which("shopify")"#),
        "primary strategy must probe for the shopify CLI via worktree.which"
    );
    assert!(
        src.contains(r#""theme""#) && src.contains(r#""language-server""#),
        "shopify CLI must be invoked with 'theme language-server' subcommands"
    );
}

#[test]
fn npm_fallback_uses_bundled_node() {
    let src = read_lib();
    assert!(
        src.contains("npm_install_package"),
        "fallback must install the package via npm"
    );
    assert!(
        src.contains("node_binary_path"),
        "fallback must run the wrapper with Zed's bundled Node binary"
    );
}

#[test]
fn workspace_config_defaults_contain_shopify_liquid_key() {
    let src = read_lib();
    assert!(
        src.contains(r#""shopifyLiquid""#),
        "workspace config defaults must include a 'shopifyLiquid' key"
    );
    assert!(
        src.contains(r#""onlySingleFileChecks""#),
        "workspace config defaults must set onlySingleFileChecks"
    );
    assert!(
        src.contains(r#""disabledChecks""#),
        "workspace config defaults must include disabledChecks"
    );
}

#[test]
fn workspace_config_disables_single_file_checks_by_default() {
    let src = read_lib();
    assert!(
        src.contains(r#""onlySingleFileChecks": false"#),
        "onlySingleFileChecks must default to false so cross-file checks run"
    );
}

#[test]
fn workspace_config_merges_user_settings_without_clobbering_defaults() {
    let src = read_lib();
    assert!(
        src.contains("or_insert"),
        "merge logic must use or_insert so user keys take precedence over defaults"
    );
}

#[test]
fn error_message_names_missing_package_entry() {
    let src = read_lib();
    assert!(
        src.contains("PACKAGE_ENTRY"),
        "install error message must reference PACKAGE_ENTRY so users know what file is missing"
    );
}

#[test]
fn extension_exposes_initialization_options_passthrough() {
    let src = read_lib();
    assert!(
        src.contains("language_server_initialization_options"),
        "extension must implement language_server_initialization_options for user overrides"
    );
    assert!(
        src.contains("initialization_options"),
        "initialization_options from LspSettings must be forwarded to the server"
    );
}
