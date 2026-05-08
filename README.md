# Liquid for Zed

[![version](https://img.shields.io/github/v/tag/ZenyaDAR/zed-liquid?label=version)](https://github.com/ZenyaDAR/zed-liquid/releases)
[![ci](https://img.shields.io/github/actions/workflow/status/ZenyaDAR/zed-liquid/ci.yml?label=ci)](https://github.com/ZenyaDAR/zed-liquid/actions/workflows/ci.yml)

[Liquid](https://shopify.github.io/liquid/) template language support for the [Zed](https://zed.dev) editor, built for Shopify theme development.

## Features

- **Syntax highlighting** — Liquid tags, filters, variables, operators, literals, comments, and doc-tags
- **Embedded languages** — HTML, CSS, JavaScript, JSON, and YAML highlighted inside appropriate Liquid blocks
- **Language server** — Shopify Theme Language Server: completions, hover docs, go-to-definition, diagnostics
- **Snippets** — 55+ snippets for every common Liquid and Shopify theme pattern
- **Document outline** — navigate to `render`/`section`/`schema`/`for`/`capture` landmarks in the outline panel
- **Auto-indentation** — block tags (`if`, `for`, `unless`, `case`, `capture`, etc.) auto-indent their body
- **Code folding** — fold any Liquid block tag (`if`, `for`, `schema`, `javascript`, `comment`, and more)
- **Bracket matching** — autoclosing brackets and quotes with Liquid-aware context
- **Formatting** — Prettier with `liquid-html` parser (requires `@shopify/prettier-plugin-liquid` in your project)

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

### Control flow

| Prefix | Expands to |
|--------|-----------|
| `if` | `{% if condition %}…{% endif %}` |
| `ife` | if / else |
| `ifee` | if / elsif / else |
| `unless` | unless block |
| `unle` | unless / else |
| `for` | for loop |
| `forl` | for loop with `limit` and `offset` |
| `tablerow` | tablerow loop (HTML table rows) |
| `cycle` | `{% cycle 'a', 'b', 'c' %}` |
| `case` | case / when statement |

### Variables

| Prefix | Expands to |
|--------|-----------|
| `assign` | assign variable |
| `assignw` | assign filtered array via `where:` |
| `assignm` | assign mapped array via `map:` |
| `assignsort` | assign sorted array via `sort:` |
| `assignfirst` | assign first matching item via `where: … \| first` |
| `capture` | capture block |
| `increment` | increment variable |
| `decrement` | decrement variable |

### Output & filters

| Prefix | Expands to |
|--------|-----------|
| `{{` | `{{ variable }}` |
| `{{f` | `{{ variable \| filter }}` |
| `{{date` | `{{ variable \| date: '%B %d, %Y' }}` |
| `{{money` | `{{ variable \| money }}` |
| `{{moneyc` | `{{ variable \| money_with_currency }}` |
| `{{def` | `{{ variable \| default: 'fallback' }}` |
| `{{trunc` | `{{ variable \| truncate: 100 }}` |
| `{{esc` | `{{ variable \| escape }}` |
| `{{strip` | `{{ variable \| strip_html }}` |
| `{{handle` | `{{ variable \| handle }}` |
| `{{size` | `{{ variable \| size }}` |
| `t` | `{{ 'key' \| t }}` translation |
| `echo` | `{% echo variable %}` |

### Shopify theme

| Prefix | Expands to |
|--------|-----------|
| `render` | render snippet |
| `renderw` | render snippet with parameter |
| `include` | include snippet (deprecated — prefer `render`) |
| `section` | section tag |
| `sections` | sections tag |
| `paginate` | paginate block |
| `form` | form tag |
| `layout` | layout tag |
| `schema` | schema block with settings/presets |
| `section-template` | full section boilerplate (HTML + schema + stylesheet + javascript) |
| `asset` | `{{ 'file.css' \| asset_url }}` |
| `assetlink` | asset as `<link>` stylesheet tag |
| `imgtag` | responsive image via `image_url` + `image_tag` |
| `scripttag` | asset as `<script>` tag |
| `cfh` | `{{ content_for_header }}` |
| `cfl` | `{{ content_for_layout }}` |
| `cfi` | `{{ content_for_index }}` |

### Embedded blocks

| Prefix | Expands to |
|--------|-----------|
| `javascript` | `{% javascript %}…{% endjavascript %}` |
| `stylesheet` | `{% stylesheet %}…{% endstylesheet %}` |
| `style` | `{% style %}…{% endstyle %}` |
| `raw` | `{% raw %}…{% endraw %}` |
| `comment` | `{% comment %}…{% endcomment %}` |
| `liquid` | `{% liquid … %}` multi-statement block |

## Schema JSON validation

The Shopify Theme Language Server provides JSON completions and validation inside `{% schema %}` blocks automatically — no extra configuration needed.

For additional JSON Schema validation (e.g. in a standalone editor or CI), point your JSON language server at Shopify's published schema:

```json
{
  "json.schemas": [
    {
      "fileMatch": ["*.json"],
      "url": "https://json.schemastore.org/shopify-section.json"
    }
  ]
}
```

The LS also validates section files in `config/settings_schema.json` and `config/settings_data.json` automatically when you open a theme folder (not a single file).

## Tailwind CSS

To use Tailwind CSS in a Shopify theme with this extension:

1. Install the [Zed Tailwind CSS extension](https://github.com/zed-industries/zed/tree/main/extensions/tailwindcss) via **zed: extensions**.

2. Configure Tailwind to scan `.liquid` files in `tailwind.config.js`:

   ```js
   module.exports = {
     content: [
       './**/*.liquid',
       './assets/*.js',
     ],
   }
   ```

3. Tailwind completions and hover docs will now work in `.liquid` files alongside Liquid completions from the Shopify LS.

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
