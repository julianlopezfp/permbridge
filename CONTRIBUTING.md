# Issue and development policy

PermBridge is authored and maintained by Julián López Jiménez. External pull
requests, patches, and other code contributions are not accepted. Please do not
open a pull request or submit code through an Issue. GitHub Issues are welcome
for bug reports, documentation problems, and suggestions. Security reports
follow the private process in [SECURITY.md](SECURITY.md).

The current repository is source-available under [BSL 1.1](LICENSE); see
[licensing details](docs/licensing.md) before using or redistributing it.

## Language and documentation

Repository content and Issues should be in English. The canonical technical
documentation and user-facing [README](README.md) are also in English.

## Development setup

For maintainer work, install stable Rust with `rustfmt` and `clippy`. The VS
Code placeholder does not require Node.js. Before publishing a change, run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --all-targets
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo build --workspace --all-targets
cargo test --workspace --all-targets
```

Maintainer changes to policy merging must show that a project policy cannot
weaken a global restriction. Adapter changes must document native capability
limits and provide behavior evidence. The maintainer should add focused tests,
avoid secrets and machine-specific paths, and use a clear conventional commit
message.
