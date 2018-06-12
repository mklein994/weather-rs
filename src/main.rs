#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate weather_rs;

mod app;

fn main() {
    let (api_key, latitude, longitude) = get_api_params();

    let m = app::build_cli().get_matches();

    let weather = weather_rs::get_weather(api_key, latitude, longitude);

    match weather {
        Ok(w) => {
            if m.is_present("json") {
                let pretty_print = m.value_of("json").is_some();
                weather_rs::print_json(w, pretty_print)
            } else {
                weather_rs::print_weather(w);
            }
        }
        Err(err) => eprintln!("{:?}", err),
    }
}

/// Get parameters needed to print weather from the environment.
///
/// Searches a file named `.env` for DARKSKY_API_KEY, DARKSKY_LATITUDE and DARKSKY_LATITUDE.
pub fn get_api_params() -> (String, f64, f64) {
    dotenv::dotenv().ok();

    let api_key = dotenv::var("DARKSKY_API_KEY").expect("Missing DARKSKY_API_KEY");

    let latitude = dotenv::var("DARKSKY_LATITUDE")
        .map(|lat| lat.parse::<f64>().expect("Couldn't parse latitude as f64"))
        .expect("Missing DARKSKY_API_KEY");

    let longitude = dotenv::var("DARKSKY_LONGITUDE")
        .map(|lat| lat.parse::<f64>().expect("Couldn't parse longitude as f64"))
        .expect("Missing DARKSKY_LONGITUDE");

    (api_key, latitude, longitude)
}
