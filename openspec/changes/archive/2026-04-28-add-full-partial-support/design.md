## Context

The content layer currently extracts frontmatter from the primary template,
derives partial mappings from `includes`, passes those mappings to the parser,
and returns the parser AST and state. The parser already accepts partial
mappings and can read mapped partial files, but it records parsed partials in
parser state and only resolves one level. Partial ASTs are not included in the
returned AST graph, nested partials are not resolved end to end, and partial
frontmatter is not skipped before parsing.

Semantic validation currently walks the primary AST. For validation to be
complete, the parse result needs to represent the full template graph: the
primary template unit plus separately parsed resolved partial template units,
with enough source metadata to report diagnostics against the correct file and
original line numbers.

Mustache partials are runtime template inclusions, not literal textual
preprocessing before parse. Each partial remains its own template unit, so
section tags must balance within the same source unit. A section opened in one
template cannot be closed by a partial, and a section opened in a partial cannot
be closed by its caller.

## Goals / Non-Goals

**Goals:**

- Resolve static partial tags through the effective partial mapping available
  to the parser.
- Report diagnostic errors when a static partial reference is unmapped or when
  the mapped file cannot be read.
- Parse resolved partial files as separate template units so nested partials
  are represented in the reachable template graph.
- Include resolved partial template units in the returned AST graph, not only in
  parser state.
- Preserve each partial reference name, resolved path, source byte offsets, and
  source line numbers for diagnostics and output.
- Detect and skip frontmatter in partial files before parsing their template
  bodies.
- Keep partial frontmatter local to the partial load; do not merge it into the
  parent template frontmatter.
- Represent recursive partial references without infinite expansion, and ensure
  graph traversal bounds recursion.

**Non-Goals:**

- Do not render templates or evaluate context values.
- Do not resolve dynamic partial tags whose target cannot be known at parse
  time.
- Do not merge partial frontmatter into parent frontmatter.
- Do not make semantic validation responsible for loading partial files.
- Do not introduce a general template runtime or cache beyond what parsing
  requires.

## Decisions

1. Represent resolved partials directly in the AST graph.

   Add an AST representation for a resolved static partial that keeps the
   original partial tag source span and a link to the separately parsed partial
   template unit. This can be implemented by extending `Node::Partial` with
   resolved metadata and a template-unit reference, or by adding a distinct
   resolved partial node. The important contract is that AST walkers can
   traverse from the partial tag into the linked partial template unit while
   preserving the source-unit boundary.

   Alternative considered: keep parsed partial ASTs only in `ParserState`.
   That preserves today’s model but forces every consumer, including semantic
   validation and JSON output, to manually join the primary AST with parser
   state.

2. Parse the complete reachable partial graph during parsing.

   The parser should resolve mapped static partials using the same effective
   partial mapping set. The existing one-level behavior should be replaced with
   graph construction so nested partials are parsed in the same parse operation
   and linked from the referencing partial node.

   Alternative considered: resolve only one level and leave nested partials as
   references. That leaves validation incomplete for realistic templates.

3. Do not textually concatenate partials before parsing.

   Each template and partial should be parsed independently. This matches
   Mustache semantics, where partials are rendered at runtime and inherit the
   caller context but are not a pre-parse textual include. This means section
   open and close tags must be balanced inside a single source unit.

   Alternative considered: concatenate resolved partial text into the caller
   before parsing. That would allow cross-file section balancing, which is not
   correct Mustache behavior and would make source metadata harder to preserve.

4. Move source preparation into a reusable boundary below content processing.

   Frontmatter extraction, body offset calculation, and body starting line
   calculation should be reusable for both the primary template and loaded
   partial files. The content layer can continue to expose the primary
   `TemplateContent` frontmatter, while the parser’s partial loader uses the
   same preparation logic to skip partial frontmatter and pass body metadata to
   nested parses.

   Alternative considered: duplicate frontmatter skipping inside the parser.
   That risks diverging line-number behavior and reintroduces frontmatter logic
   into the tokenizer path.

5. Keep parent and partial frontmatter contexts separate.

   The primary template’s frontmatter remains the frontmatter context returned
   by the content result. Partial frontmatter is used only to find the partial
   body offset and starting line, and later may be exposed as source metadata if
   needed. It is not merged into the parent context or used to derive additional
   includes in this first implementation unless the parser receives explicit
   mappings that cover those partial references.

   Alternative considered: merge partial frontmatter into parent frontmatter or
   derive new partial mappings from partial frontmatter. That creates unclear
   precedence rules and expands the scope beyond full graph inclusion.

6. Treat unresolved static partials as parse errors.

   A static partial tag that cannot be mapped or whose mapped file cannot be
   read means the returned AST is incomplete. These diagnostics should be
   errors so `parse` and `check` can fail when the reachable template graph
   cannot be built.

   Alternative considered: continue warning and return a partial AST. That
   preserves loose behavior but undermines the usefulness of full semantic
   checking.

7. Preserve dynamic partials as unresolved runtime references.

   Dynamic partial tags cannot be resolved safely without runtime context. The
   parser should keep them as dynamic partial nodes and may record them in state,
   but should not emit an unresolved-static-partial error for them.

   Alternative considered: warn for all dynamic partials. That would be noisy
   for valid runtime-driven templates.

8. Preserve recursive partial graphs without infinite expansion.

   Recursive partials are valid Mustache. The partial resolver should detect
   when a mapped partial is already present in the active resolution stack and
   record a recursive reference to the existing template unit instead of
   expanding the branch again. Validation and output traversal must also track
   visited source units to avoid infinite recursion.

   Alternative considered: emit a parse error for cycles. That prevents runaway
   recursion but rejects valid recursive partials.

## Risks / Trade-offs

- AST graph shape changes may affect compact and JSON parse output. Mitigation:
  update output formatting and compatibility tests together with the AST graph
  model.
- Recursive partial parsing can repeatedly read the same file. Mitigation:
  keep a simple per-parse cache once correctness is established, or defer cache
  work until profiling shows a need.
- Recursive references and repeated includes can make traversal noisy.
  Mitigation: preserve recursive references in the graph and bound traversal by
  source unit identity.
- Source spans across multiple files require care because byte offsets are local
  to each file. Mitigation: store source identity with resolved partial nodes
  and continue using each source’s own body offset and starting line for
  diagnostics.
- Moving source preparation can disturb existing frontmatter behavior.
  Mitigation: preserve current primary-template tests and add partial
  frontmatter tests before changing parser expansion.

## Migration Plan

- Extract or move frontmatter/body preparation into a reusable module used by
  both content processing and partial loading.
- Update the AST graph model to represent resolved partials with linked
  template units and source metadata.
- Replace one-level parser partial handling with resolved graph construction
  and recursive-reference preservation.
- Change unresolved static partial diagnostics from warnings to errors.
- Update semantic validation to traverse resolved partial template units while
  preventing infinite recursion.
- Update compact and JSON parse output for resolved partial nodes.
- Add tests for mapped partials, missing mappings, unreadable files, nested
  partials, partial frontmatter skipping, line-number preservation, dynamic
  partial preservation, recursive partial preservation, and source-unit section
  balancing.

## Open Questions

- Should repeated references to the same partial share one parsed template unit
  internally, or should each reference own its own parsed unit instance?
- Should partial frontmatter-derived `includes` be deliberately ignored for now,
  or should it be used to extend mappings for that partial subtree?
- What exact JSON representation should resolved partial nodes use in
  `parse --json` output?
