use serde_json::Value;
use std::fs;

fn load_snippets() -> Value {
    let content =
        fs::read_to_string("snippets/liquid.json").expect("snippets/liquid.json not found");
    serde_json::from_str(&content).expect("snippets/liquid.json is not valid JSON")
}

// prefix may be a string or an array of strings
fn get_prefixes(snippet: &Value) -> Vec<String> {
    match &snippet["prefix"] {
        Value::String(s) => vec![s.clone()],
        Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect(),
        _ => vec![],
    }
}

#[test]
fn snippets_json_is_valid_object() {
    let snippets = load_snippets();
    assert!(snippets.is_object());
    assert!(!snippets.as_object().unwrap().is_empty());
}

#[test]
fn every_snippet_has_required_fields() {
    let snippets = load_snippets();
    for (name, snippet) in snippets.as_object().unwrap() {
        assert!(
            snippet["prefix"].is_string() || snippet["prefix"].is_array(),
            "snippet '{name}' missing 'prefix'"
        );
        assert!(
            snippet["body"].is_array(),
            "snippet '{name}' body must be an array"
        );
        assert!(
            snippet["description"].is_string(),
            "snippet '{name}' missing 'description'"
        );
        let body = snippet["body"].as_array().unwrap();
        assert!(!body.is_empty(), "snippet '{name}' has an empty body");
        for line in body {
            assert!(
                line.is_string(),
                "snippet '{name}' body lines must be strings"
            );
        }
    }
}

#[test]
fn every_snippet_prefix_is_non_empty() {
    let snippets = load_snippets();
    for (name, snippet) in snippets.as_object().unwrap() {
        let prefixes = get_prefixes(snippet);
        assert!(!prefixes.is_empty(), "snippet '{name}' has no prefixes");
        for prefix in &prefixes {
            assert!(!prefix.is_empty(), "snippet '{name}' has an empty prefix");
        }
    }
}

#[test]
fn core_control_flow_snippets_exist() {
    let snippets = load_snippets();
    let obj = snippets.as_object().unwrap();
    for name in ["if", "for", "unless", "assign", "capture", "case/when"] {
        assert!(obj.contains_key(name), "missing core snippet: '{name}'");
    }
}

#[test]
fn core_shopify_snippets_exist() {
    let snippets = load_snippets();
    let obj = snippets.as_object().unwrap();
    for name in [
        "render", "section", "schema", "paginate", "form", "layout", "tablerow", "include",
    ] {
        assert!(obj.contains_key(name), "missing Shopify snippet: '{name}'");
    }
}

#[test]
fn block_snippets_have_matching_end_tags() {
    let snippets = load_snippets();
    let pairs = [
        ("if", "endif"),
        ("for", "endfor"),
        ("unless", "endunless"),
        ("capture", "endcapture"),
        ("paginate", "endpaginate"),
        ("schema", "endschema"),
        ("javascript", "endjavascript"),
        ("stylesheet", "endstylesheet"),
        ("style", "endstyle"),
        ("comment", "endcomment"),
        ("raw", "endraw"),
        ("form", "endform"),
        ("tablerow", "endtablerow"),
        ("unless/else", "endunless"),
    ];
    for (name, end_tag) in pairs {
        let snippet = &snippets[name];
        let body: String = snippet["body"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            body.contains(&format!("{{% {end_tag} %}}"))
                || body.contains(&format!("{{% {end_tag}%}}")),
            "snippet '{name}' missing closing '{{% {end_tag} %}}'"
        );
    }
}

#[test]
fn output_snippets_use_double_braces() {
    let snippets = load_snippets();
    for name in ["output variable", "output with filter", "t filter"] {
        let snippet = &snippets[name];
        let body: String = snippet["body"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            body.contains("{{") && body.contains("}}"),
            "snippet '{name}' must use {{{{ … }}}}"
        );
    }
}

#[test]
fn tab_stops_are_valid() {
    let snippets = load_snippets();
    for (name, snippet) in snippets.as_object().unwrap() {
        let body: String = snippet["body"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        // $0 through $9 and ${N:placeholder} are valid VS Code snippet tab stops
        // Ensure no malformed tab stops like lone $ at end of line
        for (i, line) in body.lines().enumerate() {
            let trimmed = line.trim_end();
            if trimmed.ends_with('$') {
                // A trailing $ with nothing after it is suspicious
                panic!("snippet '{name}' line {i} ends with bare '$': {trimmed:?}");
            }
        }
    }
}

#[test]
fn no_duplicate_prefixes() {
    let snippets = load_snippets();
    let mut seen: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for (name, snippet) in snippets.as_object().unwrap() {
        for prefix in get_prefixes(snippet) {
            if let Some(existing) = seen.get(&prefix) {
                panic!("duplicate prefix '{prefix}' used by both '{existing}' and '{name}'");
            }
            seen.insert(prefix, name.clone());
        }
    }
}

#[test]
fn liquid_tags_are_properly_closed() {
    let snippets = load_snippets();
    for (name, snippet) in snippets.as_object().unwrap() {
        let body: String = snippet["body"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let open_logic = body.matches("{%").count();
        let close_logic = body.matches("%}").count();
        assert_eq!(
            open_logic, close_logic,
            "snippet '{name}' has mismatched {{% and %}} tags"
        );

        // Note: `{{ }}` balance is NOT checked here because VS Code tab stop syntax
        // (e.g. `${2:, ${3:object}}`) contains `}}` that would cause false positives.
    }
}

#[test]
fn prefixes_do_not_contain_spaces() {
    let snippets = load_snippets();
    for (name, snippet) in snippets.as_object().unwrap() {
        for prefix in get_prefixes(snippet) {
            assert!(
                !prefix.contains(' '),
                "snippet '{name}' has invalid prefix '{prefix}': prefixes should not contain spaces"
            );
        }
    }
}

#[test]
fn every_snippet_description_is_non_empty() {
    let snippets = load_snippets();
    for (name, snippet) in snippets.as_object().unwrap() {
        let desc = snippet["description"].as_str().unwrap();
        assert!(
            !desc.trim().is_empty(),
            "snippet '{name}' has an empty description"
        );
    }
}

#[test]
fn body_does_not_have_trailing_or_leading_empty_lines() {
    let snippets = load_snippets();
    for (name, snippet) in snippets.as_object().unwrap() {
        let body = snippet["body"].as_array().unwrap();

        if body.len() > 1 {
            let first_line = body.first().unwrap().as_str().unwrap();
            let last_line = body.last().unwrap().as_str().unwrap();

            assert!(
                !first_line.trim().is_empty(),
                "snippet '{name}' has a leading empty line in its body"
            );
            assert!(
                !last_line.trim().is_empty(),
                "snippet '{name}' has a trailing empty line in its body"
            );
        }
    }
}

#[test]
fn block_snippets_have_final_tab_stop() {
    let snippets = load_snippets();
    let block_names = [
        "if",
        "for",
        "unless",
        "unless/else",
        "case/when",
        "paginate",
        "form",
        "tablerow",
    ];

    let obj = snippets.as_object().unwrap();

    for name in block_names {
        if let Some(snippet) = obj.get(name) {
            let body: String = snippet["body"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join("\n");

            assert!(
                body.contains("$0"),
                "block snippet '{name}' should have a final cursor position ($0)"
            );
        }
    }
}
