# template-partial-graph Specification

## Purpose
TBD - created by archiving change add-full-partial-support. Update Purpose after archive.
## Requirements
### Requirement: Static Partial Resolution

The system SHALL resolve static Mustache partial tags through the effective
partial mapping available during parsing.

#### Scenario: Mapped static partial resolves to path

- **WHEN** a template references `{{> header}}` and the effective partial
  mapping contains `header`
- **THEN** the system resolves the partial tag to the mapped filesystem path.

#### Scenario: Unmapped static partial emits error

- **WHEN** a template references `{{> header}}` and the effective partial
  mapping does not contain `header`
- **THEN** the system emits an error diagnostic for the unresolved partial.

#### Scenario: Unreadable mapped partial emits error

- **WHEN** a template references a mapped partial whose resolved file cannot be
  read
- **THEN** the system emits an error diagnostic for the unresolved partial file.

### Requirement: Full Partial Graph Parsing

The system SHALL parse resolved static partial files as separate Mustache
template units in the same resolved template graph.

#### Scenario: Mapped partial is parsed into graph

- **WHEN** a template references a mapped partial whose file is readable
- **THEN** the system parses the partial template as a separate template unit
  and links that unit into the template graph.

#### Scenario: Nested mapped partial is parsed into graph

- **WHEN** a resolved partial references another mapped partial
- **THEN** the system resolves and parses the nested partial as part of the same
  template graph.

#### Scenario: Recursive partial reference is preserved

- **WHEN** partial graph parsing encounters a partial path already active in the
  current resolution stack
- **THEN** the system preserves the reference to the already parsed template
  unit without expanding that branch indefinitely.

### Requirement: Partial AST Graph Inclusion

The system SHALL include resolved partial template units in the returned AST
graph while preserving the original partial reference.

#### Scenario: Resolved partial AST preserves reference name

- **WHEN** a template contains `{{> header}}` and `header` resolves
- **THEN** the AST includes a resolved partial node containing the partial name
  `header`.

#### Scenario: Resolved partial AST links template unit

- **WHEN** a resolved partial file contains parseable Mustache content
- **THEN** the AST partial node links to the parsed partial template unit.

#### Scenario: Semantic validation traverses resolved partial units

- **WHEN** semantic validation walks an AST containing resolved partial nodes
- **THEN** it validates variables, sections, lambdas, and nested partial content
  inside linked resolved partial template units.

#### Scenario: Source units retain independent section balance

- **WHEN** parsing a template graph with resolved partials
- **THEN** section open and close tags are matched only within the same template
  unit and are not matched across partial boundaries.

### Requirement: Partial Source Metadata

The system SHALL preserve source metadata for each resolved partial and its
parsed body.

#### Scenario: Resolved partial records path

- **WHEN** a partial tag is resolved to a file
- **THEN** the resolved partial AST metadata includes the resolved file path.

#### Scenario: Partial diagnostics use partial source location

- **WHEN** parsing a partial body produces a diagnostic
- **THEN** the diagnostic reports the partial source name and original line and
  column within that partial file.

#### Scenario: Partial reference span is preserved

- **WHEN** a template contains a resolved partial tag
- **THEN** the AST preserves the source span of the original partial tag in the
  referring template.

### Requirement: Dynamic Partial Preservation

The system SHALL preserve dynamic partial tags as runtime references without
attempting filesystem resolution during parsing.

#### Scenario: Dynamic partial remains dynamic

- **WHEN** a template contains a dynamic partial tag whose target depends on
  runtime context
- **THEN** the parser preserves the dynamic partial node without resolving a
  file.

#### Scenario: Dynamic partial does not emit static unresolved error

- **WHEN** a dynamic partial tag cannot be resolved at parse time
- **THEN** the system does not emit an unresolved static partial diagnostic for
  that dynamic partial.

### Requirement: Partial Diagnostic Context

The system SHALL expose partial graph context needed for rich diagnostics.

#### Scenario: Unresolved partial diagnostic includes reference name

- **WHEN** a static partial reference cannot be resolved
- **THEN** the diagnostic includes the partial reference name.

#### Scenario: Unreadable partial diagnostic includes resolved path

- **WHEN** a mapped partial file cannot be read
- **THEN** the diagnostic includes the mapped or resolved path that was
  attempted.

#### Scenario: Recursive partial references are not reported as cycles

- **WHEN** a partial reference is recursive and the parser preserves it as a
  recursive graph edge
- **THEN** the system does not emit a speculative cycle diagnostic.

### Requirement: Partial Near-Hit Candidates

The system SHALL provide known partial mapping keys as candidates for near-hit
suggestions.

#### Scenario: Unresolved partial suggests nearby mapping keys

- **WHEN** an unresolved partial name is close to known partial mapping keys
- **THEN** the diagnostic includes those nearby keys as suggestions.

#### Scenario: No nearby partial keys omits suggestions

- **WHEN** an unresolved partial diagnostic has no nearby known mapping keys
- **THEN** the diagnostic omits partial name suggestions.

