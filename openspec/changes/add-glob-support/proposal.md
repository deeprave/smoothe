## Why

`smoothe` currently treats quoted glob operands such as `**/*.mustache` as
literal filenames, which means command behavior depends on whether a shell
expanded arguments before invocation. Callers should be able to pass glob
patterns directly to `smoothe`, including software that invokes the binary
without a shell.

## What Changes

- Expand glob patterns supplied as template input operands for `check` and
  `parse`.
- Support commands such as `smoothe check '**/*.mustache'` without relying on
  shell expansion.
- Continue accepting already-expanded file operands from modern shells.
- Preserve stdin operand handling for `-`.
- Process glob matches in deterministic order.
- Report unmatched glob patterns as input errors instead of silently succeeding.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `cli-template-inputs`: Template input operands may be glob patterns expanded
  by `smoothe` itself.

## Impact

- Affects shared template input reading used by `check` and `parse`.
- May add a small glob-matching dependency.
- Requires CLI and behavioral fixture coverage for quoted glob operands,
  already-expanded operands, unmatched patterns, and stdin preservation.
