# Draft desired-policy examples

These YAML files illustrate a possible serialization of the in-memory
canonical policy model. There is no implemented schema, parser, validator,
comparator, or enforcement. The files have no effect on PermBridge or on any
coding agent. Field names and semantics may change before the first
functional policy release.

`global.yaml` illustrates a global desired policy. `project.yaml` illustrates
an optional project policy that increases restrictions for selected
capabilities. The `targets` list is aspirational: the Codex adapter does not
read these YAML examples, and no Claude Code adapter exists. The examples do
not prove that project restrictions are merged correctly; that behavior is
future work.
