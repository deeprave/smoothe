## MODIFIED Requirements

### Requirement: Periodic test suite curation

The system SHALL periodically inspect the test hierarchy for redundant,
obsolete, or low-value tests.

The system SHALL periodically update the test suite to coalesce tests using
similar patterns to use parameterisation where appropriate.

The system SHALL periodically remove any unused fixtures.

The system SHALL preserve behavioral coverage when tests are removed,
consolidated, or parameterised.

#### Scenario: Test inventory is inspected

- **WHEN** a maintenance cleanup pass begins
- **THEN** the current test modules and fixture files are inventoried before
  cleanup changes are made.

#### Scenario: Redundant test cleanup

- **WHEN** redundant or obsolete tests are identified during maintenance
- **THEN** the test suite is updated to remove or consolidate them.

#### Scenario: Repeated tests are parameterised

- **WHEN** multiple tests use the same setup and assertion shape with different
  input cases
- **THEN** the tests are consolidated into a parameterised or table-driven form
  when doing so keeps failures clear.

#### Scenario: Unused fixture cleanup

- **WHEN** fixtures are no longer referenced by maintained tests
- **THEN** those fixtures are removed.

#### Scenario: Coverage is preserved

- **WHEN** a test is removed or consolidated
- **THEN** equivalent remaining coverage exists for the behavior that test
  previously checked.

## ADDED Requirements

### Requirement: Spec Inventory Review

The system SHALL inspect canonical specifications during maintenance cleanup to
identify duplicated, stale, or overlapping requirements across capabilities.

#### Scenario: Canonical specs are inventoried

- **WHEN** a maintenance cleanup pass begins
- **THEN** the current canonical specification files are inventoried by
  capability.

#### Scenario: Stale spec wording is corrected

- **WHEN** a canonical spec requirement no longer matches implemented behavior
- **THEN** the requirement is updated through the maintenance change to match
  the implemented capability.

#### Scenario: Overlapping requirements are consolidated

- **WHEN** multiple canonical specs contain overlapping requirements for the
  same capability
- **THEN** the overlap is consolidated into the most appropriate canonical
  capability spec.
