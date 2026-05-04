## Context

The project already has Rust integration tests and module-oriented tests under
`tests/`. Those tests are valuable for focused development, but they are not a
complete behavioral conformance suite for the installed command-line utility.
As `smoothe` gains semantic checks, partial graph parsing, structured schema
and lambda models, richer diagnostics, and machine-readable output, the project
needs a black-box fixture suite that treats `smoothe` like an external program.

The fixture suite should exercise behavior through command-line invocation,
configuration files, templates, partials, schemas, lambdas, stdout, stderr, and
exit status. It should not import production Rust modules or rely on parser
internals. The first implementation should provide a custom `cargo behave`
command and may later use libraries such as `trycmd` or `snapbox` internally
where they fit.

The behavioral suite should be deliberately separate from the normal Rust test
suite. It should not be discovered by `cargo nextest run` or by the standard
`cargo test` integration-test path. Developers should run it explicitly through
`cargo behave`.

## Goals / Non-Goals

**Goals:**

- Add black-box CLI behavioral tests for `smoothe`.
- Start with a custom `cargo behave` runner command.
- Keep the behavioral suite opt-in and separate from the normal Rust test
  suite.
- Provide `cargo behave` to run the behavioral suite.
- Discover cases from directories containing `case.toml`, initially matching
  `behavior/fixtures/**/case.toml`.
- Require each fixture case to be self-contained in a directory named after the
  test case.
- Support per-case `smoothe` config files passed explicitly with `--config`
  where applicable.
- Cover partials supplied through explicit config mappings and through
  frontmatter `includes`.
- Keep fixtures input/output driven and independent from implementation
  internals.
- Define a repeatable fixture layout for command inputs, working directories,
  config files, templates, partials, schemas, lambdas, expected stdout,
  expected stderr, and expected exit status.
- Support incremental fixture growth from simple CLI checks to full end-to-end
  semantic validation.
- Normalize output where required, especially filesystem paths and line endings.
- Compare JSON output structurally where the runner supports it or where a thin
  harness layer can provide it.
- Allow intentional expected-output updates.

**Non-Goals:**

- Do not replace focused Rust unit or integration tests.
- Do not expose parser or checker internals to fixture cases.
- Do not require every existing test to be migrated into behavioral fixtures.
- Do not run behavioral fixtures as part of `cargo nextest run`.
- Do not require a Rust integration-test entry point under `tests/`.
- Do not require library integration for the initial runner.
- Do not build a large custom runner before evaluating whether smaller custom
  code or fixture libraries are sufficient.
- Do not make fixture snapshots obscure the reason for failures.

## Decisions

1. Build a custom `cargo behave` runner command first.

   The runner should be exposed as `cargo behave` through a separate
   `cargo-behave` binary. A project-local Cargo alias may invoke that binary
   for contributor convenience, but the shipped `smoothe` command should not
   contain behavioral-test-only subcommands. The runner should own discovery,
   command construction, normalization, comparison, filtering, listing, and
   update behavior. It may use the `trycmd` crate, `snapbox`, or a thin custom
   harness internally, depending on which pieces fit the required fixture
   model.

   Alternative considered: wire `trycmd` directly into a Rust integration test.
   That is simpler initially, but it would run under the normal test suite and
   would not provide the desired standalone command shape.

2. Keep behavioral fixtures separate from current implementation tests.

   Use a dedicated fixture hierarchy outside the normal Rust test discovery
   path. The initial discovery pattern should be
   `behavior/fixtures/**/case.toml`. Each `case.toml` belongs to a directory
   named after the test case, and all fixture-local files are resolved relative
   to that directory.

   Alternative considered: place cases in the existing `tests/fixtures`
   directory. That risks mixing internal fixtures with CLI behavioral fixtures.

3. Run behavioral fixtures through an explicit cargo command.

   The suite should be opt-in because it is a behavioral conformance tool, not
   part of the regular fast test suite. The selected command is `cargo behave`.
   It should support at least listing, filtering, running, and intentionally
   updating expected outputs.

   Alternative considered: register the suite as a standard integration test.
   That is easy to wire, but it violates the requirement that these checks run
   separately from the normal Rust test suite.

4. Model each case as command plus filesystem inputs plus expected outputs.

   Each fixture should define the command invocation, working directory,
   optional environment, expected exit status, expected stdout, and expected
   stderr. Case directories may include `smoothe.toml`, templates, partials,
   schemas, lambda definitions, and expected output files.

   A representative case layout is:

   ```text
   behavior/fixtures/check/frontmatter-partials-json/
     case.toml
     smoothe.toml
     template.mustache
     context.json
     schema.json
     lambdas.json
     partials/
       _header.mustache
       _footer.mustache
     expected.stdout.json
     expected.stderr
   ```

   The `case.toml` should describe how to run `smoothe`, expected status,
   expected stdout/stderr files, and comparison modes. Paths in `case.toml`
   should resolve relative to the case directory.

   Alternative considered: store only command transcript files. That is
   compact, but it can make larger multi-file scenarios harder to read and
   maintain.

5. Use per-case configuration deliberately.

   Behavioral cases should craft config files for their use case and pass them
   explicitly to `smoothe`, normally with `--config`. This keeps behavior
   deterministic and avoids accidental dependency on user, project-root, or
   global configuration. The runner may support a `config = "smoothe.toml"`
   field in `case.toml` that expands into the correct `--config` invocation.

   Environment-variable based config selection may be supported later if
   `smoothe` intentionally supports such a mechanism, but it should not be the
   primary fixture path.

6. Cover partial resolution behavior explicitly.

   Partial behavior should be represented in fixture cases because it is a core
   end-to-end behavior. Cases should cover explicit partial mappings from
   config and frontmatter-driven `includes`. The runner does not need to know
   the semantic mapping rules; it should provide the files and invocation so
   `smoothe` resolves them exactly as it would in normal CLI use. Relative
   configured partial paths are not based on the config file directory; they
   are resolved relative to the template file that includes them, with the
   current working directory as fallback when no template file path is
   available.

7. Normalize paths and line endings before comparison.

   Behavioral output should be stable across local machines and CI. Replace
   workspace-specific absolute paths with stable placeholders where needed and
   normalize line endings. Keep normalization explicit and minimal so it does
   not hide real output differences.

   Alternative considered: require exact raw output. That is simpler but brittle
   for path-heavy diagnostics.

8. Prefer structural JSON comparison for JSON output.

   JSON diagnostics should be compared as JSON values where supported, not raw
   text. This avoids false failures from object key order or whitespace. If
   the initial runner cannot do this directly, evaluate a small wrapper or
   `snapbox` helper before writing a larger custom runner.

   Alternative considered: snapshot JSON as raw text. That works but makes
   formatting details part of the behavior contract.

9. Allow intentional expected-output updates.

   The suite should support a deliberate workflow for updating snapshots or
   expected output files when behavior intentionally changes. This must be
   explicit so accidental output drift still fails tests.

   Alternative considered: regenerate expected output automatically. That would
   weaken the suite’s ability to catch regressions.

10. Build fixture coverage incrementally.

   Start with simple command behavior and diagnostic cases, then add more
   complex scenarios as the engine capabilities land: partials, schema checks,
   lambda checks, improved diagnostics, and machine-readable output.

   Alternative considered: wait until all engine features are complete before
   adding fixtures. That would lose regression coverage while the CLI behavior
   is actively changing.

11. Explore, but do not require, closer library integration.

   The initial suite should treat `smoothe` as a CLI process. It is acceptable
   to investigate whether loading `smoothe` as a library would help with setup,
   fixture generation, or faster execution, but this must not weaken the
   black-box contract. Library integration is optional and should not be needed
   to run the first suite.

## Risks / Trade-offs

- The initial runner may not support the desired fixture layout or JSON
  structural comparison directly. Mitigation: document gaps and move
  selectively to `trycmd`, `snapbox`, or additional custom runner code if
  needed.
- Snapshot tests can become noisy when message text changes. Mitigation: use
  behavioral fixtures for stable user-facing output and keep lower-level tests
  for precise diagnostic data.
- Path normalization can hide real bugs. Mitigation: normalize only known
  machine-specific prefixes and preserve relative paths and line/column data.
- Large fixture trees can become hard to maintain. Mitigation: use clear naming,
  small case directories, and maintenance inventory expectations.
- Behavioral tests can be slower than unit tests. Mitigation: keep the suite
  focused and make it opt-in through a dedicated command.

## Migration Plan

- Add fixture-runner dependencies such as `trycmd` or `snapbox` only if they fit
  the first fixture cases.
- Add a dedicated `cargo behave` runner command.
- Add a dedicated `behavior/fixtures/**/case.toml` fixture directory pattern.
- Create a minimal passing case that runs `smoothe` as a black-box command.
- Add a failing diagnostic case covering stdout, stderr, and exit status.
- Add a config-driven case that passes a fixture-local config using `--config`.
- Add partial cases for explicit config mappings and frontmatter `includes`.
- Add normalization for paths and line endings where needed.
- Add JSON-output fixture comparison for a simple case.
- Add progressively richer fixtures for config, templates, schemas, lambdas,
  partials, diagnostics, and machine-readable output as those features land.
- Document how to add a new fixture and intentionally update expected output.
- Document whether the suite can be run in CI separately from the normal Rust
  test suite.

## Open Questions

- Can the initial runner satisfy structural JSON comparison, or should JSON
  cases use a `snapbox` helper from the start?
- What command should developers use to intentionally refresh expected output?
- Is there value in optional library integration for fixture setup or execution,
  while preserving black-box CLI behavior as the primary contract?
