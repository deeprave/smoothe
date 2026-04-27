## Context

The project has accumulated tests and canonical specs through several completed
feature changes. The current test inventory is small enough to inspect
directly, but it already contains repeated CLI command setup, repeated output
assertion shapes, and a mix of fixture-backed and inline template cases.

The spec inventory has also just been expanded from archived changes. The
maintenance capability now provides the standing expectation that tests,
fixtures, and specs should be kept coherent, but this change needs a concrete
process for applying that expectation to the current repository.

## Goals / Non-Goals

**Goals:**

- Inventory tests, fixtures, and canonical specs before changing them.
- Remove duplicated or obsolete test coverage without reducing behavioral
  coverage.
- Parameterise repeated test shapes where that improves clarity and lowers
  maintenance cost.
- Remove unused fixtures found during the inventory.
- Keep canonical specs valid and aligned with implemented capabilities.

**Non-Goals:**

- Do not change production behavior.
- Do not rename public APIs or command-line options.
- Do not collapse separate tests when doing so would make failures harder to
  diagnose.
- Do not introduce new test framework dependencies unless the existing Rust test
  stack cannot express the cleanup clearly.

## Decisions

### Start With an Explicit Inventory

The cleanup should first enumerate current test modules, fixture files, and
canonical specs, then classify each finding as duplicated setup, duplicated
assertion shape, obsolete coverage, unused fixture, or spec overlap.

Alternative considered: immediately refactor obvious repeated tests. That is
faster, but it risks moving code without a clear record of why each cleanup is
safe.

### Prefer Local Helpers and Table-Driven Tests

Repeated CLI and parser test shapes should be consolidated with local helpers or
table-driven loops inside the existing `tests/` hierarchy. This keeps production
modules free of test-only code and follows the repository convention that tests
live outside production modules.

Alternative considered: introduce a parameterised-test crate. The current suite
does not need additional dependencies for this maintenance pass, so regular Rust
loops and helper functions are the smaller change.

### Preserve Behavioral Assertions While Reducing Duplication

Every removed or consolidated test should map to equivalent remaining coverage.
When a test is removed as redundant, the implementation should be able to point
to the retained test or parameterised case that still checks the behavior.

Alternative considered: use line-count reduction as the primary goal. That can
make the suite harder to read, so clarity and coverage preservation take
priority over minimizing file size.

### Treat Specs as Capability Contracts

Spec cleanup should preserve one canonical spec per capability and avoid moving
requirements between specs unless the target capability is a better semantic
match. Any stale text discovered during the inventory should be updated through
the maintenance delta so archived history and current behavior remain coherent.

Alternative considered: leave specs untouched unless validation fails. That
would miss redundant or stale capability wording that still validates
syntactically.

## Risks / Trade-offs

- Coverage accidentally removed -> Map each removed test to retained coverage
  and run `cargo nextest run`.
- Parameterised tests become less readable -> Only parameterise repeated shapes
  where case names and failure messages stay clear.
- Fixture deletion breaks hidden assumptions -> Search for all fixture
  references before removal and run the full test suite.
- Spec consolidation changes intent -> Keep spec edits scoped to the
  maintenance capability and current capability inventory.
