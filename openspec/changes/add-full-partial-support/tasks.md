## 1. Source Preparation

- [ ] 1.1 Extract frontmatter/body preparation into a reusable API for primary templates and partial files.
- [ ] 1.2 Preserve existing primary-template content behavior through the reusable source preparation API.
- [ ] 1.3 Add partial source preparation support that skips partial frontmatter and returns partial body offset and body starting line.
- [ ] 1.4 Ensure partial frontmatter is not merged into parent template frontmatter.

## 2. AST Model

- [ ] 2.1 Add an AST graph representation for resolved static partials that preserves the partial name, reference span, resolved path, source metadata, and linked parsed template unit.
- [ ] 2.2 Preserve unresolved static partial references when no resolved content is attached.
- [ ] 2.3 Preserve dynamic partial nodes as runtime references without filesystem resolution.
- [ ] 2.4 Update AST traversal helpers to visit resolved partial template units while preserving source-unit boundaries.
- [ ] 2.5 Ensure section open and close tags are balanced within each parsed template unit, not across partial boundaries.

## 3. Partial Graph Parsing

- [ ] 3.1 Replace one-level partial parsing into parser state with resolved partial graph construction in the returned AST graph.
- [ ] 3.2 Resolve mapped partial paths relative to the source template root.
- [ ] 3.3 Emit error diagnostics for unmapped static partial references.
- [ ] 3.4 Emit error diagnostics for mapped partial files that cannot be read.
- [ ] 3.5 Parse nested mapped partials as separate template units in the same parse operation.
- [ ] 3.6 Detect recursive partial references and link to the existing parsed template unit without expanding indefinitely.
- [ ] 3.7 Preserve partial parser diagnostics using the partial file source name and original line and column.

## 4. Consumers and Output

- [ ] 4.1 Update semantic validation to validate variables, sections, lambdas, and nested partial content inside linked resolved partial template units.
- [ ] 4.2 Update compact parse output to represent resolved partial nodes and linked template units.
- [ ] 4.3 Update JSON parse output to represent resolved partial nodes, resolved paths, source metadata, linked template units, and recursive references.
- [ ] 4.4 Update parse and check exit behavior so unresolved static partial errors fail the command.

## 5. Tests

- [ ] 5.1 Add tests for successful mapped partial resolution and AST inclusion.
- [ ] 5.2 Add tests for unmapped static partial errors.
- [ ] 5.3 Add tests for unreadable mapped partial errors.
- [ ] 5.4 Add tests for nested partial graph parsing.
- [ ] 5.5 Add tests proving recursive partial references are preserved without infinite expansion and are not parse errors.
- [ ] 5.6 Add tests for partial frontmatter skipping and parent frontmatter isolation.
- [ ] 5.7 Add tests for partial diagnostic source path, offset, and line-number preservation.
- [ ] 5.8 Add tests proving dynamic partials remain unresolved runtime references without static-partial errors.
- [ ] 5.9 Add tests proving semantic validation traverses resolved partial template units and bounds recursive traversal.
- [ ] 5.10 Update compact and JSON parse output tests for resolved partial nodes.
- [ ] 5.11 Add tests proving sections cannot open in one template unit and close in a partial, or open in a partial and close in its caller.

## 6. Validation

- [ ] 6.1 Run `cargo fmt --check`.
- [ ] 6.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] 6.3 Run `cargo nextest run`.
- [ ] 6.4 Run `openspec validate add-full-partial-support --strict`.
- [ ] 6.5 Run `openspec validate --specs --strict`.
