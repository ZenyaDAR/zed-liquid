# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-05-08

### Added
- **Syntax highlighting — major expansion**
  - Object property access (`product.title`, `cart.item_count`) highlighted as `@property`
  - Shopify global objects (`product`, `cart`, `shop`, `customer`, `collection`, etc.) highlighted as `@type`
  - Special Liquid constants (`nil`, `null`, `blank`, `empty`) highlighted as `@constant.builtin`
  - `layout none` sentinel highlighted as `@constant.builtin`
  - File references in `render`/`include`/`section`/`sections` highlighted as `@string.special`
  - Built-in loop objects (`forloop`, `tablerowloop`, `paginate`) highlighted as `@variable.builtin`
  - Variable declaration positions in `assign`/`for`/`capture`/`tablerow`/`increment`/`decrement` highlighted as `@variable.special`
  - Named argument keys (`param: value`) highlighted as `@property`
  - Range operator `..` and surrounding parentheses highlighted as `@operator` / `@punctuation.bracket`
- **Syntax highlighting — bug fixes**
  - Fixed `(predicate)` node incorrectly captured wholesale as `@operator`; now only the `operator:` field is captured, preventing stray coloring on complex conditions
  - Raised `and` / `or` / `contains` / `in` to priority 103 so they always win over the predicate operator rule
- **Document outline** — navigate to render / section / sections / schema / javascript / for / capture landmarks
- **Outline expanded** — form, paginate, and if blocks now appear in the outline panel
- **Auto-indentation for `{% form %}`** — `form_statement` body now indents on Enter (grammar edge case: no `block` wrapper)
- **Multiplayer redactions** (`redactions.scm`) — `{% schema %}` JSON content and string assignment values are hidden from Zed collaborators
- **15 new snippets** (40 → 55+)
  - Shopify globals & assets: `asset`, `assetlink`, `imgtag`, `cfh`, `cfl`, `cfi`, `scripttag`
  - Common filters: `{{esc`, `{{strip`, `{{handle`, `{{size`
  - Array helpers: `assignsort`, `assignfirst`
  - Full section boilerplate: `section-template`
- **LSP resilience** — added pinned fallback version (`3.0.0`) when `npm_package_latest_version` fails, so offline installs do not break
- **README** — added version and CI badges; expanded snippet tables; added Schema JSON validation and Tailwind CSS setup sections

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
