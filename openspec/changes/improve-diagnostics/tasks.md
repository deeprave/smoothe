## 1. Diagnostic Model

- [x] 1.1 Add optional structured detail fields to the diagnostic model for expected values, found values, expectation source, notes, suggestions, and related locations.
- [x] 1.2 Add typed helper structures for diagnostic suggestions and related source locations.
- [x] 1.3 Preserve existing diagnostic core fields: severity, issue kind, source name, line, column, span, and message.
- [x] 1.4 Add helper constructors or builder methods so existing simple diagnostics can be created without verbose detail setup.
- [x] 1.5 Ensure issue kind string identifiers remain explicit and stable.

## 2. Suggestion Support

- [x] 2.1 Add a deterministic near-hit suggestion utility for local candidate sets.
- [x] 2.2 Cap the number of suggestions returned for any diagnostic.
- [x] 2.3 Suppress suggestions when no reliable local candidate set is available.
- [x] 2.4 Add focused tests for suggestion ranking, bounding, and no-candidate behavior.

## 3. Source Location Accuracy

- [x] 3.1 Verify parser diagnostics preserve caller-provided body offset and starting line metadata.
- [x] 3.2 Preserve source identity and locations for diagnostics emitted from resolved partials.
- [x] 3.3 Add related-location support for diagnostics that need both an origin and a referring source location.
- [x] 3.4 Add tests for main-template diagnostics after frontmatter.
- [x] 3.5 Add tests for diagnostics originating in partial files.

## 4. Schema Diagnostic Enrichment

- [x] 4.1 Attach known object fields to missing-path diagnostics when the current schema scope is known.
- [x] 4.2 Attach optionality details to diagnostics for paths that exist but are not required.
- [x] 4.3 Attach enum allowed values when schema diagnostics involve constrained values.
- [x] 4.4 Attach found scalar shape details for scalar traversal diagnostics.
- [x] 4.5 Add near-hit suggestions for missing schema path segments using local object fields.
- [x] 4.6 Avoid speculative schema suggestions for permissive object scopes.
- [x] 4.7 Suppress child missing-path cascades when an enclosing section path is missing from the schema.
- [x] 4.8 Emit at most one secondary warning or note when child references cannot be fully validated because their enclosing section scope is unknown.
- [x] 4.9 Preserve normal child validation for known object, array, boolean, and lambda section semantics.
- [x] 4.10 Add schema diagnostic tests for known fields, optional paths, enum context, scalar traversal, suggestions, and unknown-section cascade suppression.

## 5. Lambda Diagnostic Enrichment

- [x] 5.1 Attach expected and actual usage forms to lambda usage diagnostics.
- [x] 5.2 Attach argument and return shape context to lambda type-compatibility diagnostics where known.
- [x] 5.3 Attach side-effect metadata to relevant lambda diagnostics where available.
- [x] 5.4 Omit speculative lambda suggestions when ordinary Mustache syntax cannot identify unknown lambdas.
- [x] 5.5 Preserve known lambda diagnostic context without adding speculative name suggestions.
- [x] 5.6 Add lambda diagnostic tests for usage form, shape context, side-effect metadata, inverted lambda sections, and suggestion omission.

## 6. Partial Diagnostic Enrichment

- [x] 6.1 Attach partial reference names to unresolved partial diagnostics.
- [x] 6.2 Attach mapped or resolved paths to unreadable partial diagnostics.
- [x] 6.3 Preserve recursive partial references without emitting speculative cycle diagnostics.
- [x] 6.4 Add near-hit suggestions for unresolved partial names using known partial mapping keys.
- [x] 6.5 Omit partial suggestions when no nearby partial mapping keys exist.
- [x] 6.6 Add partial diagnostic tests for unresolved partials, unreadable files, recursive references, source locations, and suggestions.

## 7. Rendering And JSON Projection

- [x] 7.1 Update human-readable diagnostic formatting to render useful structured details consistently.
- [x] 7.2 Ensure text output still includes severity, issue kind, source, line, column, and message.
- [x] 7.3 Update JSON diagnostic projection to include optional structured detail fields.
- [x] 7.4 Preserve existing JSON diagnostic grouping into `errors` and `warnings`.
- [x] 7.5 Ensure empty diagnostic groups are still emitted as empty lists in JSON output.
- [x] 7.6 Add rendering tests for text diagnostics and JSON diagnostics with structured details.

## 8. Integration Coverage

- [x] 8.1 Add semantic-check integration tests covering enriched schema diagnostics.
- [x] 8.2 Add semantic-check integration tests covering enriched lambda diagnostics.
- [x] 8.3 Add content or parser integration tests covering source-accurate diagnostics across frontmatter and partials.
- [x] 8.4 Add JSON output tests that assert stable issue identifiers and structured detail fields rather than brittle prose.
- [x] 8.5 Review existing diagnostic tests and update only the assertions affected by intentional output changes.

## 9. Validation

- [x] 9.1 Run `cargo fmt --check`.
- [x] 9.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 9.3 Run `cargo nextest run`.
- [x] 9.4 Run `openspec validate improve-diagnostics --strict`.
- [x] 9.5 Run `openspec validate --specs --strict`.
