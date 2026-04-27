## 1. Inventory

- [x] 1.1 Inventory all test modules under `tests/`.
- [x] 1.2 Inventory all fixture files under `tests/fixtures/`.
- [x] 1.3 Inventory all canonical specs under `openspec/specs/`.
- [x] 1.4 Identify repeated test setup, repeated assertion shapes, obsolete tests, unused fixtures, and overlapping spec requirements.

## 2. Test Cleanup

- [x] 2.1 Consolidate repeated CLI command setup or output assertion helpers where it improves clarity.
- [x] 2.2 Convert repeated CLI test shapes to table-driven cases where failures remain clear.
- [x] 2.3 Consolidate repeated parser or content test setup where it improves clarity.
- [x] 2.4 Remove redundant or obsolete tests only when equivalent behavioral coverage remains.
- [x] 2.5 Remove fixture files that are not referenced by maintained tests.

## 3. Spec Cleanup

- [x] 3.1 Review canonical specs for duplicated, stale, or overlapping requirements.
- [x] 3.2 Update stale spec wording to match implemented behavior.
- [x] 3.3 Consolidate overlapping requirements into the most appropriate canonical capability spec.
- [x] 3.4 Validate that the `maintenance` capability delta still describes the cleanup behavior.

## 4. Verification

- [x] 4.1 Run `openspec validate maintenance-cleanup --strict`.
- [x] 4.2 Run `openspec validate --specs --strict`.
- [x] 4.3 Run `cargo fmt --check`.
- [x] 4.4 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 4.5 Run `cargo nextest run`.
