pub mod check;
pub mod parse;

use std::{
    fs,
    io::{self, Read},
    path::Path,
    process::ExitCode,
};

use crate::cli::Commands;

pub fn dispatch(command: Commands) -> ExitCode {
    match command {
        Commands::Check(args) => check::check(args),
        Commands::Parse(args) => parse::parse(args),
    }
}

struct TemplateInput {
    name: String,
    source: String,
}

fn read_template_inputs(inputs: &[String]) -> Result<Vec<TemplateInput>, InputReadError> {
    inputs
        .iter()
        .map(|input| read_template_input(input))
        .collect()
}

fn read_template_input(input: &str) -> Result<TemplateInput, InputReadError> {
    if input == "-" {
        let mut source = String::new();
        io::stdin()
            .read_to_string(&mut source)
            .map_err(|error| InputReadError::new(input, error))?;

        return Ok(TemplateInput {
            name: "<stdin>".to_owned(),
            source,
        });
    }

    let source = fs::read_to_string(input).map_err(|error| InputReadError::new(input, error))?;

    Ok(TemplateInput {
        name: display_name(input),
        source,
    })
}

fn display_name(input: &str) -> String {
    Path::new(input)
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .unwrap_or(input)
        .to_owned()
}

struct InputReadError {
    input: String,
    source: io::Error,
}

impl InputReadError {
    fn new(input: &str, source: io::Error) -> Self {
        Self {
            input: input.to_owned(),
            source,
        }
    }
}

impl std::fmt::Display for InputReadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "failed to read {}: {}", self.input, self.source)
    }
}
