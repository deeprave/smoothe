## MODIFIED Requirements

### Requirement: JSON Diagnostics Projection

The system SHALL include parser diagnostics in JSON mode as structured lists
grouped by severity, preserving both core diagnostic fields and optional
structured detail data.

#### Scenario: Error diagnostics are grouped

- **WHEN** parsing produces one or more error diagnostics in JSON mode
- **THEN** the input result includes those diagnostics in an `errors` list.

#### Scenario: Warning diagnostics are grouped

- **WHEN** parsing produces one or more warning diagnostics in JSON mode
- **THEN** the input result includes those diagnostics in a `warnings` list.

#### Scenario: Diagnostics include location and message

- **WHEN** a diagnostic appears in the `errors` or `warnings` list
- **THEN** the diagnostic object includes the issue kind, source name, line,
  column, span, and message.

#### Scenario: Diagnostics include structured details

- **WHEN** a diagnostic has expected, found, note, suggestion, or related
  location details
- **THEN** the diagnostic object includes those details in optional structured
  fields.

#### Scenario: No diagnostics uses empty lists

- **WHEN** parsing produces no error or warning diagnostics in JSON mode
- **THEN** the input result includes empty `errors` and `warnings` lists.
