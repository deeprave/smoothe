## ADDED Requirements

### Requirement: Event Diagnostic Projection

The system SHALL project enriched diagnostic data through check diagnostic
events and selected output listeners.

#### Scenario: Diagnostic event carries structured diagnostic data

- **WHEN** check diagnostics are emitted
- **THEN** the diagnostic event carries structured diagnostic data including
  severity, issue, source, location, span, message, and optional details.

#### Scenario: Compiler listener projects diagnostic details

- **WHEN** compiler-style check output renders a diagnostic with structured
  details
- **THEN** the listener includes useful detail text without changing the
  diagnostic issue identifier.

#### Scenario: JSON listener projects diagnostic details

- **WHEN** JSON check output renders a diagnostic with structured details
- **THEN** the listener includes those details as structured JSON fields.

#### Scenario: Listener does not mutate diagnostics

- **WHEN** a listener emits diagnostics
- **THEN** it does not change diagnostic severity, issue kind, source location,
  or exit-status behavior.
