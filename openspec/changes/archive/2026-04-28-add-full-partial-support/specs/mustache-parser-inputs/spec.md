## REMOVED Requirements

### Requirement: One-Level Partial Parsing

**Reason**: Full template validation requires resolved partial graph parsing
and AST graph inclusion rather than one-level parsing into parser state.

**Migration**: Use `Full Partial Graph Parsing` in this capability. Consumers
that previously read parsed partial ASTs from parser state SHALL traverse the
returned AST graph resolved partial nodes instead.

## ADDED Requirements

### Requirement: Full Partial Graph Parsing

The system SHALL resolve configured static partials relative to the source
template root and parse the full reachable static partial graph as separate
template units.

#### Scenario: Configured partial is resolved

- **WHEN** a template references `{{> header}}` and parser input maps `header`
  to `partials/header.mustache`
- **THEN** the parser resolves that path relative to the source template root.

#### Scenario: Configured partial is parsed

- **WHEN** a resolved partial file is readable
- **THEN** the parser parses the partial source as a separate template unit and
  includes that unit in the returned AST graph.

#### Scenario: Missing partial reports diagnostic

- **WHEN** a template references a partial absent from the partial mapping
- **THEN** the parser emits an error diagnostic for the unresolved partial.

#### Scenario: Nested partial is resolved into graph

- **WHEN** a resolved partial references another mapped partial
- **THEN** the parser resolves and parses the nested partial source in the same
  parse operation.

#### Scenario: Partial graph preserves source metadata

- **WHEN** parser input resolves and parses partial files
- **THEN** each parsed partial retains its partial name, resolved path, body
  offset, and body starting line metadata.

#### Scenario: Recursive partial reference preserves graph

- **WHEN** recursive partial parsing detects that a partial path is already in
  the active resolution stack
- **THEN** the parser records a recursive reference to the existing parsed
  template unit without expanding that branch indefinitely.
