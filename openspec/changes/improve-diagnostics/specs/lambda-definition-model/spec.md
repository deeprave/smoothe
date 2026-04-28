## ADDED Requirements

### Requirement: Lambda Diagnostic Context

The system SHALL expose lambda definition context needed for rich diagnostics.

#### Scenario: Usage diagnostic includes expected and actual usage

- **WHEN** lambda validation reports incompatible usage
- **THEN** the diagnostic includes the expected usage forms and the actual
  usage form.

#### Scenario: Type diagnostic includes shape context

- **WHEN** lambda validation reports type incompatibility
- **THEN** the diagnostic includes available argument or return shape context.

#### Scenario: Side-effect metadata is available to diagnostics

- **WHEN** lambda validation reports a diagnostic involving a lambda with
  side-effect metadata
- **THEN** the diagnostic can include that side-effect metadata.

### Requirement: Lambda Near-Hit Candidates

The system SHALL provide known lambda names as candidates for near-hit
suggestions.

#### Scenario: Unknown lambda suggests nearby known names

- **WHEN** an unknown lambda diagnostic is emitted and nearby known lambda names
  exist
- **THEN** the diagnostic includes those nearby lambda names as suggestions.

#### Scenario: No nearby lambda names omits suggestions

- **WHEN** an unknown lambda diagnostic is emitted and no nearby known lambda
  names exist
- **THEN** the diagnostic omits lambda name suggestions.
