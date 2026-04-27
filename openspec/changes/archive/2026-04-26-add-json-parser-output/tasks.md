## 1. CLI Argument Wiring

- [x] 1.1 Add a `json` flag to `ParseArgs` using `--json` and `-j`.
- [x] 1.2 Route parse command output through a selectable compact-tree or JSON
  output mode.

## 2. JSON Output Projection

- [x] 2.1 Add command-level serializable result types for JSON parse output.
- [x] 2.2 Project each parsed input into a JSON input result containing input
  name, AST nodes, `errors`, and `warnings`.
- [x] 2.3 Project scalar AST nodes with explicit `kind`, span, and node-specific
  fields.
- [x] 2.4 Project container AST nodes with explicit `kind`, span,
  node-specific name fields, and `children`.
- [x] 2.5 Represent empty ASTs as empty node lists.

## 3. JSON Diagnostics Projection

- [x] 3.1 Project error diagnostics into each input result's `errors` list.
- [x] 3.2 Project warning diagnostics into each input result's `warnings` list.
- [x] 3.3 Include issue kind, source name, line, column, span, and message in
  JSON diagnostic objects.
- [x] 3.4 Emit empty `errors` and `warnings` lists when no matching diagnostics
  exist.

## 4. Output Behavior

- [x] 4.1 Emit one valid JSON document with a top-level `inputs` list in JSON
  mode.
- [x] 4.2 Keep the existing compact tree output unchanged when JSON mode is not
  selected.
- [x] 4.3 Preserve current `--out` behavior while writing JSON content when JSON
  mode is selected.
- [x] 4.4 Preserve current parse command exit-status behavior for JSON mode.

## 5. Verification

- [x] 5.1 Add CLI tests under `tests/` for `parse --json` output being valid
  JSON.
- [x] 5.2 Add CLI tests under `tests/` for `parse -j` selecting JSON output.
- [x] 5.3 Add CLI tests under `tests/` proving default `parse` output remains
  compact tree output.
- [x] 5.4 Add tests under `tests/` for representative scalar and container AST
  node JSON fields.
- [x] 5.5 Add tests under `tests/` for grouped `errors` and `warnings`
  diagnostic output.
- [x] 5.6 Add tests under `tests/` for JSON mode exit status with errors and
  warnings.
- [x] 5.7 Run `cargo fmt --check`.
- [x] 5.8 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 5.9 Run `cargo nextest run`.
