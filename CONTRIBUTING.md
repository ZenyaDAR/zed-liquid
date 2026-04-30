# Contributing

## Requirements

- [Rust](https://rustup.rs/) stable toolchain
- [Zed](https://zed.dev/) editor (to test the extension locally)
- Node.js 18+ (to run the Shopify Language Server locally)

## Local development

Clone the repo and install the extension into Zed directly from the local path:

1. Open Zed → **Extensions** → **Install Dev Extension**
2. Select the root of this repository

Zed will compile the Rust crate to WASM and load the extension. After any change to `src/lib.rs`, restart the extension from the Extensions panel.

## Project structure

```
src/lib.rs                     — Extension entry point (Rust → WASM)
languages/liquid/
  config.toml                  — Language metadata, brackets, Prettier parser
  highlights.scm               — Syntax highlighting queries
  injections.scm               — Embedded language injections (HTML, CSS, JS, JSON, YAML)
  folds.scm                    — Code folding queries
snippets/liquid.json           — Editor snippets
extension.toml                 — Extension manifest (id, grammar, language server)
```

## Making changes

### Syntax highlighting

Edit `languages/liquid/highlights.scm`. Capture names follow the [Zed highlight reference](https://zed.dev/docs/extensions/languages#syntax-highlighting).

### Snippets

Edit `snippets/liquid.json`. Each entry must have `prefix`, `body` (array of strings), and `description`. Run `cargo test` to verify structure.

### Tree-sitter grammar

The grammar is pinned by commit hash in `extension.toml`. To update it:

1. Find the new commit SHA on [hankthetank27/tree-sitter-liquid](https://github.com/hankthetank27/tree-sitter-liquid)
2. Update the `commit` field in `extension.toml`
3. Delete `grammars/liquid/` and let Zed re-fetch on next load

### Language server options

Workspace configuration defaults live in `language_server_workspace_configuration()` in `src/lib.rs`.

## Running tests

```sh
cargo test
```

## Checking code style

```sh
cargo fmt --check
cargo clippy -- -D warnings
```

## Building the WASM artifact

```sh
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
```

## Releasing

1. Update `version` in `extension.toml` and `Cargo.toml`
2. Add an entry to `CHANGELOG.md`
3. Commit, tag, and push:
   ```sh
   git tag v0.x.0
   git push origin v0.x.0
   ```
4. The release workflow builds the WASM and creates a GitHub release automatically.
5. To publish to the Zed extension registry, submit a PR to [zed-industries/extensions](https://github.com/zed-industries/extensions).
