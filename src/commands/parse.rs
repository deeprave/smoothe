use std::{
    io::{self, Read},
    process::ExitCode,
};

use smoothe::parser::{
    Diagnostic, DiagnosticSeverity, ParserInput, SourceMetadata, parse as parse_template,
};

use crate::cli::ParseArgs;

pub fn parse(_args: ParseArgs) -> ExitCode {
    let mut source = String::new();
    if let Err(error) = io::stdin().read_to_string(&mut source) {
        eprintln!("error: failed to read stdin: {error}");
        return ExitCode::FAILURE;
    }

    let result = parse_template(ParserInput::new(SourceMetadata::new("<stdin>"), &source));

    for diagnostic in &result.state.diagnostics {
        eprintln!("{}", format_diagnostic(diagnostic));
    }

    println!("{:#?}", result.ast);

    if result
        .state
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
    {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn format_diagnostic(diagnostic: &Diagnostic) -> String {
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
