# Testing and validation

## Current tests

Core tests check that an unspecified decision is `Ask`, that combining
comparable decisions cannot weaken a global restriction, that a new policy
starts with `Ask` across dimensions, and that a test adapter preserves
enforcement and mapping limits while keeping inspection failures distinct.
Comparator integration tests cover all ordered decisions, declaration-only
evidence, uncertain and missing observations, mixed and deterministically
ordered results, unrequested observations, and duplicate desired selectors.
The test adapter is not a product integration.

Core loader tests cover minimal and complete version-1 YAML, ASK defaults,
decisions, scopes, malformed input, version and field errors, opaque selector
validation, duplicate rules, privacy-safe errors, unchanged temporary files,
and all repository examples. One test passes a loaded policy into the existing
comparator with synthetic posture. Serialization back to YAML is not
implemented, so there are no round-trip tests.
The Codex crate tests synthetic user and project TOML fixtures, project-over-
user precedence, skipped untrusted projects, missing files, malformed and
invalid settings, unknown values, permission-profile ambiguity, unsupported
selectors, source provenance, and unchanged input files. They do not launch
Codex or validate actual OS sandbox behavior.

The Claude Code crate tests synthetic user, shared-project, and local JSON
fixtures; scalar precedence and merged rule provenance; missing and untrusted
sources; malformed JSON and invalid selected types; every recognized mode;
unknown modes; sandbox and hook uncertainty; unsupported selectors; sanitized
errors; and unchanged input files. Neither adapter launches an agent or tests
native enforcement.
Each adapter's synthetic fixture tests also feed its produced posture to the
Core comparator. Codex's declared allowance remains ambiguous as a final
outcome; Claude Code's broad uncertain observation stays ambiguous.

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
checks. A successful run does not prove agent runtime behavior, policy
equivalence, or security enforcement.

`cargo check` is also useful locally without linking. It does not replace a
build or an executed test run.

## Remaining test gaps

Before a multi-policy workflow exists, specify discovery and precedence across
scopes and test those rules. Before claiming an
adapter enforces a native boundary, test the agent's behavior on supported
platforms and versions. Comparator tests must preserve `NotConfigured`,
`Unsupported`, `Ambiguous`, and missing observations rather than treating them
as equivalent. The current adapter fixture tests establish static parser and
mapping behavior only; see [compatibility](compatibility/README.md).
The comparator tests do not validate native runtime mechanisms; the loader
integration test only supplies desired policy input. See [comparison](comparison.md).

Licensing metadata and documentation consistency are reviewed by the
maintainer when the repository license changes. Rust tests validate the code;
they do not determine legal rights or prove production readiness. See
[licensing](licensing.md).
