use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

fn smoothe() -> Command {
    Command::new(env!("CARGO_BIN_EXE_smoothe"))
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn smoothe_with_stdin(args: &[&str], stdin: &str) -> Output {
    let mut child = smoothe()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn smoothe");

    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(stdin.as_bytes())
        .expect("write stdin");

    child.wait_with_output().expect("run smoothe")
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
fn parse_command_reads_stdin_and_prints_ast() {
    let output = smoothe_with_stdin(&["parse"], include_str!("../fixtures/parse-valid.mustache"));

    assert!(output.status.success());
    assert!(stdout(&output).contains("EscapedVariable"));
    assert!(stdout(&output).contains("name"));
    assert!(stderr(&output).is_empty());
}

#[test]
fn parse_command_accepts_empty_stdin() {
    let output = smoothe_with_stdin(&["parse"], "");

    assert!(output.status.success());
    assert!(stdout(&output).contains("Ast"));
    assert!(stderr(&output).is_empty());
}

#[test]
fn parse_command_prints_error_diagnostics_and_exits_unsuccessfully() {
    let output = smoothe_with_stdin(
        &["parse"],
        include_str!("../fixtures/parse-invalid.mustache"),
    );
    let stderr = stderr(&output);

    assert!(!output.status.success());
    assert!(stderr.contains("error"));
    assert!(stderr.contains("UnclosedSection"));
    assert!(stderr.contains("<stdin>:4:7"));
    assert!(stdout(&output).contains("Ast"));
}

#[test]
fn parse_command_prints_warning_diagnostics_and_exits_successfully() {
    let output = smoothe_with_stdin(
        &["parse"],
        include_str!("../fixtures/parse-warning.mustache"),
    );
    let stderr = stderr(&output);

    assert!(output.status.success());
    assert!(stderr.contains("warning"));
    assert!(stderr.contains("FrontmatterParseError"));
    assert!(stderr.contains("<stdin>:1:1"));
    assert!(stdout(&output).contains("EscapedVariable"));
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
