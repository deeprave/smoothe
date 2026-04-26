pub mod check;
pub mod parse;

use std::process::ExitCode;

use crate::cli::Commands;

pub fn dispatch(command: Commands) -> ExitCode {
    match command {
        Commands::Check(args) => {
            check::check(args);
            ExitCode::SUCCESS
        }
        Commands::Parse(args) => parse::parse(args),
    }
}
