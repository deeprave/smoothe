# Smoothe Configuration

`smoothe` can be configured with a `smoothe.toml` file and provided with optional JSON files for semantic checking of templates, the names of which are provided on the command line.

## `smoothe.toml`

By default, `smoothe` looks for:
1. `./smoothe.toml`
2. `$XDG_CONFIG_HOME/smoothe.toml`
3. `$HOME/.config/smoothe.toml`

Or, you can also pass an explicit configuration file:
```sh
smoothe --config path/to/smoothe.toml check templates/*.mustache
```

### Example configuration file (TOML):
```toml
[options]
color = "auto"

[check]
schema = "schemas/context.json"
lambdas = "schemas/lambdas.json"
output = "compiler"
verbosity = "warning"

[check.partials]
header = "partials/_header.mustache"
footer = "partials/_footer.mustache"
```

> 📝 Command-line options override equivalent configuration-file options.


## Template inputs and glob patterns

`smoothe check` can accept one or more template input paths:

```sh
smoothe check templates/index.mustache templates/about.mustache
```

Glob patterns are supported directly by `smoothe`:

```sh
smoothe check "**/*.mustache"
```

- Input paths and glob patterns are resolved relative to the current working directory.
- If a glob matches no files or not files are given, `smoothe` reports an error.
- Paths defined in `smoothe.toml` however are resolved relative to the directory containing the configuration file.
- Paths defined in the templates themselves in _frontmatter_ are resolved relative to the template from which they are included.


## Global options

Global options are in the `[options]` section.

### color

Controls terminal colour output.

Accepted values: `"auto"` (default), `"always"` (or `true`), `"never"` (or `false`)

CLI flags:
```sh
--color <value>
--no-color
```

> 📝 The `NOCOLOR` environment variable also disables colour output (unless overridden by CLI flags).

## Check options

### schema

Specifies the path to a JSON schema describing the expected template context.

```toml
[check]
schema = "schemas/context.json"
```

- Use `"none"` to disable schema checking (default).
- Path is resolved relative to the configuration file.

CLI override:
```sh
--schema <path|none>
```

### lambdas

Specifies the path to a JSON file describing known Mustache lambda functions.

```toml
[check]
lambdas = "schemas/lambdas.json"
```

- Use `"none"` to disable lambda checking (default).
- Path is resolved relative to the configuration file.

CLI override:
```sh
--lambdas <path|none>
```

### output

Specifies output format for `smoothe check`.

| Style      | Description |
|------------|-------------|
| `compiler` | Classic compiler-style single-line output: `source:line:column:severity:issue:message`, optionally followed by context (expected vs found, notes, suggestions). |
| `json`     | Structured machine-readable output with `has_error`, grouped diagnostics, optional events, and per-input results. |

CLI overrides:
```sh
--json
--no-json
```

### verbosity

Controls diagnostic verbosity.

| Value     | Description |
|-----------|-------------|
| `error`   | Only errors |
| `warning` | Warnings and errors |
| `info`    | Informational, warnings, and errors |
| `debug`   | Detailed progress plus above |
| `trace`   | Highly detailed plus all above |


### partials

Maps partial names to template files.

```toml
[check.partials]
header = "partials/_header.mustache"
invoice_row = "partials/_invoice-row.mustache"
```

- Paths are resolved relative to the configuration file.

Frontmatter includes:

```yaml
---
includes:
  - ../partials/header.mustache
  - ../partials/footer.mustache
---
```

- Frontmatter paths are resolved relative to the template.
- Partial filenames begin with `_`, although configuration files and frontmatter should omit it.

## Context JSON Schema

Defines the data available to templates.

Supported features:

| Keyword               | Description |
|-----------------------|-------------|
| `type`                | object, array, string, number, integer, boolean, null |
| `properties`          | Object fields |
| `required`            | Required fields |
| `additionalProperties`| Boolean |
| `items`               | Array item type |
| `enum`                | Allowed values |
| `default`             | Default values |

Unsupported features (generate warnings):

- `$ref`
- `$defs`
- `definitions`
- `oneOf`
- `anyOf`
- `allOf`
- `patternProperties`
- schema-valued `additionalProperties`


## Lambda definitions JSON

Structure:

```json
{
  "lambdas": {
    "markdown": {
      "usage": "section",
      "argument": { "type": "string" },
      "returns": { "type": "string" },
      "side_effects": "none"
    }
  }
}
```

### Fields

| Field          | Required | Description |
|----------------|----------|-------------|
| usage          | Yes      | variable, section, or both |
| argument       | No       | Input shape |
| returns        | No       | Output shape |
| side_effects   | No       | none, declared, unknown |

