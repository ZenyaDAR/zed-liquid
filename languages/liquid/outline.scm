; ── render 'snippet-name' — show the included snippet file ───────────────────
(render_statement
  file: (string) @name) @item

; ── section 'name' / sections 'name' — layout includes ───────────────────────
(section_statement
  (string) @name) @item

(sections_statement
  (string) @name) @item

; ── {% schema %} block — landmark for JSON settings editing ──────────────────
(schema_statement
  "schema" @name) @item

; ── {% javascript %} block — inline client-side code ─────────────────────────
(javascript_statement
  "javascript" @name) @item

; ── {% for item in collection %} — show the loop variable ────────────────────
(for_loop_statement
  item: (identifier) @name) @item

; ── {% capture variable %} — show the variable being built ───────────────────
(capture_statement
  variable: (identifier) @name) @item

; ── {% form 'type' %} — show form type in outline ────────────────────────────
(form_statement
  type: (string) @name) @item

; ── {% paginate collection by N %} — show paginated collection ───────────────
(paginate_statement
  item: (identifier) @name) @item

; ── {% if condition %} — show condition in outline ───────────────────────────
(if_statement
  condition: (_) @name) @item
