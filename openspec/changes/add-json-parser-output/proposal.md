## Why

The `parse` command currently prints a compact tree intended for human
inspection, which is awkward for tooling and automated checks. A JSON output
mode gives callers a stable machine-readable representation of the parse result
while preserving the existing compact output by default.

## What Changes

- Add a `--json` option to the `parse` command.
- Add `-j` as the short alias for `--json`.
- When JSON output is requested, print the parse result as valid JSON instead
  of the compact tree format.
- Include parser diagnostics in JSON output, grouped into `errors` and
  `warnings` lists when present.
- Keep the existing compact tree output as the default when `--json` is absent.
- Preserve existing exit-status behavior for the `parse` command.

## Capabilities

### New Capabilities

- `json-parser-output`: JSON parse-result output mode for the `parse` command.

### Modified Capabilities

None.

## Impact

- Affected code: CLI argument model, parse command output formatting, AST and
  diagnostic serialization/projection, and CLI tests.
- APIs: may require serializable parser AST output types or a dedicated JSON
  projection for parse command output.
- Dependencies: no new runtime dependency expected beyond existing JSON support.
