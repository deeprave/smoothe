## Why

The CLI needs to operate on real template files rather than only stdin, while
still supporting shell pipelines through the conventional `-` stdin marker.
The command help should also describe `check` and `parse` consistently now that
both commands are production CLI surfaces.

## What Changes

- Update command help descriptions for `check` and `parse` so their purpose is
  concise, consistent, and accurate.
- Allow `check` to accept one or more template inputs as file paths or `-` for
  stdin.
- Allow `parse` to accept one or more template inputs as file paths or `-` for
  stdin.
- Add `parse --out <path>` so parse diagnostics and AST output can be written to
  a file instead of stdout/stderr.
- Compact parse AST output so it remains readable while using fewer lines.
- Preserve stdin-only workflows by accepting `-` wherever an input path is
  expected.

## Capabilities

### New Capabilities

- `cli-template-inputs`: CLI support for file and stdin template inputs,
  command help descriptions, parse output routing, and compact AST display.

### Modified Capabilities

None.

## Impact

- Affected code: CLI argument model, check command, parse command, command help,
  input reading, parse output formatting, and CLI tests.
- APIs: no parser API changes expected.
- Dependencies: no new dependencies expected.
