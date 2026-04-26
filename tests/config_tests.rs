use clap::ColorChoice;
use smoothe::config;

fn parse_config(source: &str) -> config::Configuration {
    toml::from_str(source).expect("parse config")
}

#[test]
fn defaults_resolve_to_auto_color() {
    let config = parse_config("");
    let options = config::resolve_with_environment(
        &config,
        &config::CliGlobalOptions {
            color: None,
            no_color: false,
        },
        config::EnvironmentOptions { nocolor: false },
    );

    assert_eq!(options.global.color, ColorChoice::Auto);
}

#[test]
fn config_overrides_built_in_color_default() {
    let config = parse_config("[options]\ncolor = false\n");
    let options = config::resolve_with_environment(
        &config,
        &config::CliGlobalOptions {
            color: None,
            no_color: false,
        },
        config::EnvironmentOptions { nocolor: false },
    );

    assert_eq!(options.global.color, ColorChoice::Never);
}

#[test]
fn environment_overrides_config_color() {
    let config = parse_config("[options]\ncolor = true\n");
    let options = config::resolve_with_environment(
        &config,
        &config::CliGlobalOptions {
            color: None,
            no_color: false,
        },
        config::EnvironmentOptions { nocolor: true },
    );

    assert_eq!(options.global.color, ColorChoice::Never);
}

#[test]
fn cli_color_overrides_environment_color() {
    let config = parse_config("[options]\ncolor = false\n");
    let options = config::resolve_with_environment(
        &config,
        &config::CliGlobalOptions {
            color: Some(ColorChoice::Always),
            no_color: false,
        },
        config::EnvironmentOptions { nocolor: true },
    );

    assert_eq!(options.global.color, ColorChoice::Always);
}

#[test]
fn cli_no_color_overrides_cli_color() {
    let config = parse_config("[options]\ncolor = \"always\"\n");
    let options = config::resolve_with_environment(
        &config,
        &config::CliGlobalOptions {
            color: Some(ColorChoice::Always),
            no_color: true,
        },
        config::EnvironmentOptions { nocolor: false },
    );

    assert_eq!(options.global.color, ColorChoice::Never);
}
