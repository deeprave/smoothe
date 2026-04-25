use std::process::{Command, Output};

fn smoothe() -> Command {
    Command::new(env!("CARGO_BIN_EXE_smoothe"))
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn long_help_exits_successfully() {
    let output = smoothe().arg("--help").output().expect("run smoothe");

    assert!(output.status.success());
    assert!(stdout(&output).contains("Usage:"));
}

#[test]
fn short_help_exits_successfully() {
    let output = smoothe().arg("-h").output().expect("run smoothe");

    assert!(output.status.success());
    assert!(stdout(&output).contains("Usage:"));
}

#[test]
fn long_version_exits_successfully() {
    let output = smoothe().arg("--version").output().expect("run smoothe");

    assert!(output.status.success());
    assert_eq!(stdout(&output), "smoothe 0.1.0\n");
}

#[test]
fn short_version_exits_successfully() {
    let output = smoothe().arg("-V").output().expect("run smoothe");

    assert!(output.status.success());
    assert_eq!(stdout(&output), "smoothe 0.1.0\n");
}

#[test]
fn check_command_exits_successfully() {
    let output = smoothe().arg("check").output().expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn no_default_command_is_dispatched() {
    let output = smoothe().output().expect("run smoothe");

    assert!(!output.status.success());
}

#[test]
fn long_color_option_dispatches_check() {
    let output = smoothe()
        .args(["--color", "always", "check"])
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn long_colour_alias_dispatches_check() {
    let output = smoothe()
        .args(["--colour", "always", "check"])
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn short_color_alias_dispatches_check() {
    let output = smoothe()
        .args(["-c", "always", "check"])
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn no_color_flag_dispatches_check() {
    let output = smoothe()
        .args(["--no-color", "check"])
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn nocolor_environment_dispatches_check() {
    let output = smoothe()
        .env("NOCOLOR", "1")
        .arg("check")
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}
