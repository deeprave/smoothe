## ADDED Requirements

### Requirement: Formatter-Independent Check Diagnostics

The system SHALL emit semantic check diagnostics independently of the selected
output format.

#### Scenario: Semantic diagnostics are collected before formatting

- **WHEN** schema, lambda, content, parser, or partial validation emits
  diagnostics during check
- **THEN** the check command collects those diagnostics before invoking the
  selected output formatter.

#### Scenario: Semantic validation does not write output directly

- **WHEN** semantic validation emits a diagnostic
- **THEN** it returns structured diagnostic data rather than writing directly to
  stdout or stderr.

#### Scenario: Format selection does not change validation

- **WHEN** the same input is checked using compiler-style output and JSON output
- **THEN** the same unfiltered diagnostics are produced before formatting.

#### Scenario: Severity filtering is applied after validation

- **WHEN** a diagnostic level filter is selected
- **THEN** semantic validation still produces the full diagnostic set and the
  formatter applies the display filter afterward.
