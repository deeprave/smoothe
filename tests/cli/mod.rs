use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

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
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let counter = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "smoothe-cli-test-{}-{timestamp}-{counter}",
        std::process::id()
    ));
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
fn help_aliases_exit_successfully() {
    for flag in ["--help", "-h"] {
        let output = smoothe().arg(flag).output().expect("run smoothe");
        let stdout = stdout(&output);

        assert!(output.status.success(), "{flag} should succeed");
        assert!(stdout.contains("Usage:"), "{flag} should print usage");
        if flag == "--help" {
            assert!(stdout.contains("check  Check template syntax and correctness"));
            assert!(stdout.contains("parse  Parse templates and print AST output"));
        }
    }
}

#[test]
fn version_aliases_exit_successfully() {
    for flag in ["--version", "-V"] {
        let output = smoothe().arg(flag).output().expect("run smoothe");

        assert!(output.status.success(), "{flag} should succeed");
        assert_eq!(stdout(&output), "smoothe 0.1.0\n");
    }
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
fn parse_command_json_projects_reachable_node_kinds() {
    struct JsonProjectionCase {
        name: &'static str,
        source: &'static str,
        assert_json: fn(&[Value]),
    }

    let cases = [
        JsonProjectionCase {
            name: "text, escaped variable, and section",
            source: "Hello {{name}}{{#items}}{{.}}{{/items}}",
            assert_json: |nodes| {
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
            },
        },
        JsonProjectionCase {
            name: "unescaped variables",
            source: "{{{raw}}} {{& other}}",
            assert_json: |nodes| {
                assert_eq!(nodes[0]["kind"], "unescaped_variable");
                assert_eq!(nodes[0]["name"], "raw");
                assert_eq!(nodes[2]["kind"], "unescaped_variable");
                assert_eq!(nodes[2]["name"], "other");
            },
        },
        JsonProjectionCase {
            name: "comment and partial",
            source: "{{! note }}{{> header}}",
            assert_json: |nodes| {
                assert_eq!(nodes[0]["kind"], "comment");
                assert_eq!(nodes[0]["text"], "note");
                assert_eq!(nodes[1]["kind"], "partial");
                assert_eq!(nodes[1]["name"], "header");
            },
        },
        JsonProjectionCase {
            name: "inverted section",
            source: "{{^empty}}x{{/empty}}",
            assert_json: |nodes| {
                assert_eq!(nodes[0]["kind"], "inverted_section");
                assert_eq!(nodes[0]["name"], "empty");
                assert_eq!(nodes[0]["children"][0]["kind"], "text");
                assert_eq!(nodes[0]["children"][0]["text"], "x");
            },
        },
        JsonProjectionCase {
            name: "dynamic partial",
            source: "{{>* partial_name}}",
            assert_json: |nodes| {
                assert_eq!(nodes[0]["kind"], "dynamic_partial");
                assert_eq!(nodes[0]["expression"], "partial_name");
            },
        },
        JsonProjectionCase {
            name: "parent and block",
            source: "{{< layout}}{{$title}}Default{{/title}}{{/layout}}",
            assert_json: |nodes| {
                assert_eq!(nodes[0]["kind"], "parent");
                assert_eq!(nodes[0]["name"]["kind"], "static");
                assert_eq!(nodes[0]["name"]["value"], "layout");
                assert_eq!(nodes[0]["children"][0]["kind"], "block");
                assert_eq!(nodes[0]["children"][0]["name"], "title");
                assert_eq!(nodes[0]["children"][0]["children"][0]["kind"], "text");
                assert_eq!(nodes[0]["children"][0]["children"][0]["text"], "Default");
            },
        },
        JsonProjectionCase {
            name: "dynamic parent",
            source: "{{<* parent_name}}{{/parent_name}}",
            assert_json: |nodes| {
                assert_eq!(nodes[0]["kind"], "parent");
                assert_eq!(nodes[0]["name"]["kind"], "dynamic");
                assert_eq!(nodes[0]["name"]["value"], "parent_name");
            },
        },
        JsonProjectionCase {
            name: "delimiter change",
            source: "{{=<% %>=}}<%name%>",
            assert_json: |nodes| {
                assert_eq!(nodes[0]["kind"], "delimiter_change");
                assert_eq!(nodes[0]["open"], "<%");
                assert_eq!(nodes[0]["close"], "%>");
                assert_eq!(nodes[1]["kind"], "escaped_variable");
                assert_eq!(nodes[1]["name"], "name");
            },
        },
    ];

    for case in cases {
        let output = smoothe_with_stdin(&["parse", "--json", "-"], case.source);
        let json = json_stdout(&output);
        let nodes = json["inputs"][0]["ast"]["nodes"]
            .as_array()
            .expect("nodes array");

        assert!(output.status.success(), "{} should parse", case.name);
        (case.assert_json)(nodes);
    }
}

#[test]
fn parse_command_compact_output_projects_reachable_node_kinds() {
    struct CompactProjectionCase {
        name: &'static str,
        source: &'static str,
        expected_fragments: &'static [&'static str],
    }

    let cases = [
        CompactProjectionCase {
            name: "unescaped variables",
            source: "{{{raw}}} {{& other}}",
            expected_fragments: &[
                "unescaped_variable name=\"raw\" span=",
                "text value=\" \" span=",
                "unescaped_variable name=\"other\" span=",
            ],
        },
        CompactProjectionCase {
            name: "comment and partial",
            source: "{{! note }}{{> header}}",
            expected_fragments: &[
                "comment text=\"note\" span=",
                "partial name=\"header\" span=",
            ],
        },
        CompactProjectionCase {
            name: "inverted section",
            source: "{{^empty}}x{{/empty}}",
            expected_fragments: &[
                "inverted_section name=\"empty\" span=",
                "children=1",
                "text value=\"x\" span=",
            ],
        },
        CompactProjectionCase {
            name: "dynamic partial",
            source: "{{>* partial_name}}",
            expected_fragments: &["dynamic_partial expression=\"partial_name\" span="],
        },
        CompactProjectionCase {
            name: "static parent and block",
            source: "{{< layout}}{{$title}}Default{{/title}}{{/layout}}",
            expected_fragments: &[
                "parent name=static:\"layout\" span=",
                "block name=\"title\" span=",
                "text value=\"Default\" span=",
            ],
        },
        CompactProjectionCase {
            name: "dynamic parent",
            source: "{{<* parent_name}}{{/parent_name}}",
            expected_fragments: &["parent name=dynamic:\"parent_name\" span="],
        },
        CompactProjectionCase {
            name: "delimiter change",
            source: "{{=<% %>=}}<%name%>",
            expected_fragments: &[
                "delimiter_change open=\"<%\" close=\"%>\" span=",
                "escaped_variable name=\"name\" span=",
            ],
        },
    ];

    for case in cases {
        let output = smoothe_with_stdin(&["parse", "-"], case.source);
        let stdout = stdout(&output);

        assert!(output.status.success(), "{} should parse", case.name);
        assert!(stderr(&output).is_empty(), "{} should not warn", case.name);
        for fragment in case.expected_fragments {
            assert!(
                stdout.contains(fragment),
                "{} should contain compact fragment {fragment:?}; stdout was:\n{stdout}",
                case.name
            );
        }
    }
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
fn commands_report_missing_file() {
    for command in ["check", "parse"] {
        let output = smoothe()
            .args([command, "missing.mustache"])
            .output()
            .expect("run smoothe");
        let stderr = stderr(&output);

        assert!(!output.status.success(), "{command} should fail");
        assert!(
            output.stdout.is_empty(),
            "{command} should not write stdout"
        );
        assert!(stderr.contains("error: failed to read missing.mustache:"));
    }
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
fn parse_command_reports_output_write_failure() {
    let dir = temp_dir();
    let template = write_template(&dir, "valid.mustache", "Hello {{name}}");
    let output = smoothe()
        .arg("parse")
        .arg("--out")
        .arg(&dir)
        .arg(&template)
        .output()
        .expect("run smoothe");
    let stderr = stderr(&output);

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(stderr.contains("error: failed to write"));
    assert!(stderr.contains(&dir.display().to_string()));
}

#[test]
fn no_default_command_is_dispatched() {
    let output = smoothe().output().expect("run smoothe");

    assert!(!output.status.success());
}

#[test]
fn color_inputs_dispatch_check() {
    struct ColorCase<'a> {
        args: &'a [&'a str],
        environment: Option<(&'a str, &'a str)>,
    }

    let cases = [
        ColorCase {
            args: &["--color", "always", "check"],
            environment: None,
        },
        ColorCase {
            args: &["--colour", "always", "check"],
            environment: None,
        },
        ColorCase {
            args: &["-c", "always", "check"],
            environment: None,
        },
        ColorCase {
            args: &["--no-color", "check"],
            environment: None,
        },
        ColorCase {
            args: &["check"],
            environment: Some(("NOCOLOR", "1")),
        },
    ];

    for case in cases {
        let dir = temp_dir();
        let template = write_template(
            &dir,
            "valid.mustache",
            include_str!("../fixtures/parse-valid.mustache"),
        );
        let mut command = smoothe();
        if let Some((name, value)) = case.environment {
            command.env(name, value);
        }
        let output = command
            .args(case.args)
            .arg(&template)
            .output()
            .expect("run smoothe");

        assert!(output.status.success(), "{:?} should succeed", case.args);
        assert!(
            output.stdout.is_empty(),
            "{:?} should not write stdout",
            case.args
        );
    }
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
