extern crate dotenv;
extern crate weather_rs;

fn main() {
    dotenv::dotenv().ok();

    let api_key = dotenv::var("DARKSKY_API_KEY").expect("Missing DARKSKY_API_KEY");

    weather_rs::get_weather(api_key);

    println!("Hello, world!");
}
