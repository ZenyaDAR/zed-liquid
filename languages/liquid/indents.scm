; ── Liquid block bodies ───────────────────────────────────────────────────────
; All control-flow and variable-capture blocks share the same `block` AST node
; (if/unless/for/case/when/capture/tablerow/paginate consequence and bodies).
; Marking it once covers every paired tag automatically.
(block) @indent
