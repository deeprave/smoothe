## 1. Content Model

- [x] 1.1 Add a content-processing module for template content results.
- [x] 1.2 Define a content result type containing raw data, frontmatter context,
  body offset, body starting line number, AST, and parser state.
- [x] 1.3 Move frontmatter format/context state out of parser state and into the
  content result model.
- [x] 1.4 Add diagnostics support for content-level warnings such as
  frontmatter parse failures and invalid `includes` values.

## 2. Frontmatter Extraction

- [x] 2.1 Move YAML/JSON/TOML frontmatter detection and parsing out of the parser
  into the content-processing layer.
- [x] 2.2 Compute the body byte offset and body starting line number for inputs
  with and without frontmatter.
- [x] 2.3 Ensure invalid frontmatter emits a warning without allowing
  frontmatter text to be parsed as template body.
- [x] 2.4 Preserve parsed frontmatter context on the content result.

## 3. Parser Boundary Refactor

- [x] 3.1 Extend parser source metadata to accept caller-provided body offset and
  starting line information.
- [x] 3.2 Update parser location/span handling so diagnostics after frontmatter
  are reported against the original raw input positions.
- [x] 3.3 Remove parser-owned frontmatter options and frontmatter state.
- [x] 3.4 Update existing parser callers to pass content/body metadata through
  the new boundary.

## 4. Includes-Derived Partials

- [x] 4.1 Parse frontmatter `includes` as a list of string paths.
- [x] 4.2 Derive each partial key from the include path basename without its
  extension.
- [x] 4.3 Rewrite include paths so the final filesystem filename is
  underscore-prefixed before passing mappings to the parser.
- [x] 4.4 Avoid duplicating an existing underscore prefix on include filenames.
- [x] 4.5 Merge frontmatter-derived partial mappings with caller-provided
  mappings, with caller-provided mappings taking precedence.
- [x] 4.6 Emit warnings for unsupported `includes` shapes while preserving valid
  entries and continuing template parsing.

## 5. Command Integration

- [x] 5.1 Update template file/stdin reading to produce content results through
  the content-processing layer.
- [x] 5.2 Update `check` to report content-level and parser diagnostics from the
  returned content result.
- [x] 5.3 Update `parse` output to use the content result AST/state after
  frontmatter extraction.

## 6. Verification

- [x] 6.1 Add integration tests under `tests/` for content results with and
  without frontmatter.
- [x] 6.2 Add tests under `tests/` proving frontmatter is skipped from AST
  parsing but preserved as content metadata.
- [x] 6.3 Add tests under `tests/` for diagnostic line/column accuracy after
  frontmatter.
- [x] 6.4 Add tests under `tests/` for `includes` key derivation and
  underscore-prefixed path rewriting.
- [x] 6.5 Add tests under `tests/` for explicit partial mappings overriding
  frontmatter-derived mappings.
- [x] 6.6 Add tests under `tests/` for invalid frontmatter and unsupported
  `includes` diagnostics.
- [x] 6.7 Run `cargo fmt --check`.
- [x] 6.8 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 6.9 Run `cargo nextest run`.
