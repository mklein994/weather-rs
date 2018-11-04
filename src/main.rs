#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate weather_rs;

mod app;

use app::OutputStyle;
use weather_rs::ApiParams;

fn main() {
    let api_params = ApiParams::get_api_params();

    dotenv::dotenv().ok();
    let matches = app::build_cli().get_matches();

    if let Err(e) = parse_args(&matches) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    if let Err(e) = weather_rs::run(api_params, &matches) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[derive(Debug)]
struct Config {
    api_key: String,
    latitude: f64,
    longitude: f64,
    historical_time: Option<u64>,
    output_style: app::OutputStyle,
}

fn parse_args(m: &clap::ArgMatches) -> Result<Config, Box<std::error::Error>> {
    let api_key = value_t!(m.value_of("api_key"), String)?;
    let latitude = value_t!(m.value_of("latitude"), f64)?;
    let longitude = value_t!(m.value_of("longitude"), f64)?;
    let historical_time = match m.value_of("historical") {
        Some(time) => Some(time.parse::<u64>()?),
        None => None,
    };
    let output_style = value_t!(m.value_of("format"), OutputStyle).unwrap_or_default();

    Ok(Config {
        api_key,
        latitude,
        longitude,
        historical_time,
        output_style,
    })
}
