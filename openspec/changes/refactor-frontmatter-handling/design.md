## Context

The parser currently owns both Mustache parsing and frontmatter extraction. That
keeps frontmatter out of the AST, but it also means file-level concerns such as
raw content, body offsets, starting line numbers, and frontmatter-derived
template inputs are mixed into parser state.

The next layer up already opens template files and stdin inputs before invoking
the parser. That boundary is the better place to produce a content model: raw
data is available there, frontmatter can be extracted once, and additional
inputs such as partial mappings can be derived before Mustache parsing begins.

## Goals / Non-Goals

**Goals:**

- Move frontmatter extraction out of the parser and into a content-processing
  layer.
- Preserve full raw template data while separately identifying the Mustache body
  start offset and start line.
- Let the parser report locations against caller-provided source position
  metadata so diagnostics remain accurate after skipping frontmatter.
- Derive configured partial mappings from a frontmatter `includes` list before
  invoking the parser.
- Keep frontmatter context available on the returned content object along with
  AST and parser state.

**Non-Goals:**

- Do not implement recursive partial expansion beyond the current one-level
  parser behavior.
- Do not execute Mustache lambdas, render templates, or validate all possible
  frontmatter schemas.
- Do not make the parser responsible for resolving framework-specific include
  conventions.

## Decisions

### Add a Template Content Boundary

Introduce a content-level model that represents one opened template input. It
should contain the raw data, frontmatter context, body start byte offset, body
start line number, parser AST, and parser state/diagnostics.

This keeps the lifecycle explicit:

1. Read raw template data.
2. Extract frontmatter and body position metadata.
3. Derive additional parser inputs from frontmatter.
4. Invoke the parser with body/source position metadata.
5. Return one content result containing raw data, metadata, AST, and state.

Alternative considered: keep frontmatter in the parser and add include handling
there. That would work mechanically, but it would expand parser responsibility
from Mustache syntax into file/content conventions and make future frontmatter
extensions harder to isolate.

### Keep the Parser Focused on Mustache Input

Parser input should accept source text and source metadata, including enough
position information to map node spans and diagnostics back to the original
file. The parser should no longer parse frontmatter or expose frontmatter state
as parser state.

The implementation can either pass only the body slice plus an origin offset and
line number, or pass the full source plus the body start position. Passing the
full source keeps byte spans stable against the raw data, while passing a slice
keeps the parser simpler but requires offset adjustment. The implementation
should choose the smaller change, provided diagnostics and node spans remain
anchored to the original source.

Alternative considered: strip frontmatter before parsing without position
metadata. That would simplify parser input, but it would regress diagnostics
because line and column locations would no longer match the source file.

### Parse Frontmatter as Metadata, Not Template Content

The content layer should parse supported frontmatter formats into a context
value and skip that region for Mustache parsing. Frontmatter parse failures
should be reported through the returned content diagnostics/state without
allowing frontmatter text to be parsed as template body.

This preserves the current useful behavior, where frontmatter does not become
AST text, while making frontmatter content available for higher-level behavior.

### Derive Includes-Based Partial Mappings

When frontmatter contains `includes`, the content layer should treat it as an
ordered list of include paths. For each include path:

- The partial key is the basename without its extension.
- The parser path is the include path transformed so the final filename is
  underscore-prefixed.
- The include path itself remains written without the underscore prefix.

For example, `../_partials/path.mustache` as an includes entry derives key
`path` and parser path `../_partials/_path.mustache`. Likewise,
`../_partials/another-path.mustache` derives key `another-path` and parser path
`../_partials/_another-path.mustache`.

This convention belongs in the content layer because it is a project/template
composition rule, not a Mustache grammar rule.

### Merge Caller and Frontmatter Partials Deliberately

Callers can already provide explicit partial mappings. Frontmatter-derived
partials should be added before parsing using a deterministic precedence rule.
Explicit caller mappings should win over frontmatter-derived mappings for the
same key, because explicit API input is the most intentional source.

Alternative considered: make duplicate mappings a hard error. That would catch
ambiguity, but it would also make it harder for callers to override frontmatter
composition during checks or tests.

## Risks / Trade-offs

- Diagnostic drift during refactor -> Add tests that include frontmatter and
  verify line/column output for body diagnostics.
- Ambiguous include shapes -> Accept only list entries that can be interpreted
  as strings and emit warnings for unsupported `includes` values.
- Duplicate partial keys -> Use explicit caller mappings as the override and
  emit a warning for frontmatter duplicates where useful.
- API churn for parser callers -> Keep compatibility helpers where practical and
  migrate CLI/check/parse call sites first.
- Path convention mistakes -> Cover basename extraction and underscore-prefix
  rewriting with focused tests, including already-underscored filenames.
