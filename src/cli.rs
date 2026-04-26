use std::{env, path::PathBuf, process::ExitCode};

use clap::{ColorChoice, Parser, Subcommand};

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
    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,

    #[arg(required = true)]
    pub inputs: Vec<String>,
}

impl Cli {
    fn color_choice(&self) -> ColorChoice {
        if self.no_color {
            return ColorChoice::Never;
        }

        if let Some(color) = self.color {
            return color;
        }

        if env::var_os("NOCOLOR").is_some() {
            return ColorChoice::Never;
        }

        ColorChoice::Auto
    }
}

pub fn run() -> ExitCode {
    let cli = Cli::parse();
    let _color_choice = cli.color_choice();

    commands::dispatch(cli.command)
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
