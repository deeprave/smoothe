use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    time::{SystemTime, UNIX_EPOCH},
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

fn temp_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("smoothe-cli-test-{unique}"));
    fs::create_dir(&dir).expect("create temp dir");
    dir
}

fn write_template(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, content).expect("write template");
    path
}

#[test]
fn long_help_exits_successfully() {
    let output = smoothe().arg("--help").output().expect("run smoothe");

    assert!(output.status.success());
    assert!(stdout(&output).contains("Usage:"));
    assert!(stdout(&output).contains("check  Check template syntax and correctness"));
    assert!(stdout(&output).contains("parse  Parse templates and print AST output"));
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
fn check_command_accepts_file_operands() {
    let dir = temp_dir();
    let template = write_template(
        &dir,
        "valid.mustache",
        include_str!("../fixtures/parse-valid.mustache"),
    );
    let output = smoothe()
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn check_command_accepts_stdin_operand() {
    let output = smoothe_with_stdin(
        &["check", "-"],
        include_str!("../fixtures/parse-valid.mustache"),
    );

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn parse_command_accepts_file_operands_and_prints_compact_ast() {
    let dir = temp_dir();
    let template = write_template(
        &dir,
        "valid.mustache",
        include_str!("../fixtures/parse-valid.mustache"),
    );
    let output = smoothe()
        .arg("parse")
        .arg(&template)
        .output()
        .expect("run smoothe");
    let stdout = stdout(&output);

    assert!(output.status.success());
    assert!(stdout.contains("input valid.mustache"));
    assert!(stdout.contains("escaped_variable name=\"name\" span="));
    assert!(!stdout.contains("EscapedVariable {"));
    assert!(stderr(&output).is_empty());
}

#[test]
fn parse_command_accepts_stdin_operand() {
    let output = smoothe_with_stdin(
        &["parse", "-"],
        include_str!("../fixtures/parse-valid.mustache"),
    );

    assert!(output.status.success());
    assert!(stdout(&output).contains("input <stdin>"));
    assert!(stdout(&output).contains("escaped_variable name=\"name\""));
    assert!(stderr(&output).is_empty());
}

#[test]
fn parse_command_prints_error_diagnostics_and_exits_unsuccessfully() {
    let output = smoothe_with_stdin(
        &["parse", "-"],
        include_str!("../fixtures/parse-invalid.mustache"),
    );
    let stderr = stderr(&output);

    assert!(!output.status.success());
    assert!(stderr.contains("error"));
    assert!(stderr.contains("UnclosedSection"));
    assert!(stderr.contains("<stdin>:4:7"));
    assert!(stdout(&output).contains("input <stdin>"));
}

#[test]
fn parse_command_prints_warning_diagnostics_and_exits_successfully() {
    let output = smoothe_with_stdin(
        &["parse", "-"],
        include_str!("../fixtures/parse-warning.mustache"),
    );
    let stderr = stderr(&output);

    assert!(output.status.success());
    assert!(stderr.contains("warning"));
    assert!(stderr.contains("FrontmatterParseError"));
    assert!(stderr.contains("<stdin>:1:1"));
    assert!(stdout(&output).contains("escaped_variable"));
}

#[test]
fn parse_command_processes_multiple_operands_in_order() {
    let dir = temp_dir();
    let first = write_template(&dir, "first.mustache", "first {{name}}");
    let second = write_template(&dir, "second.mustache", "second {{value}}");
    let output = smoothe()
        .arg("parse")
        .arg(&first)
        .arg(&second)
        .output()
        .expect("run smoothe");
    let stdout = stdout(&output);
    let first_position = stdout.find("input first.mustache").expect("first input");
    let second_position = stdout.find("input second.mustache").expect("second input");

    assert!(output.status.success());
    assert!(first_position < second_position);
}

#[test]
fn check_command_reports_missing_file() {
    let output = smoothe()
        .args(["check", "missing.mustache"])
        .output()
        .expect("run smoothe");
    let stderr = stderr(&output);

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(stderr.contains("error: failed to read missing.mustache:"));
}

#[test]
fn parse_command_reports_missing_file() {
    let output = smoothe()
        .args(["parse", "missing.mustache"])
        .output()
        .expect("run smoothe");
    let stderr = stderr(&output);

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(stderr.contains("error: failed to read missing.mustache:"));
}

#[test]
fn parse_command_writes_output_file_and_suppresses_standard_output() {
    let dir = temp_dir();
    let template = write_template(
        &dir,
        "warning.mustache",
        include_str!("../fixtures/parse-warning.mustache"),
    );
    let output_path = dir.join("parse.txt");
    let output = smoothe()
        .arg("parse")
        .arg("--out")
        .arg(&output_path)
        .arg(&template)
        .output()
        .expect("run smoothe");
    let parse_output = fs::read_to_string(&output_path).expect("read parse output");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
    assert!(parse_output.contains("warning FrontmatterParseError"));
    assert!(parse_output.contains("input warning.mustache"));
    assert!(parse_output.contains("escaped_variable"));
}

#[test]
fn no_default_command_is_dispatched() {
    let output = smoothe().output().expect("run smoothe");

    assert!(!output.status.success());
}

#[test]
fn long_color_option_dispatches_check() {
    let dir = temp_dir();
    let template = write_template(
        &dir,
        "valid.mustache",
        include_str!("../fixtures/parse-valid.mustache"),
    );
    let output = smoothe()
        .args(["--color", "always", "check"])
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn long_colour_alias_dispatches_check() {
    let dir = temp_dir();
    let template = write_template(
        &dir,
        "valid.mustache",
        include_str!("../fixtures/parse-valid.mustache"),
    );
    let output = smoothe()
        .args(["--colour", "always", "check"])
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn short_color_alias_dispatches_check() {
    let dir = temp_dir();
    let template = write_template(
        &dir,
        "valid.mustache",
        include_str!("../fixtures/parse-valid.mustache"),
    );
    let output = smoothe()
        .args(["-c", "always", "check"])
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn no_color_flag_dispatches_check() {
    let dir = temp_dir();
    let template = write_template(
        &dir,
        "valid.mustache",
        include_str!("../fixtures/parse-valid.mustache"),
    );
    let output = smoothe()
        .args(["--no-color", "check"])
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn nocolor_environment_dispatches_check() {
    let dir = temp_dir();
    let template = write_template(
        &dir,
        "valid.mustache",
        include_str!("../fixtures/parse-valid.mustache"),
    );
    let output = smoothe()
        .env("NOCOLOR", "1")
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}
