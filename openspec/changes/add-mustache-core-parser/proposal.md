## Why

`smoothe` needs a parser foundation before it can validate templates, inspect
references, or support future rendering. The first stage should establish the
AST, source-span, delimiter, and diagnostic model without taking on every
advanced Mustache feature at once.

## What Changes

- Add a decoupled parser module for core Mustache syntax.
- Parse templates into an abstract syntax tree with source spans.
- Support core nodes: text, escaped variables, unescaped variables, comments,
  sections, inverted sections, and partial references.
- Track delimiter changes so later tags use the active delimiter pair.
- Validate section nesting, unmatched closing tags, unclosed sections, and
  malformed tags.
- Return best-effort AST/state where safe while emitting structured diagnostics.
- Support caller-provided feedback handlers for errors, warnings, info, and
  optional debug events.
- Keep the parser independent from CLI command behavior.

## Capabilities

### New Capabilities

- `mustache-core-parser`: Core Mustache tokenization, AST construction, source
  spans, delimiter handling, structural validation, and diagnostics.

### Modified Capabilities

None.

## Impact

- Affected code: parser module, AST data structures, tokenization, source-span
  handling, diagnostic types, parser result/state, and tests/spec fixtures.
- Dependencies: no advanced parser-input dependencies are expected for this
  first slice unless needed for diagnostics.
- APIs: introduces the initial parser input/result boundary for source text,
  source metadata, feedback handlers, AST, and parser state.
