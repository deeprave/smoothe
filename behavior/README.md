# Behavioral Fixtures

Behavioral fixtures exercise `smoothe` as a command-line program. They are not
part of the normal Rust test suite and are not run by `cargo nextest run`.

Run them explicitly:

```sh
cargo behave
```

Useful runner options:

```sh
cargo behave --list
cargo behave --filter parse/
cargo behave check parse/json
cargo behave --update
```

Positional filters are matched against case ids with OR semantics. For example,
`cargo behave check parse/json` runs all `check/...` cases and `parse/json`.
The older `--filter <PATTERN>` option remains available and can be repeated.

Each case lives under `behavior/fixtures/**/case.toml`. The case directory name
is the test case name, and paths in `case.toml` are resolved relative to that
directory.

Example:

```text
behavior/fixtures/check/config-schema-warning/
  case.toml
  smoothe.toml
  schema.json
  template.mustache
  expected.stderr
```

`case.toml` defines the command and expected behavior:

```toml
config = "smoothe.toml"
args = ["check", "template.mustache"]
status = 0
stderr = "expected.stderr"
```

If `config` is set, the runner passes it to `smoothe` with `--config`.
Omitting `stdout` or `stderr` asserts that the corresponding stream is empty.

For `check` cases, fixture-local config may supply explicit partial mappings:

```toml
[check.partials]
header = "partials/header.mustache"
```

Relative partial mapping paths in fixture-local config files are resolved by
`smoothe` relative to the config file directory. Template frontmatter
`includes` remain relative to the template file that declares them.

Set `stdout_format = "json"` or `stderr_format = "json"` to compare a stream as
parsed JSON instead of raw text.

Partial fixtures should provide the same files `smoothe` would see in normal
use. That includes partial mappings from fixture-local config files and
frontmatter `includes`; when a partial path does not already use an
underscore-prefixed basename, `smoothe` resolves it to the underscore-prefixed
filename.

Add a behavioral fixture when the important contract is observable CLI behavior:
arguments, configuration, filesystem inputs, exit status, stdout, stderr, or
stable diagnostics. Prefer lower-level Rust tests when the behavior is internal
to parsing, schema conversion, event routing, or another implementation detail.

Current runner scope:

- status, stdout, and stderr comparison
- line-ending normalization for text output
- case-directory path normalization for text output
- structural JSON comparison
- explicit expected-output refresh with `--update`

Coverage includes both successful command behavior and negative CLI contracts
such as malformed templates, unresolved partials, missing input files, and
malformed explicit configuration.

The runner is intentionally small and CLI-oriented. Optional library integration
was considered for fixture setup and faster in-process execution, but it is not
useful for the current suite because it would weaken the black-box contract and
duplicate command setup that the fixtures need to exercise. Revisit library
integration only for helper generation or metadata inspection that does not
replace invoking the `smoothe` binary.

During cleanup, inventory behavioral fixtures separately from implementation
tests. If a behavioral fixture is removed or consolidated, keep an equivalent
black-box case for the same CLI contract before deleting the old files.
