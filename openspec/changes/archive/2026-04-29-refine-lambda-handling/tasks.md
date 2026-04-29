## 1. Lambda Model

- [x] 1.1 Define a structured lambda definition model with name, usage forms, argument shape, return shape, and side-effect metadata.
- [x] 1.2 Support usage forms for variable-only, section-only, and both variable and section lambdas.
- [x] 1.3 Define lambda shape metadata compatible with the context schema model where practical.
- [x] 1.4 Define side-effect metadata states without requiring lambda execution.
- [x] 1.5 Add conversion helpers for deriving parser lambda recognition data from structured definitions.

## 2. Lambda Definition Loading

- [x] 2.1 Update lambda definition loading to parse the structured lambda definition file format.
- [x] 2.2 Preserve existing `--lambdas none` and `[check] lambdas = "none"` behavior.
- [x] 2.3 Preserve syntax-only check behavior when no lambda definition input is supplied.
- [x] 2.4 Emit warning diagnostics for invalid or unrecognisable lambda definition files.
- [x] 2.5 Emit warning diagnostics for malformed lambda definition fields.
- [x] 2.6 Load and retain side-effect metadata without executing lambdas.

## 3. Parser Recognition Integration

- [x] 3.1 Update parser lambda input handling to accept structured lambda/helper specifications.
- [x] 3.2 Recognize configured variable lambdas only when variable usage is allowed.
- [x] 3.3 Recognize configured section lambdas only when section usage is allowed.
- [x] 3.4 Recognize both-usage lambdas in either supported form.
- [x] 3.5 Preserve lambda argument, return, and side-effect metadata for semantic validation.
- [x] 3.6 Preserve unknown lambda-like references as ordinary non-lambda references when no definition exists.

## 4. Semantic Lambda Validation

- [x] 4.1 Validate known variable lambda references against structured definitions.
- [x] 4.2 Validate known section lambda references against structured definitions.
- [x] 4.3 Warn when a known section-only lambda is used as a variable.
- [x] 4.4 Warn when a known variable-only lambda is used as a positive section.
- [x] 4.5 Preserve unmatched names as ordinary non-lambda references.
- [x] 4.6 Emit an error when an inverted section resolves to a known lambda.
- [x] 4.7 Add best-effort return shape compatibility checks for surrounding Mustache usage.
- [x] 4.8 Add best-effort section argument shape compatibility checks where context information is available.
- [x] 4.9 Avoid speculative type compatibility diagnostics when lambda shapes are unknown or omitted.
- [x] 4.10 Preserve side-effect metadata without failing checks solely because side effects are declared.

## 5. Diagnostics and Output

- [x] 5.1 Add or update diagnostic issue kinds for incompatible lambda usage, inverted lambda section, and lambda type incompatibility as needed.
- [x] 5.2 Ensure inverted known-lambda sections are reported as errors.
- [x] 5.3 Ensure incompatible usage and detectable type incompatibility are reported as warnings.
- [x] 5.4 Include useful lambda name, expected usage, actual usage, and shape information in diagnostics.
- [x] 5.5 Ensure check exit status fails when inverted lambda errors are emitted.

## 6. Tests

- [x] 6.1 Add tests for valid structured lambda definition loading.
- [x] 6.2 Add tests for invalid and malformed lambda definition warnings.
- [x] 6.3 Add tests for variable-only, section-only, and both-usage lambda recognition.
- [x] 6.4 Add tests for known variable and section lambda validation.
- [x] 6.5 Add tests for incompatible variable and section lambda usage warnings.
- [x] 6.6 Add tests proving unknown names are not inferred as lambdas.
- [x] 6.7 Add tests for inverted known-lambda section errors and failing check exit status.
- [x] 6.8 Add tests for best-effort argument and return shape compatibility warnings.
- [x] 6.9 Add tests proving unknown or omitted lambda shapes do not emit speculative type warnings.
- [x] 6.10 Add tests for side-effect metadata loading and non-failing default behavior.
- [x] 6.11 Add regression tests for disabled lambda checking and syntax-only check compatibility.

## 7. Validation

- [x] 7.1 Run `cargo fmt --check`.
- [x] 7.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 7.3 Run `cargo nextest run`.
- [x] 7.4 Run `openspec validate refine-lambda-handling --strict`.
- [x] 7.5 Run `openspec validate --specs --strict`.
