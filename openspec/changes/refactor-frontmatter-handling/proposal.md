## Why

Frontmatter handling currently lives inside the Mustache parser, which makes the
parser responsible for file/content concerns that belong at the next layer up.
Moving frontmatter extraction into a content-loading boundary creates a clear
place to derive additional template inputs, especially embedded partial mappings
from frontmatter `includes`.

## What Changes

- Introduce a content-processing layer that owns raw template data,
  frontmatter extraction, content start offsets, content start line numbers,
  parser invocation, and returned parser state.
- Refactor the parser so it parses template content using caller-provided
  source offset and line information instead of extracting frontmatter itself.
- Return a content model containing raw data, parsed frontmatter context,
  template body offset, template body starting line number, AST, and parser
  diagnostics/state.
- Add support for a frontmatter `includes` key containing partial include paths.
- Derive partial mappings from `includes` by using the include basename without
  its extension as the partial key.
- Resolve included partial filesystem paths using the underscore-prefixed file
  convention, where `includes` entries omit the underscore but the path passed
  to the parser includes it.

## Capabilities

### New Capabilities

- `template-content`: Content extraction and parsing for templates, including
  frontmatter context, body offsets, parser state, and frontmatter-derived
  partial mappings.

### Modified Capabilities

None.

## Impact

- Affected code: parser input/result boundaries, frontmatter parsing,
  diagnostic location handling, partial mapping construction, file/content
  loading, check/parse command integration, and tests.
- APIs: parser callers will provide content/body position metadata and partial
  mappings from the higher-level content layer rather than relying on parser
  frontmatter extraction.
- Behavior: frontmatter is skipped for template parsing while remaining
  available as content metadata; `includes` can automatically supply configured
  partial mappings.
