## ADDED Requirements

### Requirement: Deprecated Dependency Replacement
The system SHALL replace deprecated parsing dependencies when a maintained
compatible migration path exists for behavior the project relies on.

#### Scenario: Deprecated parser dependency has maintained successor
- **WHEN** a parser dependency used by supported behavior is deprecated
- **AND** a maintained compatible dependency supports the same behavior subset
- **THEN** the project migrates to the maintained dependency with focused
  compatibility tests.

#### Scenario: Replacement avoids speculative feature expansion
- **WHEN** replacing a deprecated parser dependency
- **THEN** the migration preserves currently supported behavior without adding
  unsupported parser feature commitments.
