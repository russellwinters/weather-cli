use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Coordinates {
    lat: f64,
    lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct LocalWeather {
    feels_like: f64,
    temp: f64,
    temp_max: f64,
    temp_min: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    country: String,
    id: u64,
    sunrise: u64,
    sunset: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Conditions {
    description: String,
    main: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    id: u64,
    dt: u64,
    coord: Coordinates,
    main: LocalWeather,
    name: String,
    sys: Sys,
    weather: Vec<Conditions>,
    wind: Wind,
}
