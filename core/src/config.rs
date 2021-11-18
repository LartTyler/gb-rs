use crate::DeviceMode;
use clap::{App, Arg, ArgMatches};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIGURATION: Config = Config::new();
}

pub struct Config {
    pub mode: Option<DeviceMode>,
    pub verbose: bool,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self::default();
        let args = Self::get_cli_options();

        if args.is_present("verbose") {
            config.verbose = true;
        }

        if let Some(mode) = args.value_of("mode") {
            match mode {
                "color" => config.mode = Some(DeviceMode::Color),
                "classic" => config.mode = Some(DeviceMode::Classic),
                "detect" | "auto" => config.mode = None,
                _ => panic!("Unrecognized mode: {}", mode),
            };
        }

        config
    }

    fn get_cli_options() -> ArgMatches<'static> {
        App::new("gb-rs")
            .version("0.1.0")
            .author("Tyler Lartonoix <tyler@lartonoix.com>")
            .arg(
                Arg::with_name("mode")
                    .short("m")
                    .value_name("MODE")
                    .possible_values(&["color", "classic", "detect", "auto"]),
            )
            .arg(
                Arg::with_name("v")
                    .short("v")
                    .multiple(true)
                    .help("Sets the level of verbosity"),
            )
            .get_matches()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: None,
            verbose: false,
        }
    }
}
