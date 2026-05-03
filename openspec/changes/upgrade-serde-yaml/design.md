## Context

`src/source_prepare.rs` currently parses YAML frontmatter through
`serde_yaml::from_str` into `serde_yaml::Value`, then converts that value into
`serde_json::Value`. The rest of content processing only relies on a small YAML
subset: a mapping object, simple metadata scalars, and `includes` as a list of
string paths.

`serde_yaml` is deprecated. YAML frontmatter remains important, so the upgrade
needs to preserve behavior rather than remove YAML or force users onto JSON or
TOML.

## Goals / Non-Goals

**Goals:**

- Replace deprecated `serde_yaml` with a maintained Serde-compatible YAML crate.
- Preserve the YAML frontmatter behavior that is currently used by templates.
- Add a focused compatibility test before swapping the dependency.
- Keep frontmatter parse failures as warning diagnostics.

**Non-Goals:**

- Do not add support for new YAML features such as anchors, custom tags, or
  complex merge behavior.
- Do not change frontmatter format detection.
- Do not change JSON or TOML frontmatter parsing.
- Do not change the public shape of `FrontmatterState`.

## Decisions

1. Prefer `serde_norway` for the replacement crate.

   `serde_norway` is a maintained fork path intended to preserve the
   `serde_yaml`-style Serde API while moving away from the deprecated crate.
   The migration should be limited to the YAML parsing branch in
   `parse_frontmatter_value`.

   Alternative considered: `serde_yaml_ng`. It is another maintained fork path,
   but it still tracks the `unsafe-libyaml` lineage more directly. It remains a
   fallback if `serde_norway` has a concrete compatibility blocker for the YAML
   subset `smoothe` uses.

2. Test the supported YAML subset, not the whole YAML language.

   The compatibility test should assert a representative `serde_json::Value`
   produced from YAML frontmatter containing ordinary metadata plus `includes`.
   Existing tests already cover simple strings, booleans, include lists, invalid
   YAML warnings, and delimiter handling; the new test should make the
   dependency swap safer without becoming a broad YAML conformance suite.

   Alternative considered: add extensive YAML feature coverage. That would be
   speculative because the project does not currently depend on those features.

3. Keep diagnostics stable at the contract level.

   Tests should continue to assert warning severity, issue kind, source
   location, and continued body parsing for invalid YAML. Exact parser error
   strings may change with the dependency and should not become the contract
   unless users rely on them directly.

## Risks / Trade-offs

- [Risk] `serde_norway` may format parse errors differently.
  -> Mitigation: keep tests focused on diagnostic kind/location and continued
  parsing rather than exact low-level parser text.
- [Risk] A maintained replacement may differ on YAML edge cases.
  -> Mitigation: only claim compatibility for the YAML subset currently used by
  `smoothe`; do not broaden support in this change.
- [Risk] Dependency replacement can shift transitive dependency versions.
  -> Mitigation: run the full validation suite and inspect `Cargo.lock` changes
  before accepting the migration.
