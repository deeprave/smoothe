use std::{path::PathBuf, process::ExitCode};

use clap::{ColorChoice, Parser, Subcommand};
use smoothe::config;

use crate::commands;

#[derive(Debug, Parser)]
#[command(name = "smoothe", version, subcommand_required = true)]
pub struct Cli {
    #[arg(
        long = "color",
        alias = "colour",
        short = 'c',
        value_name = "WHEN",
        value_parser = parse_color_choice
    )]
    color: Option<ColorChoice>,

    #[arg(long = "no-color", conflicts_with = "color")]
    no_color: bool,

    #[arg(long, short = 'C', value_name = "PATH")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check template syntax and correctness
    Check(CheckArgs),
    /// Parse templates and print AST output
    Parse(ParseArgs),
}

#[derive(Debug, Parser)]
pub struct CheckArgs {
    #[arg(required = true)]
    pub inputs: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct ParseArgs {
    #[arg(long, short = 'j')]
    pub json: bool,

    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,

    #[arg(required = true)]
    pub inputs: Vec<String>,
}

impl Cli {
    fn global_options(&self) -> config::CliGlobalOptions {
        config::CliGlobalOptions {
            color: self.color,
            no_color: self.no_color,
        }
    }
}

pub fn run() -> ExitCode {
    let cli = Cli::parse();
    let configuration = match config::load(cli.config.as_deref()) {
        Ok(configuration) => configuration,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };
    let options = config::resolve(&configuration, &cli.global_options());

    commands::dispatch(cli.command, options)
}

fn parse_color_choice(value: &str) -> Result<ColorChoice, String> {
    match value {
        "always" => Ok(ColorChoice::Always),
        "auto" => Ok(ColorChoice::Auto),
        "never" => Ok(ColorChoice::Never),
        other => Err(format!(
            "invalid color choice `{other}`; expected always, auto, or never"
        )),
    }
}
