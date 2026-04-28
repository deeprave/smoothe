## Why

The `check` command is becoming useful for CI, editor integrations, and other
automation, but its diagnostic output is currently human-oriented and tied to a
single formatter. Machine consumers need stable structured output and users
need a predictable way to select output formats and diagnostic verbosity.

## What Changes

- Add machine-readable diagnostic output support to the `check` command.
- Provide an output formatting abstraction for check diagnostics so additional
  formats can be added later without rewriting validation logic.
- Support a compiler-style diagnostic format suitable for tools that understand
  `file:line:column: severity: message` output.
- Support JSON output as an explicit option for structured consumers.
- Distinguish errors, warnings, info, and debug diagnostics in all supported
  output formats.
- Add an option to restrict displayed diagnostics by severity or log level.
- Preserve accurate file, line, and column reporting across main templates and
  resolved partials.
- Keep validation and exit-status behavior independent from formatting.

## Capabilities

### New Capabilities

- `check-machine-output`: Machine-readable and selectable diagnostic output
  formats for the `check` command, including compiler-style and JSON output.

### Modified Capabilities

- `cli`: Add check command options for selecting output format and diagnostic
  verbosity.
- `configuration`: Allow check output format and diagnostic verbosity defaults
  to be configured.
- `diagnostic-quality`: Project enriched diagnostic data through check output
  formatters.
- `template-semantic-checks`: Ensure semantic check diagnostics are emitted in
  a formatter-independent way.

## Impact

- Depends on `improve-diagnostics` for structured diagnostic data and source
  accuracy.
- Affects check command CLI options, configuration resolution, diagnostic
  formatting, and tests.
- Does not add new validation rules.
- Does not change check exit status rules except to preserve them across output
  formats.
