# Liquid for Zed

[Liquid](https://shopify.github.io/liquid/) template language support for the [Zed](https://zed.dev) editor, built for Shopify theme development.

## Features

- **Syntax highlighting** — Liquid tags, filters, variables, operators, literals, comments, and doc-tags
- **Embedded languages** — HTML, CSS, JavaScript, JSON, and YAML highlighted inside appropriate Liquid blocks
- **Language server** — Shopify Theme Language Server: completions, hover docs, go-to-definition, diagnostics
- **Snippets** — 29 snippets for every common Liquid pattern
- **Bracket matching** — autoclosing brackets and quotes with Liquid-aware context

## Requirements

### Option A — Shopify CLI (recommended, zero install overhead)

The extension first looks for `shopify` on your `PATH`. If found, it starts the language server with:

```sh
shopify theme language-server
```

Install the CLI if you don't have it:

```sh
npm install -g @shopify/cli
```

### Option B — automatic npm fallback

If Shopify CLI is not installed, the extension automatically downloads and installs `@shopify/theme-language-server-node` via npm on first use. No manual steps required.

## Installation

Open Zed, run **zed: extensions**, search for **Liquid**, and click **Install**.

## Language Server Settings

Configure via Zed's `settings.json`:

```json
{
  "lsp": {
    "shopify-theme-ls": {
      "settings": {
        "shopifyLiquid": {
          "onlySingleFileChecks": false,
          "disabledChecks": []
        }
      }
    }
  }
}
```

## Snippets

| Prefix | Expands to |
|--------|-----------|
| `if` | `{% if condition %}…{% endif %}` |
| `ife` | if / else |
| `ifee` | if / elsif / else |
| `unless` | unless block |
| `for` | for loop |
| `forl` | for loop with `limit` and `offset` |
| `assign` | assign variable |
| `capture` | capture block |
| `case` | case / when statement |
| `render` | render snippet |
| `renderw` | render snippet with parameter |
| `section` | section tag |
| `sections` | sections tag |
| `paginate` | paginate block |
| `comment` | comment block |
| `raw` | raw block |
| `schema` | schema block |
| `javascript` | javascript block |
| `stylesheet` | stylesheet block |
| `style` | style block |
| `echo` | echo tag |
| `liquid` | liquid tag (multi-statement) |
| `increment` | increment variable |
| `decrement` | decrement variable |
| `layout` | layout tag |
| `form` | form tag |
| `{{` | output variable |
| `{{f` | output variable with filter |
| `t` | translation filter |

## Grammar

Uses [`hankthetank27/tree-sitter-liquid`](https://github.com/hankthetank27/tree-sitter-liquid) — chosen over Shopify's own grammar because it correctly parses all block tags (`if`, `for`, `render`, `section`, `schema`, `javascript`, etc.), not just expressions.

## Embedded Language Support

| Liquid block | Highlighted as |
|---|---|
| Template content (outside tags) | HTML |
| `{% schema %}` | JSON |
| `{% style %}` | CSS |
| `{% stylesheet %}` | CSS |
| `{% javascript %}` | JavaScript |
| Front matter | YAML |

## License

[MIT](./LICENSE)
