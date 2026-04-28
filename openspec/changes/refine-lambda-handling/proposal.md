## Why

Lambda support currently distinguishes only basic known/unknown usage and
simple variable-vs-section form. To make semantic checking useful for real
templates, lambdas need a richer definition model that describes how each
lambda may be invoked, what argument shape it receives, what it returns, and
whether its behavior may have side effects.

## What Changes

- Introduce a structured lambda definition model for semantic checking.
- Model each lambda by name with attributes such as:
  - allowed usage form: variable, section, or both
  - argument shape/type
  - return shape/type
  - side-effect metadata
- Validate lambda references in templates against the supplied lambda
  definitions.
- Warn when a referenced lambda is unknown.
- Warn when a known lambda is used in an unsupported form, such as variable
  usage for section-only lambdas.
- Warn when lambda argument or return type appears incompatible with the
  surrounding Mustache usage, where this can be inferred.
- Emit an error when an inverted section references a known lambda, since
  inverted lambda sections are unsupported.
- Preserve existing behavior when no lambda definition input is supplied.
- Keep lambda validation semantic only; do not execute lambdas.

## Capabilities

### New Capabilities

- `lambda-definition-model`: Structured lambda definitions, usage rules,
  side-effect metadata, and semantic validation of lambda references.

### Modified Capabilities

- `mustache-advanced-features`: Refine lambda usage rules, especially variable
  vs section usage and unsupported inverted lambda sections.
- `mustache-parser-inputs`: Replace name-only lambda specifications with
  structured lambda definitions for semantic validation.
- `template-semantic-checks`: Validate lambda references using the structured
  lambda model and report warnings/errors for incompatible usage.

## Impact

- Affects `check` semantic validation and lambda definition loading.
- May change lambda diagnostics from broad warnings to more specific warnings
  or errors.
- Requires tests for known lambdas, unknown lambdas, variable/section
  compatibility, inverted lambda sections, type compatibility, and side-effect
  metadata.
- Does not require executing lambdas or validating runtime effects.
