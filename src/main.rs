mod cli;
mod commands;

use std::process::ExitCode;

fn main() -> ExitCode {
    cli::run()
}
