## MODIFIED Requirements

### Requirement: Periodic test suite curation

The system SHALL periodically inspect the test hierarchy for redundant,
obsolete, or low-value tests.

The system SHALL periodically update the test suite to coalesce tests using
similar patterns to use parameterisation where appropriate.

The system SHALL periodically remove any unused fixtures.

The system SHALL preserve behavioral coverage when tests are removed,
consolidated, or parameterised.

The system SHALL periodically inspect behavioral fixture coverage to ensure
black-box CLI behavior remains represented by maintained fixture cases.

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

#### Scenario: Behavioral fixtures are inventoried

- **WHEN** a maintenance cleanup pass inspects the test hierarchy
- **THEN** behavioral fixture cases are inventoried separately from
  implementation-focused tests.

#### Scenario: Behavioral fixture coverage is preserved

- **WHEN** behavioral fixtures are removed, updated, or consolidated
- **THEN** equivalent black-box CLI coverage remains for the behavior they
  represented.
