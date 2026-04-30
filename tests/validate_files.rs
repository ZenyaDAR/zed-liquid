use std::fs;
use std::path::Path;

#[test]
fn required_files_exist() {
    let files = [
        "extension.toml",
        "Cargo.toml",
        "src/lib.rs",
        "languages/liquid/config.toml",
        "languages/liquid/highlights.scm",
        "languages/liquid/injections.scm",
        "snippets/liquid.json",
        "LICENSE",
        "README.md",
    ];
    for path in files {
        assert!(Path::new(path).exists(), "required file missing: {path}");
    }
}

#[test]
fn extension_toml_has_correct_structure() {
    let content = fs::read_to_string("extension.toml").unwrap();
    assert!(content.contains(r#"id = "liquid""#), "missing id");
    assert!(content.contains("[grammars.liquid]"), "missing grammar declaration");
    assert!(
        content.contains("[language_servers.shopify-theme-ls]"),
        "missing language server declaration"
    );
    assert!(
        content.contains(r#"languages = ["Liquid"]"#),
        "language server not bound to Liquid"
    );
    assert!(
        content.contains("hankthetank27/tree-sitter-liquid"),
        "wrong grammar repository"
    );
}

#[test]
fn extension_toml_version_is_semver() {
    let content = fs::read_to_string("extension.toml").unwrap();
    let version_line = content
        .lines()
        .find(|l| l.starts_with("version"))
        .expect("no version line in extension.toml");
    // Expect: version = "X.Y.Z"
    let version = version_line
        .split('"')
        .nth(1)
        .expect("version not quoted");
    let parts: Vec<&str> = version.split('.').collect();
    assert_eq!(parts.len(), 3, "version must be X.Y.Z, got: {version}");
    for part in &parts {
        part.parse::<u32>()
            .unwrap_or_else(|_| panic!("non-numeric version component: {part}"));
    }
}

#[test]
fn config_toml_specifies_liquid_grammar_and_suffix() {
    let content = fs::read_to_string("languages/liquid/config.toml").unwrap();
    assert!(
        content.contains(r#"grammar = "liquid""#),
        "config.toml must reference the 'liquid' grammar"
    );
    assert!(
        content.contains(r#"path_suffixes = ["liquid"]"#),
        "config.toml must associate .liquid files"
    );
}

#[test]
fn config_toml_has_block_comment() {
    let content = fs::read_to_string("languages/liquid/config.toml").unwrap();
    assert!(
        content.contains("{% comment %}"),
        "block_comment start should be {{% comment %}}"
    );
    assert!(
        content.contains("{% endcomment %}"),
        "block_comment end should be {{% endcomment %}}"
    );
}

#[test]
fn highlights_contains_all_required_capture_types() {
    let content = fs::read_to_string("languages/liquid/highlights.scm").unwrap();
    let captures = [
        "@keyword",
        "@keyword.operator",
        "@string",
        "@comment",
        "@number",
        "@boolean",
        "@operator",
        "@function.call",
        "@punctuation.bracket",
        "@punctuation.delimiter",
        "@tag.delimiter",
        "@variable",
    ];
    for capture in captures {
        assert!(
            content.contains(capture),
            "highlights.scm missing capture type: {capture}"
        );
    }
}

#[test]
fn highlights_covers_core_liquid_keywords() {
    let content = fs::read_to_string("languages/liquid/highlights.scm").unwrap();
    for keyword in ["if", "for", "unless", "assign", "render", "schema", "capture"] {
        assert!(
            content.contains(&format!("\"{keyword}\"")),
            "highlights.scm missing keyword: {keyword}"
        );
    }
}

#[test]
fn injections_covers_all_embedded_languages() {
    let content = fs::read_to_string("languages/liquid/injections.scm").unwrap();
    let expected = ["html", "json", "css", "javascript", "yaml"];
    for lang in expected {
        assert!(
            content.contains(&format!("\"{lang}\"")),
            "injections.scm missing injection for: {lang}"
        );
    }
}

#[test]
fn injections_uses_combined_flag_for_block_languages() {
    let content = fs::read_to_string("languages/liquid/injections.scm").unwrap();
    let combined_count = content.matches("injection.combined").count();
    // html, json, css (style), css (stylesheet), javascript = 5 combined injections
    assert!(
        combined_count >= 5,
        "expected at least 5 combined injections, found {combined_count}"
    );
}

#[test]
fn lib_rs_registers_extension() {
    let content = fs::read_to_string("src/lib.rs").unwrap();
    assert!(
        content.contains("register_extension!(LiquidExtension)"),
        "lib.rs must call register_extension!"
    );
    assert!(
        content.contains("impl zed::Extension for LiquidExtension"),
        "LiquidExtension must implement zed::Extension"
    );
}

#[test]
fn lib_rs_implements_both_lsp_startup_strategies() {
    let content = fs::read_to_string("src/lib.rs").unwrap();
    // Strategy 1: Shopify CLI
    assert!(
        content.contains("shopify"),
        "lib.rs must try Shopify CLI as primary strategy"
    );
    assert!(
        content.contains("language-server"),
        "lib.rs must pass 'language-server' arg to shopify CLI"
    );
    // Strategy 2: npm fallback
    assert!(
        content.contains("npm_install_package"),
        "lib.rs must fall back to npm install"
    );
    assert!(
        content.contains("node_binary_path"),
        "lib.rs must use bundled Node for npm fallback"
    );
}
