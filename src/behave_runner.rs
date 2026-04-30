use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs, io,
    path::{Component, Path, PathBuf},
    process::Command,
};

use serde::Deserialize;

#[derive(Debug)]
pub struct RunnerOptions {
    pub root: PathBuf,
    pub smoothe_bin: Option<PathBuf>,
    pub list: bool,
    pub filters: Vec<String>,
    pub update: bool,
}

#[derive(Debug)]
struct Case {
    id: String,
    dir: PathBuf,
    manifest: CaseManifest,
}

#[derive(Debug, Deserialize)]
struct CaseManifest {
    args: Vec<String>,
    #[serde(default)]
    config: Option<PathBuf>,
    #[serde(default)]
    status: i32,
    #[serde(default)]
    stdout: Option<PathBuf>,
    #[serde(default)]
    stderr: Option<PathBuf>,
    #[serde(default)]
    stdout_format: OutputFormat,
    #[serde(default)]
    stderr_format: OutputFormat,
    #[serde(default)]
    env: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Debug)]
struct CaseResult {
    id: String,
    passed: bool,
    messages: Vec<String>,
}

pub fn run_from_args() -> Result<bool, String> {
    run_from_args_os(std::env::args_os().skip(1))
}

pub fn run_from_args_os(args: impl IntoIterator<Item = OsString>) -> Result<bool, String> {
    match parse_args(args)? {
        ParsedArgs::Run(args) => run(args),
        ParsedArgs::Done => Ok(true),
    }
}

enum ParsedArgs {
    Run(RunnerOptions),
    Done,
}

pub fn run(args: RunnerOptions) -> Result<bool, String> {
    let fixture_root = args.root.join("behavior").join("fixtures");
    if !args.list && !fixture_root.is_dir() {
        return Err(format!(
            "fixture root not found: {}",
            fixture_root.display()
        ));
    }
    let cases = discover_cases(&fixture_root, &args.filters)?;

    if args.list {
        for case in cases {
            println!("{}", case.id);
        }
        return Ok(true);
    }

    if cases.is_empty() {
        return Err("no behavior cases matched".to_owned());
    }

    let smoothe_bin = args
        .smoothe_bin
        .unwrap_or_else(default_smoothe_bin)
        .canonicalize()
        .map_err(|error| format!("resolve smoothe binary: {error}"))?;

    let mut passed = 0;
    let mut failed = 0;

    for case in cases {
        let result = run_case(&case, &smoothe_bin, args.update)?;
        if result.passed {
            passed += 1;
            println!("ok {}", result.id);
        } else {
            failed += 1;
            println!("not ok {}", result.id);
            for message in result.messages {
                println!("  {message}");
            }
        }
    }

    println!("{passed} passed; {failed} failed");
    Ok(failed == 0)
}

fn parse_args(args: impl IntoIterator<Item = OsString>) -> Result<ParsedArgs, String> {
    let mut parsed = RunnerOptions {
        root: PathBuf::from("."),
        smoothe_bin: None,
        list: false,
        filters: Vec::new(),
        update: false,
    };

    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.to_string_lossy().as_ref() {
            "--root" => {
                parsed.root = next_path_arg(&mut args, "--root")?;
            }
            "--smoothe-bin" => {
                parsed.smoothe_bin = Some(next_path_arg(&mut args, "--smoothe-bin")?);
            }
            "--list" => {
                parsed.list = true;
            }
            "--filter" => {
                parsed.filters.push(next_string_arg(&mut args, "--filter")?);
            }
            "--update" => {
                parsed.update = true;
            }
            "--help" | "-h" => {
                print_help();
                return Ok(ParsedArgs::Done);
            }
            "--version" | "-V" => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                return Ok(ParsedArgs::Done);
            }
            other if other.starts_with('-') => {
                return Err(format!("unknown argument `{other}`"));
            }
            _ => {
                parsed.filters.push(
                    arg.into_string()
                        .map_err(|_| "filter arguments must be UTF-8".to_owned())?,
                );
            }
        }
    }

    Ok(ParsedArgs::Run(parsed))
}

fn next_path_arg(
    args: &mut impl Iterator<Item = OsString>,
    option: &str,
) -> Result<PathBuf, String> {
    args.next()
        .map(PathBuf::from)
        .ok_or_else(|| format!("{option} requires a path"))
}

fn next_string_arg(
    args: &mut impl Iterator<Item = OsString>,
    option: &str,
) -> Result<String, String> {
    args.next()
        .and_then(|value| value.into_string().ok())
        .ok_or_else(|| format!("{option} requires a UTF-8 value"))
}

fn print_help() {
    println!(
        "Usage: cargo behave [OPTIONS] [FILTER]...\n\nArguments:\n  [FILTER]...          Run/list case ids containing any filter\n\nOptions:\n      --root <PATH>\n      --smoothe-bin <PATH>\n      --list\n      --filter <PATTERN>\n      --update\n  -h, --help\n  -V, --version"
    );
}

fn discover_cases(fixture_root: &Path, filters: &[String]) -> Result<Vec<Case>, String> {
    let mut manifests = Vec::new();
    collect_case_manifests(fixture_root, &mut manifests)
        .map_err(|error| format!("discover fixtures: {error}"))?;
    manifests.sort();

    let mut cases = Vec::new();
    for manifest_path in manifests {
        let dir = manifest_path
            .parent()
            .ok_or_else(|| format!("case manifest has no parent: {}", manifest_path.display()))?
            .to_owned();
        let id = dir
            .strip_prefix(fixture_root)
            .map_err(|error| format!("build case id for {}: {error}", dir.display()))?
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");

        if !filters.is_empty() && !filters.iter().any(|filter| id.contains(filter)) {
            continue;
        }

        let source = fs::read_to_string(&manifest_path)
            .map_err(|error| format!("read {}: {error}", manifest_path.display()))?;
        let manifest: CaseManifest = toml::from_str(&source)
            .map_err(|error| format!("parse {}: {error}", manifest_path.display()))?;
        validate_manifest_paths(&id, &manifest)?;
        if manifest.config.is_none() && dir.join("smoothe.toml").exists() {
            return Err(format!(
                "case {id} contains smoothe.toml but does not set `config` in case.toml"
            ));
        }
        cases.push(Case { id, dir, manifest });
    }

    Ok(cases)
}

fn validate_manifest_paths(id: &str, manifest: &CaseManifest) -> Result<(), String> {
    validate_optional_case_path(id, "config", manifest.config.as_deref())?;
    validate_optional_case_path(id, "stdout", manifest.stdout.as_deref())?;
    validate_optional_case_path(id, "stderr", manifest.stderr.as_deref())
}

fn validate_optional_case_path(id: &str, field: &str, path: Option<&Path>) -> Result<(), String> {
    let Some(path) = path else {
        return Ok(());
    };

    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(format!(
            "case {id} `{field}` path must stay within the case directory"
        ));
    }

    Ok(())
}

fn collect_case_manifests(dir: &Path, manifests: &mut Vec<PathBuf>) -> io::Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_case_manifests(&path, manifests)?;
        } else if path.file_name().is_some_and(|name| name == "case.toml") {
            manifests.push(path);
        }
    }

    Ok(())
}

fn run_case(case: &Case, smoothe_bin: &Path, update: bool) -> Result<CaseResult, String> {
    let mut command = Command::new(smoothe_bin);
    command.current_dir(&case.dir);
    command.env_remove("NOCOLOR");
    command.env("HOME", &case.dir);
    command.env("XDG_CONFIG_HOME", case.dir.join(".xdg-config"));
    for (key, value) in &case.manifest.env {
        command.env(key, value);
    }

    if let Some(config) = &case.manifest.config {
        command.arg("--config").arg(config);
    }
    command.args(&case.manifest.args);

    let output = command
        .output()
        .map_err(|error| format!("run {}: {error}", case.id))?;

    let mut messages = Vec::new();
    let status = output.status.code().unwrap_or(1);
    if status != case.manifest.status {
        messages.push(format!(
            "status mismatch: expected {}, got {status}",
            case.manifest.status
        ));
    }

    compare_stream(
        "stdout",
        &case.dir,
        case.manifest.stdout.as_deref(),
        case.manifest.stdout_format,
        &output.stdout,
        update,
        &mut messages,
    )?;
    compare_stream(
        "stderr",
        &case.dir,
        case.manifest.stderr.as_deref(),
        case.manifest.stderr_format,
        &output.stderr,
        update,
        &mut messages,
    )?;

    Ok(CaseResult {
        id: case.id.clone(),
        passed: messages.is_empty(),
        messages,
    })
}

fn compare_stream(
    name: &str,
    case_dir: &Path,
    expected_path: Option<&Path>,
    format: OutputFormat,
    actual: &[u8],
    update: bool,
    messages: &mut Vec<String>,
) -> Result<(), String> {
    let Some(expected_path) = expected_path else {
        if !actual.is_empty() {
            messages.push(format!(
                "{name} mismatch: expected empty output, got {} bytes",
                actual.len()
            ));
        }
        return Ok(());
    };

    let expected_path = case_dir.join(expected_path);
    let actual_text = String::from_utf8(actual.to_vec())
        .map_err(|error| format!("{name} is not utf-8: {error}"))?;
    let actual_normalized = normalize_text(&actual_text, case_dir);

    if update {
        let output = match format {
            OutputFormat::Text => actual_normalized,
            OutputFormat::Json => pretty_json(name, &actual_text)?,
        };
        fs::write(&expected_path, output)
            .map_err(|error| format!("write {}: {error}", expected_path.display()))?;
        return Ok(());
    }

    let expected = fs::read_to_string(&expected_path)
        .map_err(|error| format!("read {}: {error}", expected_path.display()))?;
    match format {
        OutputFormat::Text => {
            let expected_normalized = normalize_text(&expected, case_dir);
            if actual_normalized != expected_normalized {
                messages.push(format!(
                    "{name} mismatch in {}: {}",
                    expected_path.display(),
                    first_text_difference(&expected_normalized, &actual_normalized)
                ));
            }
        }
        OutputFormat::Json => {
            let actual_json: serde_json::Value = serde_json::from_str(&actual_text)
                .map_err(|error| format!("parse actual {name} JSON: {error}"))?;
            let expected_json: serde_json::Value = serde_json::from_str(&expected)
                .map_err(|error| format!("parse expected {name} JSON: {error}"))?;
            if actual_json != expected_json {
                messages.push(format!(
                    "{name} JSON mismatch in {}",
                    expected_path.display()
                ));
            }
        }
    }

    Ok(())
}

fn normalize_text(source: &str, case_dir: &Path) -> String {
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    normalized.replace(&case_dir.to_string_lossy().into_owned(), "<case>")
}

fn first_text_difference(expected: &str, actual: &str) -> String {
    let mut expected_lines = expected.split('\n');
    let mut actual_lines = actual.split('\n');
    for line_number in 1.. {
        match (expected_lines.next(), actual_lines.next()) {
            (Some(expected), Some(actual)) if expected == actual => {}
            (Some(expected), Some(actual)) => {
                return format!(
                    "line {line_number}: expected {}, got {}",
                    quoted_line(expected),
                    quoted_line(actual)
                );
            }
            (Some(expected), None) => {
                return format!(
                    "line {line_number}: expected {}, got <end of output>",
                    quoted_line(expected)
                );
            }
            (None, Some(actual)) => {
                return format!(
                    "line {line_number}: expected <end of output>, got {}",
                    quoted_line(actual)
                );
            }
            (None, None) => return "outputs differ".to_owned(),
        }
    }
    unreachable!("unbounded loop returns on exhausted iterators")
}

fn quoted_line(line: &str) -> String {
    const MAX_CHARS: usize = 120;
    let mut output = line.chars().take(MAX_CHARS).collect::<String>();
    if line.chars().count() > MAX_CHARS {
        output.push_str("...");
    }
    format!("{output:?}")
}

fn pretty_json(name: &str, source: &str) -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(source).map_err(|error| format!("parse {name} JSON: {error}"))?;
    serde_json::to_string_pretty(&value)
        .map(|mut output| {
            output.push('\n');
            output
        })
        .map_err(|error| format!("serialize {name} JSON: {error}"))
}

fn default_smoothe_bin() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_owned))
        .map(|dir| dir.join(smoothe_exe_name()))
        .unwrap_or_else(|| PathBuf::from(smoothe_exe_name()))
}

fn smoothe_exe_name() -> OsString {
    let mut name = OsString::from("smoothe");
    name.push(std::env::consts::EXE_SUFFIX);
    name
}
