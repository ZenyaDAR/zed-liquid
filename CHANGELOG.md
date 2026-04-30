# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-30 — Initial Release

### Added
- Syntax highlighting via [`hankthetank27/tree-sitter-liquid`](https://github.com/hankthetank27/tree-sitter-liquid) grammar
- Language server integration — Shopify Theme Language Server
  - Prefers Shopify CLI (`shopify theme language-server`) when available
  - Falls back to auto-installing `@shopify/theme-language-server-node` via npm
- Embedded language injections: HTML, CSS, JavaScript, JSON, YAML
- Prettier formatting support (`prettier_parser_name = "liquid-html"`)
- Code folding for all Liquid block tags: `if`, `for`, `unless`, `capture`, `case`, `form`, `paginate`, `tablerow`, `schema`, `javascript`, `style`, `stylesheet`, `raw`, `comment`, `doc`
- 40 snippets covering all core Liquid and Shopify theme constructs
- Bracket matching and auto-close
- Block comment toggling (`{% comment %} … {% endcomment %}`)
- GitHub Actions CI: test, lint, WASM build on every push and PR
- GitHub Actions release workflow: builds WASM and creates GitHub Release on version tags
