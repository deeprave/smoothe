## Context

The advanced feature stage assumes the core parser and parser-input stages are
already in place. This change focuses on preserving and validating Mustache
features that are more complex to model: lambdas, inheritance, and dynamic
names. The parser still does not render templates or execute lambda code.

## Goals / Non-Goals

**Goals:**

- Represent lambda-related template structures without executing lambdas.
- Represent Mustache inheritance syntax explicitly in the AST/state.
- Represent dynamic names explicitly in the AST/state.
- Emit diagnostics for malformed advanced-feature syntax.
- Expand fixture coverage using relevant upstream Mustache spec cases.
- Preserve enough advanced structure for future rendering and validation work.

**Non-Goals:**

- Execute lambdas.
- Implement full rendering semantics.
- Add recursive partial expansion.
- Rework the parser API established by earlier stages except where new node
  types/state fields are required.

## Decisions

### Preserve Advanced Syntax as First-Class Nodes

Inheritance and dynamic-name syntax should be represented as explicit AST nodes
or state entries rather than opaque text. This keeps future rendering and
validation possible.

Alternative considered: keep advanced syntax as unsupported diagnostics only.
That would be simpler but would not move the parser toward practical Mustache
coverage.

### Recognize Lambdas Without Runtime Execution

The parser should model lambda references and lambda-related sections where
possible, but execution remains out of scope. Runtime expansion belongs to a
future renderer.

Alternative considered: execute caller-provided lambda code during parsing. That
would make parsing nondeterministic and tightly couple it to runtime behavior.

### Use Upstream Fixtures Selectively

The change should add fixture coverage for relevant upstream Mustache spec
modules and supported cases without claiming full conformance across every
optional behavior.

Alternative considered: require full upstream conformance in this stage. That
would make the slice too large and likely delay merging useful parser support.

## Risks / Trade-offs

- [Risk] Advanced syntax semantics may differ across Mustache implementations.
  -> Mitigation: follow upstream spec fixtures where possible and document
  unsupported edge cases as diagnostics.
- [Risk] AST node expansion can destabilize prior stages. -> Mitigation: add
  advanced nodes incrementally without changing existing core node behavior.
- [Risk] Lambda modeling can be confused with lambda execution. -> Mitigation:
  keep this stage strictly parse/model/validate only.
