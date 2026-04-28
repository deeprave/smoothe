## Context

The parser currently recognises configured lambdas through name-only
`LambdaSpec` values and uses that to classify lambda variable and section AST
nodes. The `check` command has a minimal JSON lambda loader with a single
`usage` field plus optional argument and return type strings. That is enough to
distinguish simple variable-vs-section usage, but it does not provide a stable
lambda model for richer semantic validation.

Lambda validation needs to answer different questions from parsing. The parser
must continue to model syntax and avoid executing user code. Semantic checking
needs to know whether a lambda name is known, which forms are allowed, what
argument shape a section lambda receives, what shape a variable or section
lambda returns, whether the return value is compatible with the surrounding
Mustache usage, and whether the lambda is declared as having side effects.

## Goals / Non-Goals

**Goals:**

- Define a structured lambda definition model for semantic checking.
- Allow lambdas to declare variable usage, section usage, or both.
- Model lambda argument and return shapes in a way that can reuse context-shape
  type concepts where practical.
- Preserve side-effect metadata for diagnostics, policy checks, and future
  integrations.
- Validate known and unknown lambda references without executing lambdas.
- Warn for unknown lambdas and incompatible variable/section usage.
- Emit an error when an inverted section resolves to a known lambda.
- Warn or error for detectable lambda type incompatibility according to the
  severity defined by the lambda rule.
- Preserve syntax-only behavior when no lambda definitions are supplied.

**Non-Goals:**

- Do not execute lambdas.
- Do not verify actual runtime side effects.
- Do not evaluate lambda output content.
- Do not infer arbitrary lambda names from normal variables unless the AST or
  supplied definitions make the reference identifiable as a lambda reference.
- Do not require lambda definitions for ordinary parsing.

## Decisions

1. Keep lambda execution out of scope.

   Lambda definitions describe allowed usage and shapes. They do not provide
   executable code and the checker must not invoke lambdas. Side-effect metadata
   is declarative and can influence diagnostics or policy later.

   Alternative considered: support test execution of lambdas. That would make
   checks dependent on runtime code and data, which is outside this static
   validation tool.

2. Introduce a semantic lambda definition model.

   A lambda definition should include at minimum:

   ```text
   LambdaDefinition
     name
     usage: variable | section | both
     argument: optional shape
     returns: optional shape
     side_effects: none | declared | unknown
   ```

   The shape fields should be compatible with the context schema model once
   that model is available, so lambda validation and context validation can
   speak the same type language.

   Alternative considered: keep the current ad hoc JSON structure private to
   `check`. That prevents consistent diagnostics and makes type compatibility
   difficult to extend.

3. Treat variable and section lambda usage as independent capabilities.

   A lambda can be valid as a variable, as a section, or as both. The checker
   should warn when a known lambda is used in a form its definition does not
   allow.

   Alternative considered: infer usage from the first occurrence. That is
   brittle and hides contradictory template usage.

4. Make inverted lambda sections errors.

   When an inverted section name resolves to a known lambda, the checker should
   emit an error because negative lambda sections are unsupported by the engine.
   This is stronger than incompatible variable/section usage because the syntax
   cannot be supported by adding type metadata.

   Alternative considered: keep inverted lambda diagnostics as warnings. That
   permits templates the engine cannot support reliably.

5. Use best-effort static type compatibility.

   Type compatibility should be checked only when both the lambda definition
   and surrounding Mustache usage provide enough information. For example,
   scalar returns used as object scopes can warn, section argument shape can be
   compared with section body expectations when context information is
   available, and unknown shapes should not produce speculative failures.

   Alternative considered: require all lambda definitions to include complete
   types. That is too strict for incremental adoption.

6. Keep lambda loading and validation in the semantic check layer.

   The parser can keep accepting name-oriented lambda data for syntax
   classification, but structured lambda validation belongs in `check`.
   Eventually, parser classification can be fed from structured definitions by
   deriving names and allowed forms.

   Alternative considered: move the entire structured model into the parser
   API immediately. That would couple parsing to semantic policy and make the
   parser responsible for concerns outside AST construction.

7. Preserve disabled/default behavior.

   If lambda checking is disabled or no lambda definitions are supplied, syntax
   parsing and check behavior should remain compatible with today’s
   syntax-only flow.

   Alternative considered: warn on every lambda-like syntax without
   definitions. That would be noisy and would make adopting the checker harder.

## Risks / Trade-offs

- Distinguishing lambdas from variables can be ambiguous. Mitigation: only
  report unknown-lambda warnings for references the parser classifies as lambda
  references or names that are checked against supplied lambda definitions.
- Type compatibility may be incomplete until the context schema model is
  refined. Mitigation: perform best-effort checks and avoid diagnostics when
  shape information is unknown.
- Side-effect metadata can imply enforcement that does not exist yet.
  Mitigation: store and surface metadata, but do not block templates solely on
  side-effect declarations in this change.
- Making inverted lambda sections errors may break templates previously treated
  as warnings. Mitigation: this aligns diagnostics with unsupported engine
  behavior and should be covered by explicit tests.
- Supporting both legacy and structured lambda inputs can complicate loading.
  Mitigation: define one structured input format for this change and emit clear
  input warnings for invalid data.

## Migration Plan

- Define the structured lambda model and input file shape.
- Update lambda definition loading to parse the structured model and report
  input warnings for invalid definitions.
- Derive parser lambda names/forms from structured definitions where needed for
  AST classification.
- Update semantic validation to use the structured model for known, unknown,
  and incompatible lambda usage.
- Change inverted known-lambda sections from warning diagnostics to error
  diagnostics.
- Add best-effort argument and return shape compatibility checks.
- Preserve `--lambdas none`, `[check] lambdas = "none"`, and syntax-only check
  behavior.
- Add tests for valid definitions, invalid definitions, known and unknown
  lambdas, variable/section/both usage, inverted lambda errors, type
  compatibility, and side-effect metadata.

## Open Questions

- Should side-effect declarations ever affect exit status, or only diagnostics?
- Should lambda type shapes reuse the context schema model directly, or use a
  smaller lambda-specific shape format and convert later?
- Should a name that exists both as a context variable and a lambda definition
  prefer lambda semantics, variable semantics, or produce an ambiguity warning?
