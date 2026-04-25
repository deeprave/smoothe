# smoothe

## Purpose

`smoothe` is a proposed separate project for Mustache-compatible template
tooling, implemented in Rust with a focus on speed, correctness, and practical
developer workflows.

The immediate need comes from a gap in existing tooling:

- Mustache syntax/render libraries exist, but checker-quality tooling is weak
- modern, maintained Rust Mustache libraries do not appear to offer a strong
  static-analysis/checking story
- this repo uses Mustache templates with custom injected context and lambda
  helpers, which makes generic syntax-only validation insufficient
- existing validation approaches tend to be:
  - parser/render only
  - markup-focused rather than text-template-focused
  - weak on custom lambdas and injected context

The project should not be built inside `mcp-guide`, but it may later be adopted
by `mcp-guide` once the checking functionality is mature enough.

The name `smoothe` is intentionally a loose nod to `ruff`.

## Intended Use

`smoothe` should be more than a checker.

It should support two primary modes:

1. Check

- parse Mustache templates
- validate syntax and section nesting
- surface useful diagnostics with file/line/column context
- statically inspect referenced variables, sections, partials, and helper names
- validate against known context schemas and allowlists for injected helpers
- support custom repo/framework conventions such as lambda/helper registries

2. Render

- render templates to text files
- accept context values explicitly
- ideally support custom helper/lambda hooks
- potentially allow Python-provided context and/or lambda functions at runtime
  if that proves practical for interoperability

The rendering mode is not intended to replace every existing Mustache runtime,
but to provide a practical CLI/library workflow around checking plus rendering.

## Why This Is Needed

In `mcp-guide`, the template layer is not plain data interpolation. It relies on
custom context injection and lambda helpers. Examples in the current repo
include:

- custom helper/lambda sections such as `resource`, `_error`, command helpers,
  equality helpers, workflow helpers, and formatting helpers
- partial resolution rules
- dotted-path context access
- frontmatter-driven template composition

This means a useful checker must understand more than:

- "is the template syntactically valid?"

It should also answer:

- which names are referenced?
- which of those are plain context variables?
- which are injected helpers/lambdas?
- which partials are required?
- are section names balanced and semantically plausible?
- does the template depend on context that the renderer/checker knows how to
  provide?

## Existing Tools and Prior Research

### General Mustache Ecosystem

- `mustache.js`
  - provides a real parser and renderer
  - useful for syntax parsing and rendering
  - not, by itself, a semantic checker
  - supports Mustache behavior documented in the standard manpage/spec
  - sources:
    - https://github.com/janl/mustache.js/
    - https://mustache.github.io/mustache.5.html

- `mustache-validator`
  - oriented toward validating that referenced properties exist in provided
    data during rendering
  - closer to data-shape validation than full template analysis
  - source:
    - https://socket.dev/npm/package/mustache-validator

- `@markuplint/mustache-parser`
  - parser integration around markup linting
  - more HTML/markup-oriented than general text-template checking
  - source:
    - https://www.skypack.dev/view/@markuplint/mustache-parser

- editor support (for example JetBrains/WebStorm)
  - useful for syntax/highlighting
  - not a project-aware semantic checker
  - source:
    - https://www.jetbrains.com/help/webstorm/using-handlebars-and-mustache-templates.html

### Rust Candidates Investigated

#### `ramhorns`

- strongest maintenance signal among the Rust options reviewed
- recent release activity
- performance-oriented Mustache engine in pure Rust
- good candidate if the goal were fast rendering only
- concerns:
  - public API appears render-oriented rather than checker-oriented
  - no clear evidence found of first-class lambda support suitable for this use
  - no obvious exposed AST/token API aimed at static analysis
- sources:
  - https://github.com/maciejhirsz/ramhorns
  - https://docs.rs/ramhorns
  - https://lib.rs/crates/ramhorns

#### `moostache`

- newer than the older classic Rust Mustache crates
- explicitly documents unsupported features
- supports only a subset of Mustache semantics
- concerns:
  - explicitly does not support lambdas
  - also omits several other advanced features
  - public API appears compile/render oriented, not checker-oriented
- likely unsuitable as the foundation for this project if lambda/helper-aware
  checking matters
- sources:
  - https://docs.rs/moostache
  - https://users.rust-lang.org/t/i-published-my-first-open-source-rust-crate/122778

#### `mustache` (Rust crate)

- appears closer to classic Mustache behavior
- has parse/compile/render APIs
- likely more compatible than newer subset engines
- major concern:
  - old and apparently unmaintained for too long to be a comfortable base for a
    new project
- sources:
  - https://docs.rs/crate/mustache/latest
  - https://github.com/nickel-org/rust-mustache

#### `rustache`

- very old
- not a credible starting point for a new maintained project
- sources:
  - https://github.com/rustache/rustache
  - https://socket.dev/cargo/package/rustache

## Conclusion From Investigation

There does not appear to be a clear, modern, actively maintained Rust library
that simultaneously provides:

- Mustache compatibility suitable for real-world template usage
- static-analysis-friendly parsing / syntax tree access
- strong support for custom lambdas/helpers and injected context
- a good foundation for checker diagnostics

That suggests `smoothe` should likely own its parser/checker model rather than
trying to wrap one of the existing crates too tightly.

## Practical Scope for a Custom Parser

A custom parser appears practical because the checker problem is narrower than
"full generic Mustache engine implementation".

What is likely needed for useful checking:

- text nodes
- escaped variable tags
- unescaped variable tags
- section tags
- inverted section tags
- partial tags
- comments
- dotted names
- section balancing and nesting validation
- source spans for diagnostics

What is likely *not* required for the first useful version:

- full spec-perfect rendering compatibility across every optional Mustache
  feature
- executing arbitrary lambda logic during static checking

This suggests a staged design:

1. Parser

- tokenize Mustache tags
- build a syntax tree / AST
- retain spans for diagnostics

2. Checker

- validate structure
- extract referenced names
- classify names as:
  - ordinary context paths
  - registered helpers/lambdas
  - partial references
- validate against a declared schema/registry

3. Renderer

- render from the parsed template representation
- support plain data context first
- later add helper/lambda integration
- potentially bridge to Python for runtime helper functions if that is
  ergonomically valuable

## Relevance to `mcp-guide`

This project should remain separate from `mcp-guide`.

However, `mcp-guide` provides a good real-world reference workload because its
templates make use of:

- Mustache sections and inverted sections
- partials
- dotted context lookup
- many custom lambda/helper-style injections
- command-oriented and workflow-oriented template conventions

If `smoothe` reaches a useful checker milestone, `mcp-guide` could become an
early adopter for template validation.

## Open Design Questions

- Should `smoothe` target strict Mustache compatibility, or "Mustache plus
  practical checker extensions"?
- Should lambdas/helpers be modeled as:
  - named special forms in the checker
  - runtime plugin hooks
  - both
- What is the cleanest Python interop model for rendering with Python-provided
  helpers?
- Should checking be schema-driven via a configuration file, code API, or both?
- Should partial resolution be customizable enough for repo-specific rules?
- Should the first release focus only on checking, with rendering added after
  the AST/checker architecture stabilizes?
