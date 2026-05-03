# Smoothe

Smoothe is a command-line companion for Mustache templates. It helps you inspect
template structure, catch broken template syntax, and spot common template
contract issues before they reach production.

Use it when you want fast feedback on templates that live in apps, static sites,
email systems, documentation pipelines, generators, or any workflow where
Mustache files are edited by people and rendered somewhere else.

Smoothe does not render templates. It reads Mustache source and reports what it
finds.

## Usage

Run Smoothe with one or more template files:

```sh
smoothe check template.mustache
smoothe parse template.mustache
```

Use `-` to read from standard input:

```sh
cat template.mustache | smoothe check -
```

Multiple inputs are processed in the order provided:

```sh
smoothe check layout.mustache email.mustache partials/header.mustache
```

Smoothe accepts a configuration file named `smoothe.toml` from the current
directory or config home. You can also choose one explicitly:

```sh
smoothe --config smoothe.toml check template.mustache
smoothe -C config/smoothe.toml check template.mustache
```

Color can be controlled globally:

```sh
smoothe --color always check template.mustache
smoothe --no-color check template.mustache
```

## Commands

### `check`

Checks template syntax and correctness. This is the command to use in everyday
template editing and automation.

```sh
smoothe check [OPTIONS] <INPUTS>...
```

Useful options:

- `--schema <PATH|none>` checks template references against a JSON Schema for
  the expected context data.
- `--lambdas <PATH|none>` recognizes configured Mustache lambdas/helpers during
  checking.
- `--json` prints machine-readable diagnostics.
- `--no-json` prints compiler-style diagnostics, which is useful when JSON is
  configured as the default.
- `--verbosity <error|warning|info|debug|trace>` controls how much diagnostic
  detail is displayed.

Compiler-style output is intended for people and editor/terminal workflows. JSON
output is intended for tools, scripts, and CI systems that want structured
diagnostics.

Examples:

```sh
smoothe check email.mustache
smoothe check --schema schema.json email.mustache
smoothe check --lambdas lambdas.json email.mustache
smoothe check --json email.mustache
smoothe check --verbosity info templates/*.mustache
```

### `parse`

Parses templates and prints their AST output. Use this when you want to inspect
how Smoothe understands a template, debug tricky Mustache structure, or compare
template changes in a precise way.

```sh
smoothe parse [OPTIONS] <INPUTS>...
```

Useful options:

- `--json` prints structured parse output.
- `--out <PATH>` writes parse output to a file instead of standard output.

Examples:

```sh
smoothe parse template.mustache
smoothe parse --json template.mustache
smoothe parse --out parsed.txt template.mustache
```

Text output shows the input name, node kinds, names, text values, source spans,
and nested structure. JSON output is better when another tool needs to consume
the parse result.

## Partials And Frontmatter

Smoothe supports static Mustache partials such as `{{> header}}` when they are
mapped to files. Partial mappings can come from configuration or from
frontmatter `includes`.

Example `smoothe.toml`:

```toml
[check.partials]
header = "partials/header.mustache"
footer = "partials/footer.mustache"
```

Configuration partial paths are resolved relative to the configuration file that
declares them. Template frontmatter includes are resolved relative to the
template file that declares the frontmatter.

Frontmatter can declare includes near the template that uses them:

```mustache
---
includes:
  header: partials/header.mustache
---
{{> header}}
Hello {{name}}
```

Smoothe parses reachable static partials as part of the template graph and can
report unresolved, unreadable, nested, and recursive partial references.
When a configured or frontmatter partial path does not already use an
underscore-prefixed filename, Smoothe looks for the underscore-prefixed
filename, such as `partials/_header.mustache` for `partials/header.mustache`.

## Configuration

Configuration is optional. Smoothe looks for `smoothe.toml` in the current
directory, then in the user config location. An explicit `--config` path skips
discovery.

Example:

```toml
[options]
color = "auto"

[check]
schema = "schema.json"
lambdas = "lambdas.json"
output = "compiler"
verbosity = "warning"

[check.partials]
header = "partials/header.mustache"
```

Command-line options override configuration values.

## Expected Output

`check` exits successfully when the requested templates can be checked without
errors. Warnings may still be displayed depending on verbosity.

`parse` exits successfully when the requested templates can be parsed. It prints
a compact tree by default, or JSON when requested.

Both commands exit unsuccessfully for blocking problems such as unreadable input
files, malformed templates, invalid configuration, or unresolved required
template files.

## References

- Mustache manual: https://mustache.github.io/
- Mustache specification: https://github.com/mustache/spec
- Mustache partials overview: https://mustache.github.io/mustache.5.html#Partials
- Mustache lambdas overview: https://mustache.github.io/mustache.5.html#Lambdas
- JSON Schema: https://json-schema.org/
