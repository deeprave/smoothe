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

### Requirement: Lambda Suggestion Boundaries

The system SHALL avoid speculative lambda suggestions when no diagnostic can
reliably identify a reference as an unknown lambda.

#### Scenario: Ordinary references do not receive lambda suggestions

- **WHEN** a template reference does not match a supplied lambda definition
- **AND** ordinary Mustache syntax cannot distinguish it from a context
  variable
- **THEN** the diagnostic omits lambda name suggestions.

#### Scenario: Known lambda diagnostics use definition context

- **WHEN** validation emits a diagnostic for a known lambda
- **THEN** the diagnostic uses that lambda definition for expected usage,
  actual usage, shape context, and side-effect metadata.
