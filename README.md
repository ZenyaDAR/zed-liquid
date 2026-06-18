# Liquid for Zed

[![version](https://img.shields.io/github/v/tag/ZenyaDAR/zed-liquid?label=version)](https://github.com/ZenyaDAR/zed-liquid/releases)
[![ci](https://img.shields.io/github/actions/workflow/status/ZenyaDAR/zed-liquid/ci.yml?label=ci)](https://github.com/ZenyaDAR/zed-liquid/actions/workflows/ci.yml)

[Liquid](https://shopify.github.io/liquid/) template language support for the [Zed](https://zed.dev) editor, built for Shopify theme development.

## Features

- **Syntax highlighting** — Liquid tags, filters, variables, operators, literals, comments, and doc-tags
- **Embedded languages** — HTML, CSS, JavaScript, JSON, and YAML highlighted inside appropriate Liquid blocks
- **Language server** — Shopify Theme Language Server: completions, hover docs, go-to-definition, diagnostics
- **Snippets** — 120+ snippets for every common Liquid and Shopify theme pattern
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

## Hover Documentation

The Shopify Theme Language Server provides rich hover documentation for all Liquid objects. Hover over any Shopify object to see its type, description, and available properties:

- `product` → title, handle, description, variants, images, price_range, …
- `collection` → title, handle, products, all_products_count, …
- `cart` → items, total_price, item_count, …
- `customer` → name, email, orders, addresses, …
- `section` → id, settings, blocks, …
- `forloop` → index, index0, first, last, length, …
- `link` → title, url, active, links, …

> **Note:** Hover docs work best when you open the full theme folder (not a single file). The language server uses the project structure to resolve objects and their types.

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

Most block-tag snippets respond to multiple prefixes. For example, typing `if` or `{%if` both expand to a full `{% if %}…{% endif %}` block.

### Control flow

| Prefix | Expands to |
|--------|-----------|
| `if` / `{%if` / `{% if` | `{% if condition %}…{% endif %}` |
| `ife` | if / else |
| `ifee` | if / elsif / else |
| `ifne` | if array/collection not empty |
| `unless` / `{%unless` | unless block |
| `unle` | unless / else |
| `case` / `{%case` | case / when statement |

### Loops

| Prefix | Expands to |
|--------|-----------|
| `for` / `{%for` | for loop |
| `forl` | for loop with `limit` and `offset` |
| `forr` | for loop `reversed` |
| `forlr` | for loop with limit, offset, and reversed |
| `break` | `{% break %}` |
| `continue` | `{% continue %}` |
| `tablerow` | tablerow loop (HTML table rows) |
| `tablerowc` | tablerow with `cols:` |
| `cycle` | `{% cycle 'a', 'b', 'c' %}` |

### Variables

| Prefix | Expands to |
|--------|-----------|
| `assign` / `{%assign` | assign variable |
| `assignw` | assign filtered array via `where:` |
| `assignm` | assign mapped array via `map:` |
| `assignsort` | assign sorted array via `sort:` |
| `assignfirst` | assign first matching item via `where: … \| first` |
| `capture` / `{%capture` | capture block |
| `increment` | increment variable |
| `decrement` | decrement variable |

### Output & filters

| Prefix | Expands to |
|--------|-----------|
| `{{` | `{{ variable }}` |
| `{{f` | `{{ variable \| filter }}` |
| `t` | `{{ 'key' \| t }}` translation |
| `echo` | `{% echo variable %}` |
| `{{date` | `\| date: '%B %d, %Y'` |
| `{{money` | `\| money` |
| `{{moneyc` | `\| money_with_currency` |
| `{{def` | `\| default: 'fallback'` |
| `{{trunc` | `\| truncate: 100` |
| `{{esc` | `\| escape` |
| `{{strip` | `\| strip_html` |
| `{{handle` | `\| handle` |
| `{{size` | `\| size` |
| `{{up` | `\| upcase` |
| `{{down` | `\| downcase` |
| `{{rep` | `\| replace: 'from', 'to'` |
| `{{app` | `\| append: 'suffix'` |
| `{{pre` | `\| prepend: 'prefix'` |
| `{{split` | `\| split: ','` |
| `{{join` | `\| join: ', '` |
| `{{nl` | `\| newline_to_br` |
| `{{url` | `\| url_encode` |
| `{{imgurl` | `\| image_url: width: 800` |
| `{{weight` | `\| weight_with_unit` |
| `{{times` / `{{minus` / `{{plus` | math filters |
| `{{round` / `{{floor` / `{{ceil` | rounding filters |
| `{{slice` | `\| slice: 0, 5` |
| `{{compact` / `{{uniq` / `{{reverse` | array filters |
| `{{first` / `{{last` | first / last array item |

### Shopify theme tags

| Prefix | Expands to |
|--------|-----------|
| `render` / `{%render` | render snippet |
| `renderw` | render snippet with parameter |
| `renderfor` | `{% render 'snippet' for array as item %}` |
| `include` | include snippet (deprecated — prefer `render`) |
| `section` / `{%section` | section tag |
| `sections` | sections tag |
| `paginate` / `{%paginate` | paginate block |
| `layout` | layout tag |
| `asset` | `{{ 'file.css' \| asset_url }}` |
| `assetlink` | asset as `<link>` stylesheet tag |
| `imgtag` | responsive image via `image_url` + `image_tag` |
| `img-src` | responsive image with lazy loading |
| `scripttag` | asset as `<script>` tag |
| `cfh` | `{{ content_for_header }}` |
| `cfl` | `{{ content_for_layout }}` |
| `cfi` | `{{ content_for_index }}` |

### Forms

| Prefix | Expands to |
|--------|-----------|
| `form` / `{%form` | generic form tag |
| `form-contact` | `{% form 'contact' %}` |
| `form-cart` | `{% form 'cart', cart %}` |
| `form-login` | `{% form 'customer_login' %}` |
| `form-register` | `{% form 'create_customer' %}` |
| `form-reset` | `{% form 'reset_customer_password' %}` |
| `form-address` | `{% form 'customer_address', customer.new_address %}` |
| `form-activate` | `{% form 'activate_customer_password' %}` |

### Embedded blocks

| Prefix | Expands to |
|--------|-----------|
| `javascript` / `{%javascript` | `{% javascript %}…{% endjavascript %}` |
| `stylesheet` / `{%stylesheet` | `{% stylesheet %}…{% endstylesheet %}` |
| `style` / `{%style` | `{% style %}…{% endstyle %}` |
| `raw` / `{%raw` | `{% raw %}…{% endraw %}` |
| `comment` / `{%comment` | `{% comment %}…{% endcomment %}` |
| `liquid` / `{%liquid` | `{% liquid … %}` multi-statement block |
| `liquidb` | `{% liquid … %}` with two assign stubs |

### Schema

| Prefix | Expands to |
|--------|-----------|
| `schema` / `{%schema` | schema block with settings/presets |
| `schema-full` | schema with settings, blocks, and presets |
| `schema-block` | block definition inside `"blocks": []` |
| `section-template` | full section boilerplate (HTML + schema + stylesheet + javascript) |

### Schema settings

Use inside `"settings": []` in any schema block.

| Prefix | Setting type |
|--------|-------------|
| `s-text` | `text` |
| `s-textarea` | `textarea` |
| `s-richtext` | `richtext` |
| `s-inlinert` | `inline_richtext` |
| `s-number` | `number` |
| `s-check` | `checkbox` |
| `s-range` | `range` (min / max / step / default) |
| `s-select` | `select` with two option stubs |
| `s-radio` | `radio` with two option stubs |
| `s-color` | `color` |
| `s-colorbg` | `color_background` (gradient) |
| `s-img` | `image_picker` |
| `s-video` | `video` |
| `s-videourl` | `video_url` (YouTube / Vimeo) |
| `s-url` | `url` |
| `s-html` | `html` |
| `s-font` | `font_picker` |
| `s-linklist` | `link_list` |
| `s-product` | `product` |
| `s-coll` | `collection` |
| `s-blog` | `blog` |
| `s-page` | `page` |
| `s-header` | `header` (section divider, no value) |
| `s-para` | `paragraph` (info text, no value) |

### Shopify patterns

| Prefix | Expands to |
|--------|-----------|
| `prod-loop` | paginated products loop with `default_pagination` |
| `nav-loop` | navigation menu links loop |
| `section-block` | `for block in section.blocks` with `case`/`when` |
| `meta-key` | `product.metafields.namespace.key \| metafield_tag` |

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

## Known Limitations

### CSS highlighting inside `{% style %}` blocks

Liquid expressions embedded **within** CSS values or selectors
(`--color: {{ variable }};`, `#id-{{ section.id }} {}`) create
gaps in the combined CSS injection stream. The surrounding static
CSS tokens may show reduced or error highlighting near those points.

The Liquid tags themselves (`{{ }}`, `{% %}`) are always highlighted
correctly — only the adjacent CSS text is affected.

**Workaround:** when possible, keep Liquid expressions on their own
lines and surround them with newlines. CSS highlighting is more stable
when Liquid tags do not split CSS property values mid-token.

## License

[MIT](./LICENSE)
