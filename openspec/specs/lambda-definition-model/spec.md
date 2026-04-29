# lambda-definition-model Specification

## Purpose
TBD - created by archiving change refine-lambda-handling. Update Purpose after archive.
## Requirements
### Requirement: Structured Lambda Definitions

The system SHALL support structured lambda definitions for semantic checking.

#### Scenario: Lambda definition declares name

- **WHEN** a lambda definition is loaded
- **THEN** the definition identifies the lambda by a stable name.

#### Scenario: Lambda definition declares usage forms

- **WHEN** a lambda definition is loaded
- **THEN** the definition declares whether the lambda supports variable usage,
  section usage, or both.

#### Scenario: Lambda definition declares argument shape

- **WHEN** a lambda definition includes an argument shape
- **THEN** the system preserves that shape for semantic compatibility checks.

#### Scenario: Lambda definition declares return shape

- **WHEN** a lambda definition includes a return shape
- **THEN** the system preserves that shape for semantic compatibility checks.

#### Scenario: Lambda definition declares side-effect metadata

- **WHEN** a lambda definition includes side-effect metadata
- **THEN** the system preserves that metadata without executing the lambda.

### Requirement: Lambda Definition Loading

The system SHALL load structured lambda definitions from the configured lambda
definition input.

#### Scenario: Valid structured lambda file is accepted

- **WHEN** the check command loads a valid structured lambda definition file
- **THEN** the system uses those definitions for lambda semantic validation.

#### Scenario: Invalid structured lambda file emits warning

- **WHEN** the check command loads a lambda definition file that is not valid or
  not recognisable
- **THEN** the system emits a warning diagnostic for the lambda input.

#### Scenario: Lambda checking is disabled by default

- **WHEN** the check command is run without lambda definition input
- **THEN** the system does not require lambda definitions for syntax checking.

#### Scenario: Lambdas none disables checking

- **WHEN** lambda definition input is configured as `none` with any letter
  casing
- **THEN** the system does not load a lambda definition file and does not
  perform lambda semantic validation.

### Requirement: Lambda Usage Compatibility

The system SHALL validate lambda references against structured lambda
definitions without executing lambda code.

#### Scenario: Known variable lambda is accepted

- **WHEN** a template uses a variable tag whose name matches a lambda definition
  that allows variable usage
- **THEN** the system does not emit an incompatible-lambda-usage diagnostic for
  that reference.

#### Scenario: Known section lambda is accepted

- **WHEN** a template uses a positive section whose name matches a lambda
  definition that allows section usage
- **THEN** the system does not emit an incompatible-lambda-usage diagnostic for
  that section.

#### Scenario: Both-usage lambda accepts variable and section

- **WHEN** a lambda definition allows both variable and section usage
- **THEN** the system accepts either usage form for that lambda.

#### Scenario: Unsupported variable usage warns

- **WHEN** a template uses a known section-only lambda as a variable
- **THEN** the system emits a warning diagnostic for incompatible lambda usage.

#### Scenario: Unsupported section usage warns

- **WHEN** a template uses a known variable-only lambda as a positive section
- **THEN** the system emits a warning diagnostic for incompatible lambda usage.

#### Scenario: Unknown names remain ordinary references

- **WHEN** a template reference does not match a supplied lambda definition
- **THEN** the system does not infer that the reference is an unknown lambda.

### Requirement: Inverted Lambda Rejection

The system SHALL reject inverted sections that resolve to known lambdas.

#### Scenario: Known inverted lambda section emits error

- **WHEN** a template uses an inverted section whose name resolves to a known
  lambda
- **THEN** the system emits an error diagnostic because inverted lambda
  sections are unsupported.

#### Scenario: Inverted non-lambda section remains non-lambda

- **WHEN** a template uses an inverted section whose name does not resolve to a
  known lambda
- **THEN** the system does not emit an inverted-lambda diagnostic for that
  section.

### Requirement: Lambda Type Compatibility

The system SHALL perform best-effort static compatibility checks for lambda
argument and return shapes where enough information is available.

#### Scenario: Compatible return shape is accepted

- **WHEN** a known lambda return shape is compatible with the surrounding
  Mustache usage
- **THEN** the system does not emit a lambda type compatibility diagnostic for
  that usage.

#### Scenario: Incompatible return shape warns

- **WHEN** a known lambda return shape is detectably incompatible with the
  surrounding Mustache usage
- **THEN** the system emits a warning diagnostic for lambda type compatibility.

#### Scenario: Incompatible section argument warns

- **WHEN** a known section lambda argument shape is detectably incompatible with
  the section body expectations
- **THEN** the system emits a warning diagnostic for lambda type compatibility.

#### Scenario: Unknown shape does not warn speculatively

- **WHEN** lambda argument or return shape information is unknown or absent
- **THEN** the system does not emit speculative type compatibility diagnostics.

### Requirement: Lambda Side-Effect Metadata

The system SHALL preserve declared lambda side-effect metadata for diagnostics
and future policy checks.

#### Scenario: Side-effect metadata is loaded

- **WHEN** a lambda definition declares side-effect metadata
- **THEN** the system stores that metadata with the lambda definition.

#### Scenario: Side-effect metadata does not execute lambda

- **WHEN** semantic validation checks a lambda with side-effect metadata
- **THEN** the system does not execute the lambda or verify runtime side
  effects.

#### Scenario: Side-effect metadata does not fail by default

- **WHEN** a lambda definition declares side effects
- **THEN** semantic validation does not fail solely because side effects are
  declared.

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

