## Why

The `check` command is becoming useful for CI, editor integrations, and other
automation, but its diagnostic output is currently human-oriented and tied to a
single formatter. Machine consumers need stable structured output and users
need a predictable way to select output formats and event verbosity.
Interactive consumers such as IDEs also need diagnostics and status information
as a stream of events rather than only as a complete batch produced after all
checking has finished.

## What Changes

- Add machine-readable diagnostic output support to the `check` command.
- Introduce a check event stream so diagnostics and check lifecycle events can
  be consumed incrementally.
- Provide event listeners for check output so multiple consumers can observe
  the same run without changing validation logic.
- Support a compiler-style diagnostic format suitable for tools that understand
  `file:line:column: severity: message` output.
- Support JSON output as an explicit option for structured consumers.
- Distinguish errors, warnings, info, and debug diagnostics in all supported
  output formats.
- Add an option to restrict displayed events by verbosity level.
- Support informational, debug, and trace-style events for progress and detailed
  traversal reporting.
- Preserve accurate file, line, and column reporting across main templates and
  resolved partials.
- Keep validation, event emission, display filtering, and exit-status behavior
  separated.

## Capabilities

### New Capabilities

- `check-machine-output`: Event-driven, machine-readable, and selectable output
  for the `check` command, including compiler-style and JSON listeners.

### Modified Capabilities

- `cli`: Add check command options for selecting output format and diagnostic
  verbosity.
- `configuration`: Allow check output format and event verbosity defaults to be
  configured.
- `diagnostic-quality`: Project enriched diagnostic data through check events
  and output listeners.
- `template-semantic-checks`: Ensure semantic check diagnostics are emitted in
  a listener-independent way.

## Impact

- Depends on `improve-diagnostics` for structured diagnostic data and source
  accuracy.
- Affects check command CLI options, configuration resolution, diagnostic
  event emission, output listeners, and tests.
- Does not add new validation rules.
- Does not change check exit status rules except to preserve them across output
  formats.
