use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

fn smoothe() -> Command {
    Command::new(env!("CARGO_BIN_EXE_smoothe"))
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn json_stdout(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("parse JSON stdout")
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

fn smoothe_with_isolated_config(cwd: &std::path::Path, home: &std::path::Path) -> Command {
    let mut command = smoothe();
    command
        .current_dir(cwd)
        .env_remove("XDG_CONFIG_HOME")
        .env("HOME", home)
        .env_remove("NOCOLOR");
    command
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
fn parse_command_json_flag_prints_valid_json() {
    let dir = temp_dir();
    let template = write_template(&dir, "valid.mustache", "Hello {{name}}");
    let output = smoothe()
        .arg("parse")
        .arg("--json")
        .arg(&template)
        .output()
        .expect("run smoothe");
    let json = json_stdout(&output);

    assert!(output.status.success());
    assert!(stderr(&output).is_empty());
    assert_eq!(json["inputs"][0]["name"], "valid.mustache");
    assert!(json["inputs"][0]["ast"]["nodes"].is_array());
}

#[test]
fn parse_command_short_json_flag_prints_json() {
    let output = smoothe_with_stdin(&["parse", "-j", "-"], "Hello {{name}}");
    let json = json_stdout(&output);

    assert!(output.status.success());
    assert_eq!(json["inputs"][0]["name"], "<stdin>");
    assert_eq!(json["inputs"][0]["errors"], Value::Array(Vec::new()));
    assert_eq!(json["inputs"][0]["warnings"], Value::Array(Vec::new()));
}

#[test]
fn parse_command_json_output_contains_all_inputs() {
    let dir = temp_dir();
    let first = write_template(&dir, "first.mustache", "first {{name}}");
    let second = write_template(&dir, "second.mustache", "second {{value}}");
    let output = smoothe()
        .arg("parse")
        .arg("--json")
        .arg(&first)
        .arg(&second)
        .output()
        .expect("run smoothe");
    let json = json_stdout(&output);
    let inputs = json["inputs"].as_array().expect("inputs array");

    assert!(output.status.success());
    assert_eq!(inputs.len(), 2);
    assert_eq!(inputs[0]["name"], "first.mustache");
    assert_eq!(inputs[1]["name"], "second.mustache");
}

#[test]
fn parse_command_json_represents_empty_ast() {
    let output = smoothe_with_stdin(&["parse", "--json", "-"], "");
    let json = json_stdout(&output);

    assert!(output.status.success());
    assert_eq!(json["inputs"][0]["ast"]["nodes"], Value::Array(Vec::new()));
}

#[test]
fn parse_command_json_writes_output_file() {
    let dir = temp_dir();
    let template = write_template(&dir, "valid.mustache", "Hello {{name}}");
    let output_path = dir.join("parse.json");
    let output = smoothe()
        .arg("parse")
        .arg("--json")
        .arg("--out")
        .arg(&output_path)
        .arg(&template)
        .output()
        .expect("run smoothe");
    let json: Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("read JSON output"))
            .expect("parse JSON output file");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
    assert_eq!(json["inputs"][0]["name"], "valid.mustache");
}

#[test]
fn parse_command_without_json_keeps_compact_tree_output() {
    let output = smoothe_with_stdin(&["parse", "-"], "Hello {{name}}");
    let stdout = stdout(&output);

    assert!(output.status.success());
    assert!(stdout.starts_with("input <stdin>\n"));
    assert!(stdout.contains("escaped_variable name=\"name\""));
    assert!(serde_json::from_str::<Value>(&stdout).is_err());
}

#[test]
fn parse_command_json_projects_scalar_and_container_nodes() {
    let output = smoothe_with_stdin(
        &["parse", "--json", "-"],
        "Hello {{name}}{{#items}}{{.}}{{/items}}",
    );
    let json = json_stdout(&output);
    let nodes = json["inputs"][0]["ast"]["nodes"]
        .as_array()
        .expect("nodes array");

    assert!(output.status.success());
    assert_eq!(nodes[0]["kind"], "text");
    assert_eq!(nodes[0]["text"], "Hello ");
    assert_eq!(nodes[0]["span"]["start"], 0);
    assert_eq!(nodes[0]["span"]["end"], 6);
    assert_eq!(nodes[1]["kind"], "escaped_variable");
    assert_eq!(nodes[1]["name"], "name");
    assert_eq!(nodes[2]["kind"], "section");
    assert_eq!(nodes[2]["name"], "items");
    assert_eq!(nodes[2]["children"][0]["kind"], "escaped_variable");
    assert_eq!(nodes[2]["children"][0]["name"], ".");
}

#[test]
fn parse_command_json_groups_errors_and_warnings() {
    let warning = smoothe_with_stdin(
        &["parse", "--json", "-"],
        include_str!("../fixtures/parse-warning.mustache"),
    );
    let warning_json = json_stdout(&warning);
    let error = smoothe_with_stdin(
        &["parse", "--json", "-"],
        include_str!("../fixtures/parse-invalid.mustache"),
    );
    let error_json = json_stdout(&error);

    assert!(warning.status.success());
    assert!(stderr(&warning).is_empty());
    assert_eq!(
        warning_json["inputs"][0]["warnings"][0]["issue"],
        "FrontmatterParseError"
    );
    assert_eq!(
        warning_json["inputs"][0]["warnings"][0]["source"],
        "<stdin>"
    );
    assert_eq!(warning_json["inputs"][0]["warnings"][0]["line"], 1);
    assert_eq!(warning_json["inputs"][0]["warnings"][0]["column"], 1);
    assert!(
        warning_json["inputs"][0]["warnings"][0]["message"]
            .as_str()
            .expect("warning message")
            .contains("frontmatter")
    );
    assert_eq!(
        warning_json["inputs"][0]["errors"],
        Value::Array(Vec::new())
    );

    assert!(!error.status.success());
    assert!(stderr(&error).is_empty());
    assert_eq!(
        error_json["inputs"][0]["errors"][0]["issue"],
        "UnclosedSection"
    );
    assert_eq!(error_json["inputs"][0]["errors"][0]["source"], "<stdin>");
    assert_eq!(error_json["inputs"][0]["errors"][0]["line"], 4);
    assert_eq!(error_json["inputs"][0]["errors"][0]["column"], 7);
    assert!(error_json["inputs"][0]["errors"][0]["span"]["start"].is_number());
    assert_eq!(
        error_json["inputs"][0]["warnings"],
        Value::Array(Vec::new())
    );
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

#[test]
fn startup_succeeds_without_discovered_config() {
    let dir = temp_dir();
    let home = temp_dir();
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    let output = smoothe_with_isolated_config(&dir, &home)
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn current_directory_config_is_loaded() {
    let dir = temp_dir();
    let home = temp_dir();
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    fs::write(dir.join("smoothe.toml"), "not toml =").expect("write config");
    let output = smoothe_with_isolated_config(&dir, &home)
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("failed to parse configuration"));
}

#[test]
fn xdg_config_home_config_is_loaded_after_current_directory_miss() {
    let dir = temp_dir();
    let xdg = temp_dir();
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    fs::write(xdg.join("smoothe.toml"), "not toml =").expect("write config");
    let output = smoothe()
        .current_dir(&dir)
        .env("XDG_CONFIG_HOME", &xdg)
        .env_remove("NOCOLOR")
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("failed to parse configuration"));
}

#[test]
fn default_home_config_is_loaded_when_xdg_config_home_is_unset() {
    let dir = temp_dir();
    let home = temp_dir();
    let config_dir = home.join(".config");
    fs::create_dir(&config_dir).expect("create config dir");
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    fs::write(config_dir.join("smoothe.toml"), "not toml =").expect("write config");
    let output = smoothe_with_isolated_config(&dir, &home)
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("failed to parse configuration"));
}

#[test]
fn explicit_config_path_is_loaded() {
    let dir = temp_dir();
    let home = temp_dir();
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    let config = dir.join("custom.toml");
    fs::write(&config, "[options]\ncolor = \"auto\"\n[check]\n").expect("write config");
    let output = smoothe_with_isolated_config(&dir, &home)
        .arg("--config")
        .arg(&config)
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}

#[test]
fn short_explicit_config_path_is_loaded() {
    let dir = temp_dir();
    let home = temp_dir();
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    let config = dir.join("custom.toml");
    fs::write(&config, "[options]\ncolor = true\n").expect("write config");
    let output = smoothe_with_isolated_config(&dir, &home)
        .arg("-C")
        .arg(&config)
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}

#[test]
fn missing_explicit_config_path_fails_startup() {
    let dir = temp_dir();
    let home = temp_dir();
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    let output = smoothe_with_isolated_config(&dir, &home)
        .args(["--config", "missing.toml", "check"])
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("failed to read configuration missing.toml:"));
}

#[test]
fn malformed_explicit_config_path_fails_startup() {
    let dir = temp_dir();
    let home = temp_dir();
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    let config = dir.join("invalid.toml");
    fs::write(&config, "not toml =").expect("write config");
    let output = smoothe_with_isolated_config(&dir, &home)
        .arg("--config")
        .arg(&config)
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("failed to parse configuration"));
}

#[test]
fn config_color_values_parse_successfully() {
    for color in ["true", "false", "\"always\"", "\"never\"", "\"auto\""] {
        let dir = temp_dir();
        let home = temp_dir();
        let template = write_template(&dir, "valid.mustache", "{{name}}");
        let config = dir.join("custom.toml");
        fs::write(&config, format!("[options]\ncolor = {color}\n")).expect("write config");
        let output = smoothe_with_isolated_config(&dir, &home)
            .arg("--config")
            .arg(&config)
            .arg("check")
            .arg(&template)
            .output()
            .expect("run smoothe");

        assert!(output.status.success(), "color value {color} should parse");
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn current_directory_config_prevents_config_home_read() {
    let dir = temp_dir();
    let xdg = temp_dir();
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    fs::write(dir.join("smoothe.toml"), "[options]\ncolor = \"auto\"\n").expect("write config");
    fs::write(xdg.join("smoothe.toml"), "not toml =").expect("write config");
    let output = smoothe()
        .current_dir(&dir)
        .env("XDG_CONFIG_HOME", &xdg)
        .env_remove("NOCOLOR")
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}

#[test]
fn explicit_config_bypasses_discovered_config_paths() {
    let dir = temp_dir();
    let xdg = temp_dir();
    let template = write_template(&dir, "valid.mustache", "{{name}}");
    let explicit = dir.join("custom.toml");
    fs::write(dir.join("smoothe.toml"), "not toml =").expect("write config");
    fs::write(xdg.join("smoothe.toml"), "not toml =").expect("write config");
    fs::write(&explicit, "[options]\ncolor = \"auto\"\n").expect("write config");
    let output = smoothe()
        .current_dir(&dir)
        .env("XDG_CONFIG_HOME", &xdg)
        .env_remove("NOCOLOR")
        .arg("--config")
        .arg(&explicit)
        .arg("check")
        .arg(&template)
        .output()
        .expect("run smoothe");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}
