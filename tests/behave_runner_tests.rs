use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn cargo_behave() -> Command {
    Command::new(std::env::var("CARGO_BIN_EXE_cargo-behave").expect("CARGO_BIN_EXE_cargo-behave"))
}

fn temp_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let counter = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "smoothe-behave-test-{}-{timestamp}-{counter}",
        std::process::id()
    ));
    fs::create_dir(&dir).expect("create temp dir");
    dir
}

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write file");
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn behave_lists_discovered_case_manifests() {
    let root = temp_dir();
    write(
        &root.join("behavior/fixtures/parse/simple/case.toml"),
        r#"
args = ["parse", "template.mustache"]
status = 0
"#,
    );

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--list")
        .output()
        .expect("run cargo behave");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(stdout(&output), "parse/simple\n");
}

#[test]
fn behave_filters_with_positional_arguments_using_or_semantics() {
    let root = temp_dir();
    for id in [
        "check/config-partials",
        "check/schema-warning",
        "parse/json",
        "parse/simple",
    ] {
        write(
            &root.join("behavior/fixtures").join(id).join("case.toml"),
            r#"
args = ["parse", "template.mustache"]
status = 0
"#,
        );
    }

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--list")
        .arg("check")
        .arg("parse/json")
        .output()
        .expect("run cargo behave");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "check/config-partials\ncheck/schema-warning\nparse/json\n"
    );
}

#[test]
fn behave_list_allows_missing_fixture_root() {
    let root = temp_dir().join("missing-root");

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--list")
        .output()
        .expect("run cargo behave");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(stdout(&output).is_empty());
}

#[test]
fn behave_run_fails_for_missing_fixture_root() {
    let root = temp_dir().join("missing-root");

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .output()
        .expect("run cargo behave");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("fixture root not found"));
}

#[test]
fn behave_run_fails_when_filters_match_no_cases() {
    let root = temp_dir();
    write(
        &root.join("behavior/fixtures/parse/simple/case.toml"),
        r#"
args = ["parse", "template.mustache"]
status = 0
"#,
    );

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .arg("does-not-match")
        .output()
        .expect("run cargo behave");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("no behavior cases matched"));
}

#[test]
fn behave_rejects_fixture_local_config_without_manifest_config() {
    let root = temp_dir();
    let case = root.join("behavior/fixtures/check/implicit-config");
    write(
        &case.join("case.toml"),
        r#"
args = ["check", "template.mustache"]
status = 0
"#,
    );
    write(
        &case.join("smoothe.toml"),
        "[check]\nverbosity = \"trace\"\n",
    );
    write(&case.join("template.mustache"), "Hello {{name}}\n");

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .output()
        .expect("run cargo behave");

    assert!(!output.status.success());
    assert!(
        stderr(&output)
            .contains("case check/implicit-config contains smoothe.toml but does not set `config`")
    );
}

#[test]
fn behave_rejects_manifest_paths_that_escape_case_directory() {
    let root = temp_dir();
    let outside = root.join("behavior/fixtures/parse/outside.txt");
    write(&outside, "do not overwrite\n");
    let case = root.join("behavior/fixtures/parse/escape");
    write(
        &case.join("case.toml"),
        r#"
args = ["parse", "template.mustache"]
status = 0
stdout = "../outside.txt"
"#,
    );
    write(&case.join("template.mustache"), "Hello {{name}}\n");

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .arg("--update")
        .output()
        .expect("run cargo behave");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("`stdout` path must stay within the case directory"));
    assert_eq!(
        fs::read_to_string(&outside).expect("read outside file"),
        "do not overwrite\n"
    );
}

#[test]
fn behave_rejects_absolute_manifest_paths() {
    let root = temp_dir();
    let case = root.join("behavior/fixtures/check/absolute-config");
    let absolute_config = case.join("smoothe.toml");
    write(
        &case.join("case.toml"),
        &format!(
            "config = {:?}\nargs = [\"check\", \"template.mustache\"]\nstatus = 0\n",
            absolute_config
        ),
    );
    write(&absolute_config, "[check]\n");
    write(&case.join("template.mustache"), "Hello {{name}}\n");

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .output()
        .expect("run cargo behave");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("`config` path must stay within the case directory"));
}

#[test]
fn behave_runs_fixture_and_compares_text_output() {
    let root = temp_dir();
    let case = root.join("behavior/fixtures/parse/simple");
    write(
        &case.join("case.toml"),
        r#"
args = ["parse", "template.mustache"]
status = 0
stdout = "expected.stdout"
stderr = "expected.stderr"
"#,
    );
    write(&case.join("template.mustache"), "Hello {{name}}\n");
    write(
        &case.join("expected.stdout"),
        "input template.mustache\n  text value=\"Hello \" span=0..6\n  escaped_variable name=\"name\" span=6..14\n  text value=\"\\n\" span=14..15\n",
    );
    write(&case.join("expected.stderr"), "");

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .output()
        .expect("run cargo behave");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(stdout(&output).contains("ok parse/simple"));
}

#[test]
fn behave_text_mismatch_reports_expected_path_and_first_difference() {
    let root = temp_dir();
    let case = root.join("behavior/fixtures/parse/mismatch");
    write(
        &case.join("case.toml"),
        r#"
args = ["parse", "template.mustache"]
status = 0
stdout = "expected.stdout"
"#,
    );
    write(&case.join("template.mustache"), "Hello {{name}}\n");
    write(&case.join("expected.stdout"), "not the parser output\n");

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .output()
        .expect("run cargo behave");
    let stdout = stdout(&output);

    assert!(!output.status.success());
    assert!(stdout.contains("stdout mismatch in"));
    assert!(stdout.contains("expected.stdout"));
    assert!(
        stdout.contains(
            "line 1: expected \"not the parser output\", got \"input template.mustache\""
        )
    );
}

#[test]
fn behave_compares_json_output_structurally() {
    let root = temp_dir();
    let case = root.join("behavior/fixtures/parse/json");
    write(
        &case.join("case.toml"),
        r#"
args = ["parse", "--json", "template.mustache"]
status = 0
stdout = "expected.stdout.json"
stdout_format = "json"
stderr = "expected.stderr"
"#,
    );
    write(&case.join("template.mustache"), "Hello {{name}}\n");
    write(
        &case.join("expected.stdout.json"),
        r#"{"inputs":[{"name":"template.mustache","ast":{"nodes":[{"kind":"text","text":"Hello ","span":{"start":0,"end":6}},{"kind":"escaped_variable","name":"name","span":{"start":6,"end":14}},{"kind":"text","text":"\n","span":{"start":14,"end":15}}],"template_units":[]},"errors":[],"warnings":[]}]}"#,
    );
    write(&case.join("expected.stderr"), "");

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .output()
        .expect("run cargo behave");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(stdout(&output).contains("ok parse/json"));
}

#[test]
fn behave_json_mismatch_reports_expected_path() {
    let root = temp_dir();
    let case = root.join("behavior/fixtures/parse/json-mismatch");
    write(
        &case.join("case.toml"),
        r#"
args = ["parse", "--json", "template.mustache"]
status = 0
stdout = "expected.stdout.json"
stdout_format = "json"
"#,
    );
    write(&case.join("template.mustache"), "Hello {{name}}\n");
    write(&case.join("expected.stdout.json"), r#"{"inputs":[]}"#);

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .output()
        .expect("run cargo behave");
    let stdout = stdout(&output);

    assert!(!output.status.success());
    assert!(stdout.contains("stdout JSON mismatch in"));
    assert!(stdout.contains("expected.stdout.json"));
}

#[test]
fn behave_update_writes_expected_outputs() {
    let root = temp_dir();
    let case = root.join("behavior/fixtures/parse/update");
    write(
        &case.join("case.toml"),
        r#"
args = ["parse", "template.mustache"]
status = 0
stdout = "expected.stdout"
stderr = "expected.stderr"
"#,
    );
    write(&case.join("template.mustache"), "Hello\n");

    let output = cargo_behave()
        .arg("--root")
        .arg(&root)
        .arg("--smoothe-bin")
        .arg(env!("CARGO_BIN_EXE_smoothe"))
        .arg("--update")
        .output()
        .expect("run cargo behave");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(case.join("expected.stdout").exists());
    assert!(case.join("expected.stderr").exists());
}
