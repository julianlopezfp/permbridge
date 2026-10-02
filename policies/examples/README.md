# Version-1 policy examples

These files are valid inputs to `permbridge_core::load_policy_file`.
`minimal.yaml` shows all omitted decisions defaulting to `ASK`. `global.yaml`
exercises current dimensions and exact, opaque selectors. `project.yaml` is a
stricter standalone project policy; no global/project merge is implemented.

Loading a file only validates and represents desired policy. It does not
configure or enforce any agent. See the [policy format](../../docs/policy-format.md).
