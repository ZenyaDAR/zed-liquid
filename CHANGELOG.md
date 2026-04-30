# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Code folding (`folds.scm`) for all Liquid block tags: `if`, `for`, `unless`, `capture`, `case`, `form`, `paginate`, `tablerow`, `schema`, `javascript`, `style`, `stylesheet`, `raw`, `comment`, `doc`
- GitHub Actions CI: test, lint, WASM build on every push and PR
- GitHub Actions release workflow: builds WASM and creates GitHub release on version tags
- 11 new snippets (total 40):
  - `tablerow` — `{% tablerow %}…{% endtablerow %}` block
  - `include` — `{% include %}` (deprecated; for legacy themes)
  - `cycle` — `{% cycle 'a', 'b', 'c' %}`
  - `unle` — unless / else block
  - `{{date` — output formatted with `date:` filter
  - `{{money` — output with `money` filter
  - `{{moneyc` — output with `money_with_currency` filter
  - `{{def` — output with `default:` fallback
  - `{{trunc` — output with `truncate:` filter
  - `assignw` — assign filtered array via `where:`
  - `assignm` — assign mapped array via `map:`
- 14 new tests: 4 unit tests for `merge_workspace_config` logic, 10 integration tests in `tests/lsp_test.rs` covering LSP constants, server strategies, and config defaults
- `CONTRIBUTING.md` — local setup, project structure, build and release instructions
- `CHANGELOG.md`

### Changed
- Extracted `merge_workspace_config` from `language_server_workspace_configuration` into a standalone testable function

## [0.1.0] - 2026-04-30

### Added
- Syntax highlighting via `tree-sitter-liquid` grammar
- Language server integration — Shopify Theme Language Server
  - Prefers Shopify CLI (`shopify theme language-server`) when available
  - Falls back to auto-installing `@shopify/theme-language-server-node` via npm
- Embedded language injections: HTML, CSS, JavaScript, JSON, YAML
- Prettier formatting support (`prettier_parser_name = "liquid-html"`)
- 29 snippets covering all core Liquid and Shopify theme constructs
- Bracket matching and auto-close
- Block comment toggling (`{% comment %} … {% endcomment %}`)
