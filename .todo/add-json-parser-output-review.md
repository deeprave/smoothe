## Review: add-json-parser-output

### Scope

Reviewed the working-tree changes for `add-json-parser-output` against `HEAD`,
including:

- `src/cli.rs`
- `src/commands/parse.rs`
- `src/parser/diagnostic.rs`
- `tests/cli/mod.rs`
- `.gitignore`
- `openspec/changes/add-json-parser-output/`

`guide://code-review` was unavailable with error: `Category 'code-review' not
found in project`.

### Findings

No blocking issues found.

### Notes

- JSON parse output is gated behind `parse --json` / `parse -j`.
- Default compact tree output remains unchanged.
- JSON diagnostics are grouped into `errors` and `warnings`.
- `IssueKind` JSON output uses an explicit `as_str` mapping rather than `Debug`
  formatting.

### Verification

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo nextest run`
