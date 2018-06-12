extern crate clap;
extern crate darksky;
extern crate reqwest;

use darksky::DarkskyReqwestRequester;
use reqwest::Client;

pub fn get_weather(
    api_key: String,
    lat: f64,
    long: f64,
) -> Result<darksky::models::Forecast, darksky::Error> {
    let client = Client::new();

    client.get_forecast(&api_key, lat, long)
}

pub fn print_weather(weather: darksky::models::Forecast) {
    println!("{:#?}", weather);
}
