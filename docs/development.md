# Development

Install stable Rust with `rustfmt` and `clippy`, as specified in
`rust-toolchain.toml`. Core uses `serde_yaml_ng` for YAML syntax and `tempfile`
in loader tests. The Codex adapter uses `toml` and its tests use
`tempfile`. Node.js is unnecessary because the VS Code extension is a
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

Run all six validation commands locally before committing or pushing. The
[GitHub Actions workflow](../.github/workflows/ci.yml) repeats them in a clean
Ubuntu environment on pushes to `main` and pull requests targeting `main`.
It uses the repository's stable Rust toolchain and requires no secrets.

`cargo run -p permbridge-cli` is only a startup smoke test. It prints a notice
and does not read policy files or inspect agents.

Core modules are organized by responsibility: `decision`, `policy`, `loading`,
`adapter`, and `comparison`. The public exports in `lib.rs` are an experimental
domain surface, not a stable integration API. The provider-agnostic `loading`
module owns YAML input; the canonical policy types remain independent of YAML
representation. Keep Core independent of provider SDKs and UI strings.
Changes to the model should update Rustdoc, focused tests, and the relevant
technical document together.
The provider-agnostic `compare` function consumes an in-memory policy and
posture. It performs no file access. See [posture comparison](comparison.md)
for its evidence threshold and handling of incomplete observations.

Run `cargo test -p permbridge-core --test policy_loading` for the focused
version-1 loader suite. See the [policy format](policy-format.md) for the
accepted structure and limits.

`permbridge-codex` is a separate crate with a read-only file loader and a
conservative mapping step. Its caller must provide the Codex home, optional
workspace root, and project trust explicitly. The crate's integration tests
use synthetic temporary roots; they never inspect the developer's real Codex
files. `toml` parses configuration syntax, and the test-only `tempfile` crate
keeps isolated fixtures easy to clean up. Run `cargo test -p permbridge-codex`
for its focused suite. See [Codex compatibility](compatibility/codex.md).

`permbridge-claude-code` also takes injected configuration roots and trust.
It uses `serde_json` to parse selected Claude Code JSON settings and reuses
`tempfile` for isolated tests. It does not inspect real user settings during
tests or run Claude Code. Run `cargo test -p permbridge-claude-code` for its
focused suite. See [Claude Code compatibility](compatibility/claude-code.md).

The [contribution guide](../CONTRIBUTING.md) defines language and review
expectations. [Testing](testing.md) explains what the current checks prove.

Development and local evaluation are covered by the current BSL grant;
production use of a BSL-covered version before its Change Date requires
separate terms. See [licensing](licensing.md). This repository does not accept
external pull requests or patches.
