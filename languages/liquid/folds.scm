; ── Control flow ──────────────────────────────────────────────────────────────
(if_statement)       @fold
(unless_statement)   @fold
(case_statement)     @fold
(for_loop_statement) @fold

; ── Variable / output ─────────────────────────────────────────────────────────
(capture_statement)  @fold

; ── Shopify block tags ────────────────────────────────────────────────────────
(form_statement)        @fold
(paginate_statement)    @fold
(tablerow_statement)    @fold

; ── Embedded language blocks ──────────────────────────────────────────────────
(schema_statement)      @fold
(javascript_statement)  @fold
(style_statement)       @fold
(stylesheet_statement)  @fold
(raw_statement)         @fold

; ── Comments ──────────────────────────────────────────────────────────────────
(comment) @fold
(doc)     @fold
