# Development

Install stable Rust with `rustfmt` and `clippy`. The workspace has no external
Rust dependencies. Node.js is unnecessary because the VS Code extension is a
placeholder.

From the repository root, run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --all-targets
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

`cargo run -p permbridge-cli` is only a startup smoke test. It prints a notice
and does not read policy files or inspect agents.

Core modules are organized by responsibility: `decision`, `policy`, `adapter`,
and `comparison`. The public exports in `lib.rs` are an experimental domain
surface, not a stable integration API. Keep Core independent of provider SDKs,
UI strings, and file formats. Changes to the model should update Rustdoc,
focused tests, and the relevant technical document together.

The [contribution guide](../CONTRIBUTING.md) defines language and review
expectations. [Testing](testing.md) explains what the current checks prove.

Development and local evaluation are covered by the current BSL grant;
production use of a BSL-covered version before its Change Date requires
separate terms. See [licensing](licensing.md). This repository does not accept
external pull requests or patches.
