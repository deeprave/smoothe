## 1. Diagnostic Model

- [ ] 1.1 Add optional structured detail fields to the diagnostic model for expected values, found values, expectation source, notes, suggestions, and related locations.
- [ ] 1.2 Add typed helper structures for diagnostic suggestions and related source locations.
- [ ] 1.3 Preserve existing diagnostic core fields: severity, issue kind, source name, line, column, span, and message.
- [ ] 1.4 Add helper constructors or builder methods so existing simple diagnostics can be created without verbose detail setup.
- [ ] 1.5 Ensure issue kind string identifiers remain explicit and stable.

## 2. Suggestion Support

- [ ] 2.1 Add a deterministic near-hit suggestion utility for local candidate sets.
- [ ] 2.2 Cap the number of suggestions returned for any diagnostic.
- [ ] 2.3 Suppress suggestions when no reliable local candidate set is available.
- [ ] 2.4 Add focused tests for suggestion ranking, bounding, and no-candidate behavior.

## 3. Source Location Accuracy

- [ ] 3.1 Verify parser diagnostics preserve caller-provided body offset and starting line metadata.
- [ ] 3.2 Preserve source identity and locations for diagnostics emitted from resolved partials.
- [ ] 3.3 Add related-location support for diagnostics that need both an origin and a referring source location.
- [ ] 3.4 Add tests for main-template diagnostics after frontmatter.
- [ ] 3.5 Add tests for diagnostics originating in partial files.

## 4. Schema Diagnostic Enrichment

- [ ] 4.1 Attach known object fields to missing-path diagnostics when the current schema scope is known.
- [ ] 4.2 Attach optionality details to diagnostics for paths that exist but are not required.
- [ ] 4.3 Attach enum allowed values when schema diagnostics involve constrained values.
- [ ] 4.4 Attach found scalar shape details for scalar traversal diagnostics.
- [ ] 4.5 Add near-hit suggestions for missing schema path segments using local object fields.
- [ ] 4.6 Avoid speculative schema suggestions for permissive object scopes.
- [ ] 4.7 Suppress child missing-path cascades when an enclosing section path is missing from the schema.
- [ ] 4.8 Emit at most one secondary warning or note when child references cannot be fully validated because their enclosing section scope is unknown.
- [ ] 4.9 Preserve normal child validation for known object, array, boolean, and lambda section semantics.
- [ ] 4.10 Add schema diagnostic tests for known fields, optional paths, enum context, scalar traversal, suggestions, and unknown-section cascade suppression.

## 5. Lambda Diagnostic Enrichment

- [ ] 5.1 Attach expected and actual usage forms to lambda usage diagnostics.
- [ ] 5.2 Attach argument and return shape context to lambda type-compatibility diagnostics where known.
- [ ] 5.3 Attach side-effect metadata to relevant lambda diagnostics where available.
- [ ] 5.4 Add near-hit suggestions for unknown lambdas using known lambda names.
- [ ] 5.5 Omit lambda suggestions when no nearby known lambda names exist.
- [ ] 5.6 Add lambda diagnostic tests for usage form, shape context, side-effect metadata, inverted lambda sections, and suggestions.

## 6. Partial Diagnostic Enrichment

- [ ] 6.1 Attach partial reference names to unresolved partial diagnostics.
- [ ] 6.2 Attach mapped or resolved paths to unreadable partial diagnostics.
- [ ] 6.3 Attach related locations for partial cycle diagnostics where available.
- [ ] 6.4 Add near-hit suggestions for unresolved partial names using known partial mapping keys.
- [ ] 6.5 Omit partial suggestions when no nearby partial mapping keys exist.
- [ ] 6.6 Add partial diagnostic tests for unresolved partials, unreadable files, cycles, source locations, and suggestions.

## 7. Rendering And JSON Projection

- [ ] 7.1 Update human-readable diagnostic formatting to render useful structured details consistently.
- [ ] 7.2 Ensure text output still includes severity, issue kind, source, line, column, and message.
- [ ] 7.3 Update JSON diagnostic projection to include optional structured detail fields.
- [ ] 7.4 Preserve existing JSON diagnostic grouping into `errors` and `warnings`.
- [ ] 7.5 Ensure empty diagnostic groups are still emitted as empty lists in JSON output.
- [ ] 7.6 Add rendering tests for text diagnostics and JSON diagnostics with structured details.

## 8. Integration Coverage

- [ ] 8.1 Add semantic-check integration tests covering enriched schema diagnostics.
- [ ] 8.2 Add semantic-check integration tests covering enriched lambda diagnostics.
- [ ] 8.3 Add content or parser integration tests covering source-accurate diagnostics across frontmatter and partials.
- [ ] 8.4 Add JSON output tests that assert stable issue identifiers and structured detail fields rather than brittle prose.
- [ ] 8.5 Review existing diagnostic tests and update only the assertions affected by intentional output changes.

## 9. Validation

- [ ] 9.1 Run `cargo fmt --check`.
- [ ] 9.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] 9.3 Run `cargo nextest run`.
- [ ] 9.4 Run `openspec validate improve-diagnostics --strict`.
- [ ] 9.5 Run `openspec validate --specs --strict`.
