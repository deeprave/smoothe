## Context

`Configuration` already records `source_dir` and uses it for schema and lambda
paths, but config-defined partial mappings are still converted directly from
their configured strings into `PartialMapping` paths. Those mappings are later
merged with frontmatter-derived mappings and parsed with the origin template
root, causing config partial paths to behave as if they were template-relative
or current-directory-relative rather than config-file-relative.

Frontmatter includes already derive the partial key from the included file stem
and prefix the filename basename with `_` when needed. Config-defined partials
need the same filename normalization while using the config file's directory as
their base path.

## Goals / Non-Goals

**Goals:**

- Make config-defined partial paths deterministic by resolving relative paths
  against the loaded config file's directory.
- Preserve template-frontmatter partial paths as origin-template-relative.
- Use the same underscore filename normalization for config-defined partial
  mappings and template-frontmatter includes.

**Non-Goals:**

- Do not change Mustache partial tag syntax or dynamic partial behavior.
- Do not change config discovery precedence or CLI option precedence.
- Do not change template input operand handling.

## Decisions

1. Resolve config partial paths during configuration option resolution.

   `Configuration::resolve_partial_mappings` should join relative configured
   partial paths to `source_dir` when present, matching existing schema and
   lambda path handling. Absolute configured paths remain absolute. When no
   config file was loaded and `source_dir` is absent, relative paths remain
   relative to the process current directory because there is no config origin.

   Alternative considered: defer config partial path resolution until parsing
   each template. That keeps paths relative for longer, but it mixes config
   origin semantics with template origin semantics and makes the merge between
   explicit and frontmatter mappings harder to reason about.

2. Share partial filename normalization.

   Introduce or expose a small helper that normalizes a partial path by adding
   `_` to the filename basename when the filename does not already start with
   `_`. Frontmatter includes and config partial mappings should both use that
   helper so cases like `partials/header.mustache` resolve to
   `partials/_header.mustache`, while `partials/_header.mustache` is unchanged.

   Alternative considered: normalize only frontmatter-derived paths and require
   config files to spell the exact underscore filename. That preserves current
   config behavior, but it leaves two different partial path languages in the
   same command.

## Risks / Trade-offs

- Config partial path normalization changes behavior for configs that currently
  point at non-underscore filenames -> this is intended to align with the
  project's partial naming convention and frontmatter rules, but tests should
  document the new contract clearly.
