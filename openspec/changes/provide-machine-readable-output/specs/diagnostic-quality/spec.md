## ADDED Requirements

### Requirement: Formatter Diagnostic Projection

The system SHALL project enriched diagnostic data through selected check output
formatters.

#### Scenario: Formatter receives structured diagnostic data

- **WHEN** check diagnostics are ready for output
- **THEN** the selected formatter receives structured diagnostic data including
  severity, issue, source, location, span, message, and optional details.

#### Scenario: Compiler formatter projects diagnostic details

- **WHEN** compiler-style check output renders a diagnostic with structured
  details
- **THEN** the formatter includes useful detail text without changing the
  diagnostic issue identifier.

#### Scenario: JSON formatter projects diagnostic details

- **WHEN** JSON check output renders a diagnostic with structured details
- **THEN** the formatter includes those details as structured JSON fields.

#### Scenario: Formatter does not mutate diagnostics

- **WHEN** a formatter emits diagnostics
- **THEN** it does not change diagnostic severity, issue kind, source location,
  or exit-status behavior.
