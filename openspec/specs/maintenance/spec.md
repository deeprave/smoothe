# Engineering Maintenance

## Purpose

Define ongoing maintenance expectations for keeping tests, fixtures, and
specifications coherent as the project evolves.

## Requirements

### Requirement: Periodic test suite curation

The system SHALL periodically remove redundant, obsolete, or low-value tests.

The system SHALL periodically update the test suite to coalesce tests using
similar patterns to use parameterisation where appropriate.

The system SHALL periodically remove any unused fixtures.

#### Scenario: Redundant test cleanup

- **WHEN** redundant or obsolete tests are identified during maintenance
- **THEN** the test suite is updated to remove or consolidate them.

#### Scenario: Unused fixture cleanup

- **WHEN** fixtures are no longer referenced by maintained tests
- **THEN** those fixtures are removed.

### Requirement: Spec consolidation

The system SHALL maintain a single canonical specification per capability,
removing duplication and redundancy.

#### Scenario: Capability specs are consolidated

- **WHEN** multiple specs describe the same capability
- **THEN** the duplicate material is consolidated into the canonical capability
  spec.

### Requirement: Maintenance cadence

Maintenance activities SHALL be performed at regular intervals or when entropy
exceeds acceptable thresholds.

#### Scenario: Maintenance is scheduled

- **WHEN** regular maintenance is due
- **THEN** the project performs maintenance activities.

#### Scenario: Entropy triggers maintenance

- **WHEN** tests, fixtures, or specs become difficult to maintain
- **THEN** the project performs maintenance activities.
