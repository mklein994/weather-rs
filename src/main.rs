#[macro_use]
extern crate clap;
extern crate weather_rs;

mod app;

use weather_rs::ApiParams;

fn main() {
    let api_params = ApiParams::get_api_params();

    let m = app::build_cli().get_matches();

    if let Err(e) = weather_rs::run(api_params, &m) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
