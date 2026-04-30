; HTML lives outside of {%…%} and {{…}} — the bulk of a .liquid file
((template_content) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined))

; {% schema %}…{% endschema %} → JSON
((json_content) @injection.content
  (#set! injection.language "json")
  (#set! injection.combined))

; {% style %}…{% endstyle %} → CSS
((style_content) @injection.content
  (#set! injection.language "css")
  (#set! injection.combined))

; {% stylesheet %}…{% endstylesheet %} → CSS
((stylesheet_content) @injection.content
  (#set! injection.language "css")
  (#set! injection.combined))

; {% javascript %}…{% endjavascript %} → JS
((js_content) @injection.content
  (#set! injection.language "javascript")
  (#set! injection.combined))

; YAML front matter (some themes use it)
((front_matter) @injection.content
  (#set! injection.language "yaml"))
