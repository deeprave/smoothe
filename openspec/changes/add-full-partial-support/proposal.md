## Why

Partials are already accepted as parser input and can be derived from document
frontmatter, but referenced partial templates are not yet resolved into the full
parsed template graph. This leaves parse output and semantic validation
incomplete whenever a template depends on partials.

## What Changes

- Resolve partial tags through the effective key-to-path partial mapping
  provided to the parser.
- Emit diagnostic errors when a partial tag cannot be resolved to an available
  mapping or when the resolved partial file cannot be read.
- Read and parse resolved partial templates as separate Mustache template units
  in the full template graph.
- Include resolved partial template units in the returned AST graph so
  downstream semantic validation sees the complete reachable template graph,
  including included partials, without treating partials as textual
  pre-expansion.
- Preserve partial names, resolved paths, byte offsets, and source line numbers
  for diagnostics and AST/source metadata.
- Detect and skip frontmatter in partial files before parsing their template
  bodies.
- Do not merge partial frontmatter into the parent template frontmatter in this
  implementation.

## Capabilities

### New Capabilities

- `template-partial-graph`: Full partial resolution, separate template-unit
  parsing, AST graph inclusion, and source metadata preservation for templates
  and their partials.

### Modified Capabilities

- `mustache-core-parser`: Partial reference behavior changes from recording
  unresolved reference nodes only to linking mapped partials into the parsed AST
  graph as separately parsed template units while preserving source metadata.
- `mustache-parser-inputs`: Partial mapping requirements change from one-level
  partial parsing into parser state to resolved graph parsing using effective
  mappings.
- `template-content`: Content processing must support partial files with
  frontmatter by detecting and skipping the frontmatter block for the partial
  body without merging it into parent frontmatter.

## Impact

- Affects parser partial resolution, AST shape, parser/content diagnostics, and
  semantic validation inputs.
- Requires file reads for resolved partials during parse/content processing.
- Requires source metadata for each parsed template unit so diagnostics can
  point to the correct template or partial file.
- May require adapting AST output tests and JSON parse output to represent
  linked partial template units.
