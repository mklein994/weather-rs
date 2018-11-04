extern crate dotenv;
extern crate weather_rs;

use weather_rs::app;

fn main() {
    dotenv::dotenv().ok();

    let matches = app::build_cli().get_matches();

    if let Err(e) = weather_rs::run(&matches) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
