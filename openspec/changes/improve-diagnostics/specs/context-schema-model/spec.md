## ADDED Requirements

### Requirement: Schema Diagnostic Context

The system SHALL expose schema model context needed for rich diagnostics.

#### Scenario: Missing path diagnostic includes known fields

- **WHEN** a missing schema path is reported in an object scope with known
  fields
- **THEN** the diagnostic includes the known fields for that scope.

#### Scenario: Optional path diagnostic includes optionality

- **WHEN** an optional schema path is reported
- **THEN** the diagnostic identifies that the path exists but is not required.

#### Scenario: Enum diagnostic includes allowed values

- **WHEN** a diagnostic involves a shape with enum values
- **THEN** the diagnostic includes the allowed enum values when useful.

#### Scenario: Scalar traversal diagnostic includes found shape

- **WHEN** a dotted path attempts to traverse a scalar shape
- **THEN** the diagnostic identifies the scalar shape that was found.

### Requirement: Schema Near-Hit Candidates

The system SHALL provide local schema candidate sets for near-hit suggestions.

#### Scenario: Object fields provide candidates

- **WHEN** a schema diagnostic is emitted inside an object scope
- **THEN** the known fields in that object scope are available as suggestion
  candidates.

#### Scenario: Permissive objects avoid speculative candidates

- **WHEN** an object scope is permissive and no fixed field set is available
- **THEN** the system does not emit speculative schema field suggestions.
