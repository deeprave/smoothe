## ADDED Requirements

### Requirement: Partial Diagnostic Context

The system SHALL expose partial graph context needed for rich diagnostics.

#### Scenario: Unresolved partial diagnostic includes reference name

- **WHEN** a static partial reference cannot be resolved
- **THEN** the diagnostic includes the partial reference name.

#### Scenario: Unreadable partial diagnostic includes resolved path

- **WHEN** a mapped partial file cannot be read
- **THEN** the diagnostic includes the mapped or resolved path that was
  attempted.

#### Scenario: Partial cycle diagnostic includes related location

- **WHEN** partial graph parsing reports a cycle
- **THEN** the diagnostic can include related locations for the partial
  references involved in the cycle.

### Requirement: Partial Near-Hit Candidates

The system SHALL provide known partial mapping keys as candidates for near-hit
suggestions.

#### Scenario: Unresolved partial suggests nearby mapping keys

- **WHEN** an unresolved partial name is close to known partial mapping keys
- **THEN** the diagnostic includes those nearby keys as suggestions.

#### Scenario: No nearby partial keys omits suggestions

- **WHEN** an unresolved partial diagnostic has no nearby known mapping keys
- **THEN** the diagnostic omits partial name suggestions.
