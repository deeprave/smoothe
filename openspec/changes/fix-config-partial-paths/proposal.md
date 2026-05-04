## Why

`smoothe` currently resolves config-defined partial paths relative to the
process current directory, which makes equivalent invocations produce different
results depending on where they are run.

## What Changes

- Resolve partial paths declared in the configuration file relative to the
  directory containing that configuration file.
- Keep template-frontmatter partial paths relative to the origin template's
  directory.
- Normalize both config-defined and template-frontmatter partial paths by
  prefixing the resolved filename basename with `_` when it is not already
  prefixed.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `configuration`: Configured partial mappings gain config-file-relative path
  semantics.
- `template-partial-graph`: Partial path normalization is defined consistently
  for config-defined and template-frontmatter partials.

## Impact

- Affects configuration loading and the representation of the configuration
  file's source directory.
- Affects partial mapping resolution used by `check` and parse graph handling.
- Requires unit, integration, and behavioral fixture coverage for deterministic
  config-relative partials and underscore normalization.
