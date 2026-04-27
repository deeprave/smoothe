use clap::ColorChoice;
use smoothe::config;

fn parse_config(source: &str) -> config::Configuration {
    toml::from_str(source).expect("parse config")
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
