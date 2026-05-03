use clap::ColorChoice;
use smoothe::config;

fn parse_config(source: &str) -> config::Configuration {
    toml::from_str(source).expect("parse config")
}

fn cli_options() -> config::CliGlobalOptions {
    config::CliGlobalOptions {
        color: None,
        no_color: false,
    }
}

fn resolve_config(configuration: &config::Configuration) -> config::ResolvedOptions {
    config::resolve_with_environment(
        configuration,
        &cli_options(),
        config::EnvironmentOptions { nocolor: false },
    )
    .expect("resolve config")
}

#[test]
fn color_options_resolve_by_precedence() {
    let cases = [
        (
            "defaults resolve to auto",
            "",
            None,
            false,
            false,
            ColorChoice::Auto,
        ),
        (
            "config overrides built-in default",
            "[options]\ncolor = false\n",
            None,
            false,
            false,
            ColorChoice::Never,
        ),
        (
            "environment overrides config",
            "[options]\ncolor = true\n",
            None,
            false,
            true,
            ColorChoice::Never,
        ),
        (
            "cli color overrides environment",
            "[options]\ncolor = false\n",
            Some(ColorChoice::Always),
            false,
            true,
            ColorChoice::Always,
        ),
        (
            "cli no-color overrides cli color",
            "[options]\ncolor = \"always\"\n",
            Some(ColorChoice::Always),
            true,
            false,
            ColorChoice::Never,
        ),
    ];

    for (name, source, cli_color, cli_no_color, env_nocolor, expected) in cases {
        let config = parse_config(source);
        let options = config::resolve_with_environment(
            &config,
            &config::CliGlobalOptions {
                color: cli_color,
                no_color: cli_no_color,
            },
            config::EnvironmentOptions {
                nocolor: env_nocolor,
            },
        )
        .expect("resolve config");

        assert_eq!(options.global.color, expected, "{name}");
    }
}

#[test]
fn check_semantic_inputs_default_to_disabled() {
    let config = parse_config("");
    let options = resolve_config(&config);

    assert_eq!(options.check.schema, config::SemanticInput::Disabled);
    assert_eq!(options.check.lambdas, config::SemanticInput::Disabled);
}

#[test]
fn check_semantic_inputs_accept_none_case_insensitively() {
    let config = parse_config("[check]\nschema = \"NONE\"\nlambdas = \"None\"\n");
    let options = resolve_config(&config);

    assert_eq!(options.check.schema, config::SemanticInput::Disabled);
    assert_eq!(options.check.lambdas, config::SemanticInput::Disabled);
}

#[test]
fn check_inputs_resolve_relative_to_config_file() {
    let project_root = std::env::current_dir().expect("current dir");
    let config = config::load(Some(std::path::Path::new(
        "tests/fixtures/smoothe-check.toml",
    )))
    .expect("load config");
    let options = resolve_config(&config);

    assert_eq!(
        options.check.schema,
        config::SemanticInput::Path(project_root.join("tests/fixtures/schemas/context.json"))
    );
    assert_eq!(
        options.check.lambdas,
        config::SemanticInput::Path(project_root.join("tests/fixtures/lambdas/known.json"))
    );
    assert_eq!(options.check.partials.len(), 1);
    assert_eq!(options.check.partials[0].name, "header");
    assert_eq!(
        options.check.partials[0].path,
        project_root.join("tests/fixtures/partials/_header.mustache")
    );
}

#[test]
fn config_partial_paths_apply_underscore_basename_convention() {
    let project_root = std::env::current_dir().expect("current dir");
    let config = config::load(Some(std::path::Path::new(
        "tests/fixtures/smoothe-check.toml",
    )))
    .expect("load config");
    let options = resolve_config(&config);

    assert_eq!(
        options.check.partials[0].path,
        project_root.join("tests/fixtures/partials/_header.mustache")
    );
}

#[test]
fn check_output_and_verbosity_defaults_are_configurable() {
    let config = parse_config("[check]\noutput = \"json\"\nverbosity = \"debug\"\n");
    let options = resolve_config(&config);

    assert_eq!(options.check.output, config::CheckOutputFormat::Json);
    assert_eq!(options.check.verbosity, config::CheckVerbosity::Debug);
}

#[test]
fn check_verbosity_accepts_trace() {
    let config = parse_config("[check]\nverbosity = \"trace\"\n");
    let options = resolve_config(&config);

    assert_eq!(options.check.verbosity, config::CheckVerbosity::Trace);
}
