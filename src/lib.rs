#[macro_use]
extern crate clap;
extern crate darksky;
extern crate reqwest;
extern crate serde_json;

use darksky::models::Forecast;
use darksky::DarkskyReqwestRequester;
use reqwest::Client;
use std::error::Error;

pub fn run(api_params: ApiParams, m: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let historical_time = parse_time(&m)?;

    let weather = get_weather(api_params, historical_time)?;

    if m.is_present("json") {
        let pretty_print = m.value_of("json").is_some();
        print_json(&weather, pretty_print)
    } else {
        print_weather(&weather);
    }

    Ok(())
}

fn parse_time(m: &clap::ArgMatches) -> Result<Option<u64>, clap::Error> {
    if m.value_of("historical").is_some() {
        value_t!(m.value_of("historical"), u64).map(Some)
    } else {
        Ok(None)
    }
}

pub fn get_weather(
    api_params: ApiParams,
    historical_time: Option<u64>,
) -> Result<Forecast, darksky::Error> {
    let ApiParams {
        api_key: key,
        latitude: lat,
        longitude: lon,
    } = api_params;

    let client = Client::new();

    if historical_time.is_some() {
        client.get_forecast_time_machine(&key, lat, lon, historical_time.unwrap(), |o| o)
    } else {
        client.get_forecast(&key, lat, lon)
    }
}

pub fn print_weather(weather: &Forecast) {
    println!("{:#?}", weather);
}

pub fn print_json(weather: &Forecast, pretty: bool) {
    println!(
        "{}",
        if pretty {
            serde_json::to_string_pretty(&weather).expect("Couldn't pretty print weather as json")
        } else {
            serde_json::to_string(&weather).expect("Couldn't print weather as json")
        }
    );
}

#[derive(Debug)]
pub struct ApiParams {
    api_key: String,
    latitude: f64,
    longitude: f64,
}

impl ApiParams {
    /// Get parameters needed to print weather from the environment.
    ///
    /// Searches a file named `.env` for DARKSKY_API_KEY, DARKSKY_LATITUDE and DARKSKY_LATITUDE.
    pub fn get_api_params() -> Self {
        dotenv::dotenv().ok();

        let api_key = dotenv::var("DARKSKY_API_KEY").expect("Missing DARKSKY_API_KEY");

        let latitude = dotenv::var("DARKSKY_LATITUDE")
            .map(|lat| lat.parse::<f64>().expect("Couldn't parse latitude as f64"))
            .expect("Missing DARKSKY_API_KEY");

        let longitude = dotenv::var("DARKSKY_LONGITUDE")
            .map(|lat| lat.parse::<f64>().expect("Couldn't parse longitude as f64"))
            .expect("Missing DARKSKY_LONGITUDE");

        Self {
            api_key,
            latitude,
            longitude,
        }
    }
}
