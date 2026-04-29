## Draft

This is a backlog proposal only. It records follow-up cleanup work discovered
during implementation and review of `provide-machine-readable-output`. It does
not yet require delta specs, design detail, or implementation tasks.

## Why

The machine-readable check output implementation now matches the intended
behavior, but the implementation shape is heavier than it should be. The
current `check` command file contains command orchestration, semantic
validation, output option resolution, compiler-style formatting, JSON listener
state, JSON DTOs, and listener failure handling.

That concentration made the implementation harder to review and led to several
late corrections around event ordering, listener lifecycle, output failure
semantics, and JSON output shape. The behavior is now covered by tests, so a
follow-up refactor can improve maintainability without changing functionality.

## What Changes

- Extract check output listener implementation details out of
  `src/commands/check.rs`.
- Move JSON listener state and JSON DTOs into a dedicated module.
- Move compiler-style output formatting into the same output/listener area or a
  closely related module.
- Keep `src/commands/check.rs` focused on command orchestration, input loading,
  option resolution, semantic setup, and invoking validation.
- Make JSON listener lifecycle invariants easier to understand, potentially
  with an explicit internal state enum such as idle, input-active, and
  finished.
- Centralize resolved-partial semantic traversal and recursive-partial skip
  handling in a helper so event emission and diagnostics are not spread through
  the main node match.
- Clarify command-level handling of listener failures versus validation errors,
  either through helper functions or focused comments near the policy.
- Preserve all existing behavior and tests while changing the implementation
  structure.

## Capabilities

### Modified Capabilities

- `check-machine-output`: Refactor listener and output formatting internals
  without changing output behavior.
- `template-semantic-checks`: Keep semantic validation behavior unchanged while
  making resolved-partial traversal easier to maintain.

## Impact

- Affects internal organization of check output and semantic validation code.
- Should not change CLI options, configuration, JSON shape, compiler-style text
  diagnostics, event stream semantics, or exit status.
- Should be mostly mechanical because the current behavior is covered by CLI and
  event-level tests.
- No implementation tasks are defined yet; this remains a draft backlog item.
