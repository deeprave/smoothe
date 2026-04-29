## ADDED Requirements

### Requirement: Structured Diagnostic Details

The system SHALL support structured diagnostic detail data in addition to a
human-readable diagnostic message.

#### Scenario: Diagnostic includes expectation context

- **WHEN** a diagnostic is emitted because an observed value does not match a
  validation rule
- **THEN** the diagnostic can include what was expected and what was found.

#### Scenario: Diagnostic includes expectation source

- **WHEN** a diagnostic expectation comes from schema, lambda, partial mapping,
  parser state, or content metadata
- **THEN** the diagnostic can include where that expectation came from.

#### Scenario: Diagnostic includes notes

- **WHEN** additional explanatory context is available for a diagnostic
- **THEN** the diagnostic can include structured notes without changing the
  stable issue identifier.

#### Scenario: Diagnostic preserves basic fields

- **WHEN** structured detail data is attached to a diagnostic
- **THEN** the diagnostic still includes severity, issue kind, source name,
  line, column, span, and message.

### Requirement: Near-Hit Suggestions

The system SHALL provide bounded near-hit suggestions when a local candidate
set is available.

#### Scenario: Schema field suggestion is available

- **WHEN** a missing schema path segment is close to a known field in the
  current schema scope
- **THEN** the diagnostic includes a bounded suggestion list containing nearby
  field names.

#### Scenario: Lambda name suggestion is available

- **WHEN** an unknown lambda name is close to a known lambda definition
- **THEN** the diagnostic includes a bounded suggestion list containing nearby
  lambda names.

#### Scenario: Partial name suggestion is available

- **WHEN** an unresolved partial name is close to a known partial mapping key
- **THEN** the diagnostic includes a bounded suggestion list containing nearby
  partial names.

#### Scenario: No local candidates suppresses suggestions

- **WHEN** no local candidate set is available for a diagnostic
- **THEN** the system does not emit speculative suggestions.

### Requirement: Source-Accurate Diagnostic Reporting

The system SHALL preserve accurate source identity and location for diagnostics
across primary templates and resolved partials.

#### Scenario: Main template diagnostic uses main source

- **WHEN** a diagnostic originates in the main template
- **THEN** the diagnostic source name, line, column, and span refer to the main
  template.

#### Scenario: Partial diagnostic uses partial source

- **WHEN** a diagnostic originates in a resolved partial
- **THEN** the diagnostic source name, line, column, and span refer to the
  partial file.

#### Scenario: Related source location is available

- **WHEN** a diagnostic has both an origin and a related location such as a
  referring partial tag
- **THEN** the diagnostic can include the related source location as structured
  detail.

### Requirement: Cascade-Aware Semantic Diagnostics

The system SHALL avoid emitting misleading cascades when an earlier semantic
diagnostic makes later diagnostics dependent on an unknown scope.

#### Scenario: Unknown section scope suppresses child missing-path cascades

- **WHEN** a section path is missing from the context schema
- **AND** child references inside that section could depend on the missing
  section scope
- **THEN** the system emits the missing-path diagnostic for the section
- **AND** the system does not emit separate missing-path diagnostics for each
  child reference as though they were definitely in the outer scope.

#### Scenario: Unknown section scope reports incomplete child validation

- **WHEN** child references are skipped because their enclosing section scope
  is unknown
- **THEN** the system emits at most one warning or note explaining that
  references inside the unknown section could not be fully validated.

#### Scenario: Known section scope still validates children

- **WHEN** a section path resolves to a known object, array, boolean, or lambda
  behavior
- **THEN** the system continues validating child references according to the
  resolved section semantics.

### Requirement: Diagnostic Text Rendering

The system SHALL render human-readable diagnostic output from the structured
diagnostic data.

#### Scenario: Text output includes core summary

- **WHEN** a diagnostic is printed in text output
- **THEN** the output includes severity, issue kind, source, line, column, and
  message.

#### Scenario: Text output includes useful details

- **WHEN** structured diagnostic details are available
- **THEN** text output includes useful expected, found, source, note, or
  suggestion details without requiring callers to parse JSON.

#### Scenario: Stable issue identifiers are preserved

- **WHEN** diagnostic text is improved
- **THEN** the issue kind identifier remains stable unless a validation rule
  explicitly introduces a new issue kind.
