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
There are no Codex or Claude Code integration tests because no such adapter
exists.

## Validation commands

CI runs formatting, strict Clippy, type checking, Rustdoc, build, and tests on
Linux:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --all-targets
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

`cargo check` is also useful locally without linking. It does not replace a
build or an executed test run.

## Remaining test gaps

Before policy files become executable, add schema and validation tests,
including malformed input and precedence across scopes. Before claiming an
agent adapter is supported, test against representative native
configurations and the agent's actual behavior where observable. Comparator
tests must preserve `Unsupported`, `Ambiguous`, and missing observations rather
than treating them as equivalent.
