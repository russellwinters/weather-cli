use dotenv::dotenv;
use reqwest;
use std::error::Error;

mod common;
mod weather;

use common::{load_env_var, Args};
use weather::{fetch_data, transform_response, WeatherData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key_var: String = load_env_var("WEATHER_API_KEY").expect("Unable to get API key");
    let city: String = Args::get_city();

    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key_var
    );

    // GET request
    let response: reqwest::Response = fetch_data(&url).await.expect("Failed to fetch API Data");

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .expect("Error transporming response to text");

        let data: WeatherData = transform_response(&body).await.expect("Error parsing data");
        data.print_weather();
    } else {
        println!("Failed to call API: {}", response.status())
    }
    Ok(())
}
