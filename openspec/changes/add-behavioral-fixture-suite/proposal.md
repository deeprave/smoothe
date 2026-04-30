## Why

As `smoothe` grows into a semantic checking utility, unit and integration tests
are not enough to prove that the command-line behavior remains consistent with
the intended user experience. The project needs a black-box behavioral fixture
suite that runs the built utility against real templates, configs, schemas,
lambdas, partials, and expected outputs.

## What Changes

- Add a behavioral fixture suite for the `smoothe` CLI.
- Start with a custom `cargo behave` command and evaluate whether libraries
  such as `trycmd` or `snapbox` are useful internally.
- Keep the behavioral suite separate from the normal Rust test suite; it shall
  not run as part of `cargo nextest run`.
- Provide an explicit `cargo behave` command to run the behavioral suite on
  demand.
- Keep behavioral tests black-box: run the utility through command-line options
  rather than importing internal Rust modules.
- Explore closer integration with `smoothe`, including whether loading the
  project as a library would be useful, but do not require this for the initial
  suite.
- Define a fixture layout where each test case lives in a directory named after
  the case and contains a `case.toml`, config files, templates, schemas,
  lambdas, partials, expected stdout, expected stderr, and expected exit codes.
- Discover fixture cases by filesystem pattern, initially
  `behavior/fixtures/**/case.toml`.
- Run cases with their own purpose-built config files, passed explicitly to
  `smoothe` with `--config` where applicable.
- Cover partial templates supplied by explicit config mappings and by template
  frontmatter `includes`.
- Cover behavioral consistency, functionality, and compliance with intended
  command behavior.
- Support output normalization where needed, especially paths and line endings.
- Compare JSON output structurally where possible rather than only as raw text.
- Compare compiler-style diagnostic output as text with path normalization.
- Allow expected output to be updated intentionally as behavior evolves.
- Build test cases incrementally from simple CLI behavior to full end-to-end
  checks across templates, partials, schemas, lambdas, and diagnostics.

## Capabilities

### New Capabilities

- `behavioral-fixture-suite`: Black-box CLI fixture format and runner for
  exercising `smoothe` behavior through command-line inputs and expected
  outputs.

### Modified Capabilities

- `maintenance`: Extend maintenance expectations to include behavioral fixture
  inventory, curation, and preservation of black-box CLI coverage.

## Impact

- Adds a dev/test dependency or runner integration, starting with a custom
  `cargo behave` command.
- Adds behavioral fixture files and an opt-in behavioral runner command.
- Does not require test fixtures to know about parser or checker internals.
- Does not add a `tests/` entry point that runs with the normal Rust test suite.
- May later adopt `trycmd`, `snapbox`, or additional custom runner code if the
  initial implementation cannot support the required fixture layout,
  normalization, or structured comparison.
