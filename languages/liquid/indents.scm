; ── Liquid block bodies ───────────────────────────────────────────────────────
; All control-flow and variable-capture blocks share the same `block` AST node
; (if/unless/for/case/when/capture/tablerow/paginate consequence and bodies).
; Marking it once covers every paired tag automatically.
(block) @indent

; form_statement has no `block` child in the grammar — its body is flat children.
; Declare the whole node as an indent container so Enter inside {% form %} indents.
(form_statement) @indent
