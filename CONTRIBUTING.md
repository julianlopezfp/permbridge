# Contributing to AgentGuard

Thank you for helping build AgentGuard. This repository is an early skeleton;
please keep changes focused and avoid describing planned security features as
working protection.

## Language and documentation

Write source code, identifiers, comments, commit messages, issues, and
canonical technical documentation in English. Keep the main user-facing
README available in both [English](README.md) and [Spanish](README.es.md).
When changing user-facing instructions, update both versions. Keep internal
design documentation in English; future user guides can be paired under
`docs/user/en/` and `docs/user/es/`.

## Development setup

Install stable Rust with `rustfmt` and `clippy`. The VS Code placeholder does
not require Node.js. Before opening a pull request, run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

Add focused tests for behavior you implement. If you change policy semantics,
document the security impact and ensure that a project policy cannot weaken a
global restriction. Do not commit secrets, private prompts, or machine-specific
paths. Use a clear conventional commit message, such as `fix: correct decision
ordering`.
