use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("json")
                .help("Print weather as json")
                .long("json"),
        )
        .arg(
            Arg::with_name("pretty-json")
                .help("Print weather as json, indented for readability.")
                .long("pretty-json"),
        )
        .arg(
            Arg::with_name("format")
                .help("The format the weather will be displayed in.")
                .short("f")
                .long("format")
                .min_values(1)
                .case_insensitive(true)
                .possible_values(&OutputStyle::variants()),
        )
        .arg(
            Arg::with_name("current")
                .help("Print current conditions")
                .short("c")
                .long("current"),
        )
        .arg(
            Arg::with_name("historical")
                .help("Print historical conditions")
                .long("historical")
                .takes_value(true)
                .validator(|time| match time.parse::<u64>() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }),
        )
        .arg(
            Arg::with_name("api_key")
                .required(true)
                .env("DARKSKY_API_KEY")
                .hide_env_values(true),
        )
        .arg(
            Arg::with_name("latitude")
                .required(true)
                .env("DARKSKY_LATITUDE")
                .hide_env_values(true)
                .validator(validate_coordinate),
        )
        .arg(
            Arg::with_name("longitude")
                .required(true)
                .env("DARKSKY_LONGITUDE")
                .hide_env_values(true)
                .validator(validate_coordinate),
        )
}

// This is the signature used by the validator() function.
#[allow(clippy::needless_pass_by_value)]
fn validate_coordinate(coord: String) -> Result<(), String> {
    match coord.parse::<f64>() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub latitude: f64,
    pub longitude: f64,
    pub historical_time: Option<u64>,
    pub output_style: OutputStyle,
}

impl Config {
    pub fn parse_args(m: &clap::ArgMatches) -> Result<Self, Box<std::error::Error>> {
        let api_key = value_t!(m.value_of("api_key"), String)?;
        let latitude = value_t!(m.value_of("latitude"), f64)?;
        let longitude = value_t!(m.value_of("longitude"), f64)?;
        let historical_time = match m.value_of("historical") {
            Some(time) => Some(time.parse::<u64>()?),
            None => None,
        };
        let output_style = value_t!(m.value_of("format"), OutputStyle).unwrap_or_default();

        Ok(Self {
            api_key,
            latitude,
            longitude,
            historical_time,
            output_style,
        })
    }
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum OutputStyle {
        Parsed,
        Json,
        PrettyJson,
    }
}

impl Default for OutputStyle {
    fn default() -> Self {
        OutputStyle::Parsed
    }
}
