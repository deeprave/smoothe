## Why

YAML frontmatter is an important supported template metadata path, but the
current `serde_yaml` dependency is deprecated and unmaintained. We need to move
to a maintained YAML parser path without changing the YAML frontmatter behavior
that `smoothe` actually uses today.

## What Changes

- Replace `serde_yaml` with a maintained Serde-compatible YAML dependency,
  preferring `serde_norway` unless a compatibility spike finds a concrete
  blocker.
- Preserve current YAML frontmatter behavior for the supported subset:
  frontmatter objects, string and boolean scalar metadata, and `includes` as a
  sequence of string paths.
- Add a focused compatibility guard test that exercises the supported YAML
  frontmatter shape through `prepare_source`.
- Keep invalid YAML behavior as a warning diagnostic with continued body
  parsing.
- Avoid broadening YAML feature support as part of this dependency upgrade.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `template-content`: YAML frontmatter parsing remains supported while moving
  away from deprecated `serde_yaml`.
- `maintenance`: Dependency maintenance expectations include replacing
  deprecated parser dependencies when a maintained compatible path exists.

## Impact

- Updates `Cargo.toml` and `Cargo.lock`.
- Affects `src/source_prepare.rs`, where YAML frontmatter is parsed and
  converted to `serde_json::Value`.
- Requires targeted frontmatter compatibility coverage plus the standard Rust,
  behavior, and OpenSpec validation commands.
