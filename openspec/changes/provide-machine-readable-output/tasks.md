## 1. Options and Configuration

- [x] 1.1 Add check output format option types for compiler-style and JSON output.
- [x] 1.2 Add check verbosity option types for error, warning, info, debug, and trace event display thresholds.
- [x] 1.3 Add `--json` to the check command as a JSON output shortcut.
- [x] 1.4 Add `--format <compiler|json>` to the check command.
- [x] 1.5 Add `--no-json` to force compiler-style output when JSON is configured by default.
- [x] 1.6 Let explicit `--format` selections override `--json` and `--no-json` default-output flags.
- [x] 1.7 Reject conflicting default-output flags such as `--json --no-json`.
- [x] 1.8 Add `--verbosity <error|warning|info|debug|trace>` to the check command.
- [x] 1.9 Add `[check] output` configuration support.
- [x] 1.10 Add `[check] verbosity` configuration support.
- [x] 1.11 Resolve check output and verbosity options with CLI values overriding configuration defaults.

## 2. Check Event Stream

- [x] 2.1 Define check event types for run started, input started, partial started, partial finished, partial skipped, progress, trace, diagnostic, input finished, and run finished.
- [x] 2.2 Include structured diagnostic data in diagnostic events.
- [x] 2.3 Assign every check event a verbosity level.
- [x] 2.4 Emit diagnostic events incrementally from schema loading, lambda loading, content processing, parser output, and semantic validation.
- [x] 2.5 Emit partial lifecycle events during resolved partial traversal.
- [x] 2.6 Emit progress and trace events for non-diagnostic check activity where useful.
- [x] 2.7 Preserve current check validation behavior while removing direct diagnostic `eprintln!` calls from validation flow.
- [x] 2.8 Track unfiltered run state separately for exit-status calculation.
- [x] 2.9 Ensure fatal setup errors that happen before check events can be emitted still report clearly.

## 3. Listener Abstraction

- [x] 3.1 Define a check event listener interface or enum dispatch pattern.
- [x] 3.2 Add fan-out support so multiple listeners can observe the same event stream in order.
- [x] 3.3 Pass structured check events to selected listeners.
- [x] 3.4 Apply verbosity filtering only inside output listeners.
- [x] 3.5 Ensure listener selection does not mutate diagnostics or affect exit status.
- [x] 3.6 Keep listener code extensible for future output formats and integrations.

## 4. Compiler-Style Listener

- [x] 4.1 Implement compiler-style output as the default check listener.
- [x] 4.2 Format diagnostics as source, line, column, severity, issue kind, and message.
- [x] 4.3 Include error, warning, info, debug, and trace event labels where rendered.
- [x] 4.4 Write compiler-style diagnostics to stderr.
- [x] 4.5 Preserve accurate main-template source locations in compiler-style output.
- [x] 4.6 Preserve accurate partial source locations in compiler-style output.
- [x] 4.7 Stream compiler-style diagnostics as diagnostic events arrive.
- [x] 4.8 Render non-diagnostic progress, partial, debug, or trace events only when selected verbosity requires them.

## 5. JSON Listener

- [x] 5.1 Implement JSON check output selected by `--json` or `--format json`.
- [x] 5.2 Emit one valid JSON document to stdout.
- [x] 5.3 Include an `inputs` list with one result object per checked input.
- [x] 5.4 Group diagnostics into `errors`, `warnings`, `info`, and `debug` lists.
- [x] 5.4a Represent trace-level output as events, not diagnostic groups, because diagnostics do not have a trace severity.
- [x] 5.5 Include issue kind, severity, source name, line, column, span, and message for each JSON diagnostic.
- [x] 5.6 Include optional structured diagnostic details when present.
- [x] 5.7 Preserve accurate main-template source locations in JSON output.
- [x] 5.8 Preserve accurate partial source locations in JSON output.
- [x] 5.9 Allow the JSON listener to buffer events internally while consuming the same event stream as other listeners.
- [x] 5.10 Include event verbosity on JSON event or diagnostic records.
- [x] 5.11 Include run-level schema and lambda diagnostics in top-level JSON diagnostic groups.
- [x] 5.12 Represent non-diagnostic progress, partial lifecycle, debug, and trace events when they pass the selected verbosity filter.

## 6. Verbosity Filtering

- [x] 6.1 Default event display to warnings and errors.
- [x] 6.2 Implement error-only event display.
- [x] 6.3 Implement info-level event display.
- [x] 6.4 Implement debug-level event display.
- [x] 6.5 Implement trace-level event display.
- [x] 6.6 Ensure verbosity filtering does not change check exit status.
- [x] 6.7 Ensure JSON output applies the selected verbosity filter consistently.
- [x] 6.8 Ensure one listener's verbosity filter does not prevent other listeners from receiving events.

## 7. Tests

- [x] 7.1 Add CLI tests for `check --json`.
- [x] 7.2 Add CLI tests for `check --format compiler` and `check --format json`.
- [x] 7.3 Add CLI tests for conflicting output options.
- [x] 7.4 Add CLI tests for valid and invalid `--verbosity` values.
- [x] 7.5 Add configuration tests for `[check] output`.
- [x] 7.6 Add configuration tests for `[check] verbosity`.
- [x] 7.7 Add tests proving CLI output options override configuration defaults.
- [x] 7.8 Add tests for compiler-style diagnostic output shape.
- [x] 7.9 Add tests for JSON check output shape, event verbosity, and diagnostic severity grouping.
- [x] 7.10 Add tests proving verbosity filtering changes displayed events but not exit status.
- [x] 7.11 Add tests for listener-independent diagnostic event emission.
- [x] 7.12 Add tests for source locations across main templates and resolved partials where partial support is available.
- [x] 7.13 Add tests for listener fan-out receiving events in order.
- [x] 7.14 Add tests for partial lifecycle events.
- [x] 7.15 Add tests for progress and trace event filtering.

## 8. Validation

- [x] 8.1 Run `cargo fmt --check`.
- [x] 8.2 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] 8.3 Run `cargo nextest run`.
- [x] 8.4 Run `openspec validate provide-machine-readable-output --strict`.
- [x] 8.5 Run `openspec validate --specs --strict`.
