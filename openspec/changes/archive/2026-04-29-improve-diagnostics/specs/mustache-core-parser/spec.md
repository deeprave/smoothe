## MODIFIED Requirements

### Requirement: Parser Diagnostics

The system SHALL produce structured diagnostics with optional detail data and
safe partial results for recoverable parse errors.

#### Scenario: Diagnostics include source location

- **WHEN** the parser emits a diagnostic for source text with filename metadata
- **THEN** the diagnostic includes filename, line, column, issue type, severity,
  span, and message.

#### Scenario: Diagnostics can include structured details

- **WHEN** parser context provides expected values, found values, notes,
  suggestions, or related locations for a diagnostic
- **THEN** the diagnostic preserves that context as structured detail data.

#### Scenario: Feedback handler receives diagnostics

- **WHEN** parser input includes feedback handlers
- **THEN** parser diagnostics are sent to the corresponding handler based on
  severity.

#### Scenario: Recoverable error returns partial state

- **WHEN** parsing encounters recoverable syntax errors after some nodes were
  parsed
- **THEN** the parser returns safe parsed AST fragments and diagnostics.
