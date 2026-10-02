# Testing and validation

## Current tests

Core unit tests check that an unspecified decision is `Ask`, that combining
comparable decisions cannot weaken a global restriction, that a new policy
starts with `Ask` across dimensions, that unsupported and ambiguous comparison
outcomes do not count as compliance, and that a test adapter preserves
enforcement and mapping limits while keeping inspection failures distinct.
The test adapter is not a product integration.

The crate does not serialize or deserialize policies, so there are no
serialization tests. `policies/examples/` contains illustrative YAML only.
The Codex crate tests synthetic user and project TOML fixtures, project-over-
user precedence, skipped untrusted projects, missing files, malformed and
invalid settings, unknown values, permission-profile ambiguity, unsupported
selectors, source provenance, and unchanged input files. They do not launch
Codex or validate actual OS sandbox behavior. No Claude Code adapter exists.

## Validation commands

Run formatting, strict Clippy, type checking, Rustdoc, build, and tests locally
before committing or pushing:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --all-targets
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

The [GitHub Actions workflow](../.github/workflows/ci.yml) runs the same six
commands in a clean Ubuntu environment for pushes to `main` and pull requests
targeting `main`. It provides independent verification of these implemented
checks. A successful run does not prove Codex runtime behavior, policy
equivalence, or security enforcement.

`cargo check` is also useful locally without linking. It does not replace a
build or an executed test run.

## Remaining test gaps

Before policy files become executable, add schema and validation tests,
including malformed input and precedence across scopes. Before claiming an
adapter enforces a native boundary, test the agent's behavior on supported
platforms and versions. Comparator tests must preserve `NotConfigured`,
`Unsupported`, `Ambiguous`, and missing observations rather than treating them
as equivalent. The current Codex fixture tests establish parser and mapping
behavior only; see [compatibility](compatibility/codex.md).

Licensing metadata and documentation consistency are reviewed by the
maintainer when the repository license changes. Rust tests validate the code;
they do not determine legal rights or prove production readiness. See
[licensing](licensing.md).
