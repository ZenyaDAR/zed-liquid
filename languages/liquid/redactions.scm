; ── Zed multiplayer redactions — hide sensitive content ──────────────────────
; Schema JSON is redacted to avoid leaking section settings to other users
(schema_statement (json_content) @redact)

; String values in assignments may contain tokens, keys, or passwords
(assignment_statement value: (string) @redact)
