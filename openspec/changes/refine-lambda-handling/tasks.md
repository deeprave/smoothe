## 1. Lambda Model

- [ ] 1.1 Define a structured lambda definition model with name, usage forms, argument shape, return shape, and side-effect metadata.
- [ ] 1.2 Support usage forms for variable-only, section-only, and both variable and section lambdas.
- [ ] 1.3 Define lambda shape metadata compatible with the context schema model where practical.
- [ ] 1.4 Define side-effect metadata states without requiring lambda execution.
- [ ] 1.5 Add conversion helpers for deriving parser lambda recognition data from structured definitions.

## 2. Lambda Definition Loading

- [ ] 2.1 Update lambda definition loading to parse the structured lambda definition file format.
- [ ] 2.2 Preserve existing `--lambdas none` and `[check] lambdas = "none"` behavior.
- [ ] 2.3 Preserve syntax-only check behavior when no lambda definition input is supplied.
- [ ] 2.4 Emit warning diagnostics for invalid or unrecognisable lambda definition files.
- [ ] 2.5 Emit warning diagnostics for malformed lambda definition fields.
- [ ] 2.6 Load and retain side-effect metadata without executing lambdas.

## 3. Parser Recognition Integration

- [ ] 3.1 Update parser lambda input handling to accept structured lambda/helper specifications.
- [ ] 3.2 Recognize configured variable lambdas only when variable usage is allowed.
- [ ] 3.3 Recognize configured section lambdas only when section usage is allowed.
- [ ] 3.4 Recognize both-usage lambdas in either supported form.
- [ ] 3.5 Preserve lambda argument, return, and side-effect metadata for semantic validation.
- [ ] 3.6 Preserve unknown lambda-like references as ordinary non-lambda references when no definition exists.

## 4. Semantic Lambda Validation

- [ ] 4.1 Validate known variable lambda references against structured definitions.
- [ ] 4.2 Validate known section lambda references against structured definitions.
- [ ] 4.3 Warn when a known section-only lambda is used as a variable.
- [ ] 4.4 Warn when a known variable-only lambda is used as a positive section.
- [ ] 4.5 Warn when an identifiable lambda reference has no matching definition.
- [ ] 4.6 Emit an error when an inverted section resolves to a known lambda.
- [ ] 4.7 Add best-effort return shape compatibility checks for surrounding Mustache usage.
- [ ] 4.8 Add best-effort section argument shape compatibility checks where context information is available.
- [ ] 4.9 Avoid speculative type compatibility diagnostics when lambda shapes are unknown or omitted.
- [ ] 4.10 Preserve side-effect metadata without failing checks solely because side effects are declared.

## 5. Diagnostics and Output

- [ ] 5.1 Add or update diagnostic issue kinds for unknown lambda, incompatible lambda usage, inverted lambda section, and lambda type incompatibility as needed.
- [ ] 5.2 Ensure inverted known-lambda sections are reported as errors.
- [ ] 5.3 Ensure incompatible usage and detectable type incompatibility are reported as warnings.
- [ ] 5.4 Include useful lambda name, expected usage, actual usage, and shape information in diagnostics.
- [ ] 5.5 Ensure check exit status fails when inverted lambda errors are emitted.

## 6. Tests

- [ ] 6.1 Add tests for valid structured lambda definition loading.
- [ ] 6.2 Add tests for invalid and malformed lambda definition warnings.
- [ ] 6.3 Add tests for variable-only, section-only, and both-usage lambda recognition.
- [ ] 6.4 Add tests for known variable and section lambda validation.
- [ ] 6.5 Add tests for incompatible variable and section lambda usage warnings.
- [ ] 6.6 Add tests for unknown identifiable lambda warnings.
- [ ] 6.7 Add tests for inverted known-lambda section errors and failing check exit status.
- [ ] 6.8 Add tests for best-effort argument and return shape compatibility warnings.
- [ ] 6.9 Add tests proving unknown or omitted lambda shapes do not emit speculative type warnings.
- [ ] 6.10 Add tests for side-effect metadata loading and non-failing default behavior.
- [ ] 6.11 Add regression tests for disabled lambda checking and syntax-only check compatibility.

## 7. Validation

- [ ] 7.1 Run `cargo fmt --check`.
- [ ] 7.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] 7.3 Run `cargo nextest run`.
- [ ] 7.4 Run `openspec validate refine-lambda-handling --strict`.
- [ ] 7.5 Run `openspec validate --specs --strict`.
