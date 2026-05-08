; ── Liquid tag delimiters ─────────────────────────────────────────────────────
([
  "{{"
  "}}"
  "{{-"
  "-}}"
  "{%"
  "%}"
  "{%-"
  "-%}"
  ] @tag.delimiter
 (#set! priority 101))

; ── Bracket-like punctuation ──────────────────────────────────────────────────
([
  "]"
  "["
  ")"
  "("
  ] @punctuation.bracket
 (#set! priority 101))

([
  ","
  "."
  ] @punctuation.delimiter
 (#set! priority 101))

; ── Keywords (all Liquid / Shopify tags) ──────────────────────────────────────
([
  "as"
  "assign"
  (break_statement)
  "by"
  "capture"
  "case"
  (continue_statement)
  (custom_unpaired_statement)
  "cycle"
  "decrement"
  "doc"
  "echo"
  "else"
  "elsif"
  "endcapture"
  "endcase"
  "enddoc"
  "endfor"
  "endform"
  "endif"
  "endjavascript"
  "endpaginate"
  "endraw"
  "endschema"
  "endstyle"
  "endstylesheet"
  "endtablerow"
  "endunless"
  "for"
  "form"
  "if"
  "include"
  "include_relative"
  "increment"
  "javascript"
  "layout"
  "liquid"
  "paginate"
  "raw"
  "render"
  "schema"
  "section"
  "sections"
  "style"
  "stylesheet"
  "tablerow"
  "unless"
  "when"
  "with"
  ] @keyword
 (#set! priority 101))

; ── Logical / comparison operators used as keywords ───────────────────────────
; priority 103 so they win over (predicate operator: _) at 102
([
  "and"
  "contains"
  "in"
  "or"
  ] @keyword.operator
 (#set! priority 103))

; ── Operators ─────────────────────────────────────────────────────────────────
([
  "|"
  ":"
  "="
  ] @operator
 (#set! priority 101))

; Capture only the operator field inside a predicate node (==, !=, <, >, etc.)
(predicate operator: _ @operator (#set! priority 102))

; ── Primitives ────────────────────────────────────────────────────────────────
((identifier) @variable              (#set! priority 101))
((string)     @string                (#set! priority 101))
((boolean)    @boolean               (#set! priority 101))
((number)     @number                (#set! priority 101))

; ── Filters: the name after | is a function call ─────────────────────────────
(filter
  name: (identifier) @function.call  (#set! priority 101))

; ── Object property access: product.title, cart.items ───────────────────────
(access
  "." @punctuation.delimiter
  (#set! priority 102))

(access
  property: (identifier) @property
  (#set! priority 102))

; ── Liquid special constants ──────────────────────────────────────────────────
((identifier) @constant.builtin
  (#match? @constant.builtin "^(nil|null|blank|empty)$")
  (#set! priority 103))

; ── layout none — disable layout sentinel ───────────────────────────────────
(layout_statement "none" @constant.builtin (#set! priority 102))

; ── File/snippet references — semantically paths, not data ──────────────────
(render_statement
  file: (string) @string.special
  (#set! priority 102))

(section_statement
  (string) @string.special
  (#set! priority 102))

(sections_statement
  (string) @string.special
  (#set! priority 102))

(include_statement
  (string) @string.special
  (#set! priority 102))

; ── Liquid built-in loop objects ─────────────────────────────────────────────
((identifier) @variable.builtin
  (#match? @variable.builtin "^(forloop|tablerowloop|paginate)$")
  (#set! priority 103))

; ── Shopify global template objects ──────────────────────────────────────────
((identifier) @type
  (#match? @type
   "^(product|variant|collection|collections|cart|checkout|customer|shop|request|theme|settings|block|section|all_products|articles|blogs|linklists|pages|routes|template|search|localization|handle|canonical_url|powered_by_link|scripts|content_for_additional_fonts)$")
  (#set! priority 102))

; ── Variable declarations (lvalue positions) ─────────────────────────────────
(assignment_statement
  variable_name: (identifier) @variable.special
  (#set! priority 103))

(for_loop_statement
  item: (identifier) @variable.special
  (#set! priority 103))

(tablerow_statement
  item: (identifier) @variable.special
  (#set! priority 103))

(capture_statement
  variable: (identifier) @variable.special
  (#set! priority 103))

(increment_statement (identifier) @variable.special (#set! priority 103))
(decrement_statement (identifier) @variable.special (#set! priority 103))

; ── Named argument keys (param: value) ───────────────────────────────────────
(argument
  key: (identifier) @property
  (#set! priority 102))

; ── Range operator ───────────────────────────────────────────────────────────
(range ".." @operator (#set! priority 102))
(range "(" @punctuation.bracket (#set! priority 102))
(range ")" @punctuation.bracket (#set! priority 102))

; ── Raw block content — shown as plain text ───────────────────────────────────
(raw_statement
  (raw_content) @text.reference      (#set! priority 102))

; ── Comments ──────────────────────────────────────────────────────────────────
((comment)            @comment                (#set! priority 102))

; ── Doc-comments (@param / @example annotations) ────────────────────────────
((doc)                       @comment.documentation (#set! priority 102))
((doc_content)               @comment.documentation (#set! priority 102))
((doc_description_annotation) @keyword              (#set! priority 103))
((doc_example_annotation)    @comment.documentation (#set! priority 102))
((doc_example_content)       @none                  (#set! priority 103))
((doc_param_name)            @variable              (#set! priority 103))
((doc_type)                  @type                  (#set! priority 103))
("@param"   @keyword (#set! priority 103))
("@example" @keyword (#set! priority 103))
