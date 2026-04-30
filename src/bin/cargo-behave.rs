use std::{
    ffi::OsString,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1).collect::<Vec<_>>();
    if should_inject_smoothe_bin(&args) {
        if let Some(path) = std::env::var_os("CARGO_BIN_EXE_smoothe") {
            args.splice(0..0, [OsString::from("--smoothe-bin"), path]);
        } else if let Err(error) = ensure_smoothe_binary() {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    }

    match smoothe::behave_runner::run_from_args_os(args) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn should_inject_smoothe_bin(args: &[OsString]) -> bool {
    !args.iter().any(|arg| arg == "--smoothe-bin")
        && !args.iter().any(|arg| {
            matches!(
                arg.to_str(),
                Some("--help" | "-h" | "--version" | "-V" | "--list")
            )
        })
}

fn ensure_smoothe_binary() -> Result<(), String> {
    let status = Command::new("cargo")
        .args(["build", "--bin", "smoothe"])
        .status()
        .map_err(|error| format!("build smoothe binary: {error}"))?;

    if status.success() {
        return Ok(());
    }

    Err(format!(
        "build smoothe binary exited with status {}",
        status.code().unwrap_or(1)
    ))
}
