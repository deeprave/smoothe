## 1. Options and Configuration

- [ ] 1.1 Add check output format option types for compiler-style and JSON output.
- [ ] 1.2 Add check diagnostic level option types for error, warning, info, and debug display thresholds.
- [ ] 1.3 Add `--json` to the check command as a JSON output shortcut.
- [ ] 1.4 Add `--format <compiler|json>` to the check command.
- [ ] 1.5 Reject conflicting check output selections such as `--json --format compiler`.
- [ ] 1.6 Add `--diagnostic-level <error|warning|info|debug>` to the check command.
- [ ] 1.7 Add `[check] output` configuration support.
- [ ] 1.8 Add `[check] diagnostic_level` configuration support.
- [ ] 1.9 Resolve check output and diagnostic-level options with CLI values overriding configuration defaults.

## 2. Diagnostic Collection

- [ ] 2.1 Refactor check command execution to collect schema, lambda, content, parser, and semantic diagnostics before formatting.
- [ ] 2.2 Preserve the unfiltered diagnostic set for exit-status calculation.
- [ ] 2.3 Preserve current check validation behavior while removing direct diagnostic `eprintln!` calls from validation flow.
- [ ] 2.4 Ensure fatal setup errors that happen before check results exist still report clearly.

## 3. Formatter Abstraction

- [ ] 3.1 Define a check diagnostic formatter interface or enum dispatch pattern.
- [ ] 3.2 Pass structured diagnostic data to the selected formatter.
- [ ] 3.3 Apply diagnostic-level filtering only in the formatter layer.
- [ ] 3.4 Ensure formatter selection does not mutate diagnostics or affect exit status.
- [ ] 3.5 Keep formatter code extensible for future output formats.

## 4. Compiler-Style Output

- [ ] 4.1 Implement compiler-style output as the default check formatter.
- [ ] 4.2 Format diagnostics as source, line, column, severity, issue kind, and message.
- [ ] 4.3 Include error, warning, info, and debug severity labels.
- [ ] 4.4 Write compiler-style diagnostics to stderr.
- [ ] 4.5 Preserve accurate main-template source locations in compiler-style output.
- [ ] 4.6 Preserve accurate partial source locations in compiler-style output.

## 5. JSON Output

- [ ] 5.1 Implement JSON check output selected by `--json` or `--format json`.
- [ ] 5.2 Emit one valid JSON document to stdout.
- [ ] 5.3 Include an `inputs` list with one result object per checked input.
- [ ] 5.4 Group diagnostics into `errors`, `warnings`, `info`, and `debug` lists.
- [ ] 5.5 Include issue kind, severity, source name, line, column, span, and message for each JSON diagnostic.
- [ ] 5.6 Include optional structured diagnostic details when present.
- [ ] 5.7 Preserve accurate main-template source locations in JSON output.
- [ ] 5.8 Preserve accurate partial source locations in JSON output.

## 6. Severity Filtering

- [ ] 6.1 Default diagnostic display to warnings and errors.
- [ ] 6.2 Implement error-only diagnostic display.
- [ ] 6.3 Implement info-level diagnostic display.
- [ ] 6.4 Implement debug-level diagnostic display.
- [ ] 6.5 Ensure filtered-out error diagnostics still cause an unsuccessful check exit status.
- [ ] 6.6 Ensure JSON output applies the selected display filter consistently.

## 7. Tests

- [ ] 7.1 Add CLI tests for `check --json`.
- [ ] 7.2 Add CLI tests for `check --format compiler` and `check --format json`.
- [ ] 7.3 Add CLI tests for conflicting output options.
- [ ] 7.4 Add CLI tests for valid and invalid `--diagnostic-level` values.
- [ ] 7.5 Add configuration tests for `[check] output`.
- [ ] 7.6 Add configuration tests for `[check] diagnostic_level`.
- [ ] 7.7 Add tests proving CLI output options override configuration defaults.
- [ ] 7.8 Add tests for compiler-style diagnostic output shape.
- [ ] 7.9 Add tests for JSON check output shape and diagnostic severity grouping.
- [ ] 7.10 Add tests proving severity filtering changes displayed diagnostics but not exit status.
- [ ] 7.11 Add tests for formatter-independent diagnostic collection.
- [ ] 7.12 Add tests for source locations across main templates and resolved partials where partial support is available.

## 8. Validation

- [ ] 8.1 Run `cargo fmt --check`.
- [ ] 8.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] 8.3 Run `cargo nextest run`.
- [ ] 8.4 Run `openspec validate provide-machine-readable-output --strict`.
- [ ] 8.5 Run `openspec validate --specs --strict`.
