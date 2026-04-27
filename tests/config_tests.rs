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
        );

        assert_eq!(options.global.color, expected, "{name}");
    }
}

#[test]
fn check_semantic_inputs_default_to_disabled() {
    let config = parse_config("");
    let options = config::resolve_with_environment(
        &config,
        &cli_options(),
        config::EnvironmentOptions { nocolor: false },
    );

    assert_eq!(options.check.schema, config::SemanticInput::Disabled);
    assert_eq!(options.check.lambdas, config::SemanticInput::Disabled);
}

#[test]
fn check_semantic_inputs_accept_none_case_insensitively() {
    let config = parse_config("[check]\nschema = \"NONE\"\nlambdas = \"None\"\n");
    let options = config::resolve_with_environment(
        &config,
        &cli_options(),
        config::EnvironmentOptions { nocolor: false },
    );

    assert_eq!(options.check.schema, config::SemanticInput::Disabled);
    assert_eq!(options.check.lambdas, config::SemanticInput::Disabled);
}

#[test]
fn check_semantic_input_paths_resolve_relative_to_config_file() {
    let config = config::load(Some(std::path::Path::new(
        "tests/fixtures/smoothe-check.toml",
    )))
    .expect("load config");
    let options = config::resolve_with_environment(
        &config,
        &cli_options(),
        config::EnvironmentOptions { nocolor: false },
    );

    assert_eq!(
        options.check.schema,
        config::SemanticInput::Path("tests/fixtures/schemas/context.json".into())
    );
    assert_eq!(
        options.check.lambdas,
        config::SemanticInput::Path("tests/fixtures/lambdas/known.json".into())
    );
}
