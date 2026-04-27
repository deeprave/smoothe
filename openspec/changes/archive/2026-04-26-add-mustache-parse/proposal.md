## Why

Developers need a small, direct way to exercise the parser from the production
CLI while parser behavior is still evolving. A `parse` command makes it easy to
pipe template content into `smoothe`, inspect diagnostics, and view the parsed
AST without wiring through the future checker workflow.

## What Changes

- Add a production `parse` CLI command that reads template content from stdin.
- Parse stdin content using the existing parser API.
- Print parser diagnostics, including warnings, when present.
- Print the parsed AST in a developer-readable debug format.
- Return a non-zero exit status when error diagnostics are present.
- Keep this command focused on parser inspection rather than template checking
  or rendering.

## Capabilities

### New Capabilities

- `mustache-parse-command`: CLI support for parsing stdin, reporting parser
  diagnostics, and printing the AST.

### Modified Capabilities

None.

## Impact

- Affected code: CLI argument model, command dispatch, parse command
  implementation, and CLI tests.
- APIs: uses the existing parser API; no parser API changes are expected.
- Dependencies: no new dependencies expected.
