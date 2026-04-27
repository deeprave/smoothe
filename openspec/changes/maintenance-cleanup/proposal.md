## Why

The current test suite and freshly archived spec inventory have grown through a
series of feature changes. This maintenance pass will reduce duplicated test
patterns, remove obsolete fixtures or coverage, and verify that canonical specs
remain consolidated by capability.

## What Changes

- Inspect the current test hierarchy for redundant assertions, repeated setup,
  and cases that can be parameterised without reducing coverage.
- Inspect the canonical spec inventory for duplicated, stale, or overlapping
  requirements.
- Consolidate test helpers and convert repeated test shapes to parameterised
  cases where doing so improves clarity.
- Remove unused fixtures or obsolete tests found during the inventory.
- Keep canonical specs matched to implemented capabilities after cleanup.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `maintenance`: Add concrete requirements for test/spec inventory-driven
  cleanup, redundancy removal, and parameterisation during maintenance passes.

## Impact

- Affected code: test modules under `tests/`, test fixtures under
  `tests/fixtures/`, and canonical OpenSpec specs under `openspec/specs/`.
- APIs: no production API changes are expected.
- Dependencies: no dependency changes are expected.
