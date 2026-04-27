## Why

After the core parser and configurable inputs exist, `smoothe` needs support for
advanced Mustache features used by real-world templates. This stage extends the
parser model toward broader Mustache compatibility without mixing that
complexity into the initial parser foundation.

## What Changes

- Add explicit parser support for lambda-related Mustache structures where they
  can be modeled without executing runtime code.
- Add support for Mustache inheritance syntax.
- Add support for dynamic names.
- Preserve enough advanced-feature structure in the AST for future validation
  and rendering work.
- Expand upstream Mustache spec fixture coverage for the supported advanced
  feature set.

## Capabilities

### New Capabilities

- `mustache-advanced-features`: Advanced Mustache parser support for lambdas,
  inheritance, dynamic names, and broader spec fixture coverage.

### Modified Capabilities

None.

## Impact

- Affected code: parser AST node kinds, token classification, validation rules,
  diagnostics, and test/spec fixtures.
- Dependencies: none expected beyond parser dependencies already introduced by
  prior stages.
- APIs: extends parser AST/state to preserve advanced feature structure.
