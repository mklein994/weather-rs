extern crate clap;
extern crate darksky;
extern crate reqwest;
extern crate serde_json;

use darksky::models::Forecast;
use darksky::DarkskyReqwestRequester;
use reqwest::Client;

pub fn get_weather(api_key: &str, lat: f64, long: f64) -> Result<Forecast, darksky::Error> {
    let client = Client::new();

    client.get_forecast(&api_key, lat, long)
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
