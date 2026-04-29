# template-semantic-checks Specification

## Purpose
TBD - created by archiving change add-variable-checks. Update Purpose after archive.
## Requirements
### Requirement: Context Schema Loading

The system SHALL allow semantic template checks to use an optional JSON context schema that describes the shape of the template context.

#### Scenario: Valid context schema is accepted

- **WHEN** the check command is run with a valid and recognisable JSON context schema
- **THEN** the system uses that schema for semantic variable validation.

#### Scenario: Invalid JSON schema input emits warning

- **WHEN** the check command is run with schema input that is not valid JSON
- **THEN** the system emits a warning diagnostic for the schema input.

#### Scenario: Unrecognisable schema input emits warning

- **WHEN** the check command is run with JSON schema input that does not have a recognisable context-shape schema structure
- **THEN** the system emits a warning diagnostic for the schema input.

#### Scenario: Schema checking is disabled by default

- **WHEN** the check command is run without schema input
- **THEN** the system does not perform context variable validation.

#### Scenario: Schema none disables checking

- **WHEN** schema input is configured as `none` with any letter casing
- **THEN** the system does not load a schema file and does not perform context variable validation.

### Requirement: Context Variable Validation

The system SHALL validate Mustache variable and section references against the supplied context schema when one is provided.

#### Scenario: Known variable path is accepted

- **WHEN** a template references a variable path that exists in the supplied context schema for the current scope
- **THEN** the system does not emit an unknown-variable warning for that reference.

#### Scenario: Unknown variable path emits warning

- **WHEN** a template references a variable path that is not present in the supplied context schema for the current scope
- **THEN** the system emits a warning diagnostic for that reference.

#### Scenario: Object section changes validation scope

- **WHEN** a template section references an object path from the supplied context schema
- **THEN** the system validates the section body relative to that object scope.

#### Scenario: Array section changes validation scope

- **WHEN** a template section references an array path from the supplied context schema
- **THEN** the system validates the section body relative to the array item scope.

#### Scenario: Dotted variable path is resolved by segment

- **WHEN** a template references a dotted variable path
- **THEN** the system validates each path segment against the current schema scope.

#### Scenario: Unexpected variable usage emits warning

- **WHEN** a template uses a schema path in a way that is incompatible with the recognised type for that path
- **THEN** the system emits a warning diagnostic for that reference.

### Requirement: Lambda Definition Loading

The system SHALL allow semantic template checks to use optional lambda definitions describing known lambdas and their allowed usage.

#### Scenario: Valid lambda definitions are accepted

- **WHEN** the check command is run with valid lambda definitions
- **THEN** the system uses those definitions for semantic lambda validation.

#### Scenario: Invalid lambda definitions emit warning

- **WHEN** the check command is run with lambda definitions that cannot be parsed or recognised
- **THEN** the system emits a warning diagnostic for the lambda definitions.

#### Scenario: Lambda checking is disabled by default

- **WHEN** the check command is run without lambda definition input
- **THEN** the system does not perform lambda definition validation.

#### Scenario: Lambdas none disables checking

- **WHEN** lambda definition input is configured as `none` with any letter casing
- **THEN** the system does not load a lambda definition file and does not perform lambda definition validation.

### Requirement: Lambda Usage Validation

The system SHALL validate lambda references against supplied lambda definitions when they are available.

#### Scenario: Known variable lambda is accepted

- **WHEN** a template uses a variable tag whose name matches a lambda definition that allows variable usage
- **THEN** the system does not emit an unknown-lambda or invalid-lambda-usage warning for that reference.

#### Scenario: Known section lambda is accepted

- **WHEN** a template uses a positive section whose name matches a lambda definition that allows section usage
- **THEN** the system does not emit an unknown-lambda or invalid-lambda-usage warning for that section.

#### Scenario: Unknown lambda emits warning

- **WHEN** a template reference can be identified as a lambda reference but no matching lambda definition is provided
- **THEN** the system emits a warning diagnostic for that reference.

#### Scenario: Incompatible lambda usage emits warning

- **WHEN** a template uses a known lambda in a form that is not allowed by its definition
- **THEN** the system emits a warning diagnostic for that reference.

#### Scenario: Inverted lambda section emits warning

- **WHEN** a template uses an inverted section whose name resolves to a known lambda
- **THEN** the system emits a warning diagnostic because inverted lambda sections are unsupported.

#### Scenario: Lambda type incompatibility emits warning

- **WHEN** a template uses a known lambda in a context that is incompatible with its declared argument or return type
- **THEN** the system emits a warning diagnostic for that reference.

### Requirement: Syntax-Only Check Compatibility

The system SHALL preserve existing check behavior when no context schema or lambda definitions are supplied.

#### Scenario: Check without semantic inputs remains syntax-only

- **WHEN** the check command is run without schema or lambda definition inputs
- **THEN** the system reports content and parser diagnostics without requiring semantic validation inputs.

