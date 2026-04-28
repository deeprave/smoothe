## ADDED Requirements

### Requirement: Structured Lambda Definition Validation

The system SHALL allow semantic template checks to use structured lambda
definitions describing known lambdas, allowed usage, argument shape, return
shape, and side-effect metadata.

#### Scenario: Valid structured lambda definitions are accepted

- **WHEN** the check command is run with valid structured lambda definitions
- **THEN** the system uses those definitions for semantic lambda validation.

#### Scenario: Invalid structured lambda definitions emit warning

- **WHEN** the check command is run with lambda definitions that cannot be
  parsed or recognised
- **THEN** the system emits a warning diagnostic for the lambda definitions.

#### Scenario: Lambda checking is disabled by default

- **WHEN** the check command is run without lambda definition input
- **THEN** the system does not perform lambda definition validation.

#### Scenario: Lambdas none disables checking

- **WHEN** lambda definition input is configured as `none` with any letter
  casing
- **THEN** the system does not load a lambda definition file and does not
  perform lambda definition validation.

### Requirement: Structured Lambda Usage Validation

The system SHALL validate lambda references against structured lambda
definitions when they are available.

#### Scenario: Known variable lambda is accepted

- **WHEN** a template uses a variable tag whose name matches a lambda definition
  that allows variable usage
- **THEN** the system does not emit an unknown-lambda or invalid-lambda-usage
  warning for that reference.

#### Scenario: Known section lambda is accepted

- **WHEN** a template uses a positive section whose name matches a lambda
  definition that allows section usage
- **THEN** the system does not emit an unknown-lambda or invalid-lambda-usage
  warning for that section.

#### Scenario: Unknown lambda emits warning

- **WHEN** a template reference can be identified as a lambda reference but no
  matching lambda definition is provided
- **THEN** the system emits a warning diagnostic for that reference.

#### Scenario: Incompatible lambda usage emits warning

- **WHEN** a template uses a known lambda in a form that is not allowed by its
  definition
- **THEN** the system emits a warning diagnostic for that reference.

#### Scenario: Inverted lambda section emits error

- **WHEN** a template uses an inverted section whose name resolves to a known
  lambda
- **THEN** the system emits an error diagnostic because inverted lambda sections
  are unsupported.

#### Scenario: Lambda type incompatibility emits warning

- **WHEN** a template uses a known lambda in a context that is detectably
  incompatible with its declared argument or return shape
- **THEN** the system emits a warning diagnostic for that reference.

#### Scenario: Unknown lambda shape avoids speculative warning

- **WHEN** a lambda definition omits argument or return shape information
- **THEN** the system does not emit speculative lambda type compatibility
  warnings for that missing shape information.

### Requirement: Lambda Side-Effect Semantic Metadata

The system SHALL preserve lambda side-effect metadata during semantic
validation without executing lambdas.

#### Scenario: Side-effect metadata is retained

- **WHEN** a lambda definition declares side-effect metadata
- **THEN** semantic validation can access that metadata while checking lambda
  references.

#### Scenario: Declared side effects do not fail check by default

- **WHEN** a lambda definition declares side effects
- **THEN** semantic validation does not fail solely because side effects are
  declared.

#### Scenario: Lambda validation does not execute lambda

- **WHEN** semantic validation checks a lambda definition with side-effect
  metadata
- **THEN** the system does not execute that lambda.
