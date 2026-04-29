## ADDED Requirements

### Requirement: Listener-Independent Check Diagnostics

The system SHALL emit semantic check diagnostics independently of the selected
output listener.

#### Scenario: Semantic diagnostics are emitted as events

- **WHEN** schema, lambda, content, parser, or partial validation emits
  diagnostics during check
- **THEN** the check command publishes diagnostic events without requiring the
  entire check run to complete first.

#### Scenario: Semantic validation does not write output directly

- **WHEN** semantic validation emits a diagnostic
- **THEN** it emits or returns structured diagnostic data that is published as a
  check event rather than writing directly to stdout or stderr.

#### Scenario: Format selection does not change validation

- **WHEN** the same input is checked using compiler-style output and JSON output
- **THEN** the same unfiltered diagnostic events are produced.

#### Scenario: Verbosity filtering is applied by listeners

- **WHEN** a verbosity filter is selected
- **THEN** semantic validation still produces the full diagnostic event stream
  and the output listener applies the display filter afterward.
