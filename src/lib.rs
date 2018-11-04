#[macro_use]
extern crate clap;
extern crate darksky;
extern crate reqwest;
extern crate serde_json;

pub mod app;

use app::{Config, OutputStyle};
use darksky::models::Forecast;
use darksky::DarkskyReqwestRequester;
use reqwest::Client;
use std::error::Error;

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let config = Config::parse_args(&matches)?;
    let weather = get_weather(&config)?;

    print_weather(&weather, &config.output_style)?;

    Ok(())
}

fn get_weather(c: &Config) -> Result<Forecast, darksky::Error> {
    let client = Client::new();

    if c.historical_time.is_some() {
        client.get_forecast_time_machine(
            &c.api_key,
            c.latitude,
            c.longitude,
            c.historical_time.unwrap(),
            |o| o,
        )
    } else {
        client.get_forecast(&c.api_key, c.latitude, c.longitude)
    }
}

fn print_weather(weather: &Forecast, format: &OutputStyle) -> Result<(), serde_json::Error> {
    match format {
        OutputStyle::Parsed => println!("{:#?}", weather),
        OutputStyle::Json => println!("{}", serde_json::to_string(&weather)?),
        OutputStyle::PrettyJson => println!("{}", serde_json::to_string_pretty(&weather)?),
    };

    Ok(())
}
