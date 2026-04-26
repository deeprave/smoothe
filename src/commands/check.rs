use std::process::ExitCode;

use smoothe::config::{ResolvedCheckOptions, ResolvedGlobalOptions};
use smoothe::content::{ContentInput, process as process_template};
use smoothe::parser::{Diagnostic, DiagnosticSeverity, SourceMetadata};

use crate::cli::CheckArgs;

use super::read_template_inputs;

pub fn check(
    args: CheckArgs,
    global_options: ResolvedGlobalOptions,
    _check_options: ResolvedCheckOptions,
) -> ExitCode {
    let _color = global_options.color;

    let inputs = match read_template_inputs(&args.inputs) {
        Ok(inputs) => inputs,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let mut has_error = false;

    for input in inputs {
        let mut source = SourceMetadata::new(&input.name);
        if let Some(root) = input.root {
            source = source.with_root(root);
        }
        let result = process_template(ContentInput::new(source, &input.source));

        for diagnostic in &result.state.diagnostics {
            eprintln!("{}", format_diagnostic(diagnostic));
        }

        has_error |= result
            .state
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
    }

    if has_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

pub(crate) fn format_diagnostic(diagnostic: &Diagnostic) -> String {
    format!(
        "{} {:?} at {}:{}:{}: {}",
        severity_label(diagnostic.severity),
        diagnostic.issue,
        diagnostic.source_name,
        diagnostic.location.line,
        diagnostic.location.column,
        diagnostic.message
    )
}

fn severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Info => "info",
        DiagnosticSeverity::Debug => "debug",
    }
}
