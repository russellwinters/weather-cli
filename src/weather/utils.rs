use super::WeatherData;

pub async fn fetch_data(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let response: Result<reqwest::Response, reqwest::Error> = reqwest::get(url).await;

    match response {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("Error Fetching Weather: {}", e);
            Err(e)
        }
    }
}

pub async fn transform_response(data: &str) -> Result<WeatherData, serde_json::Error> {
    let result: Result<WeatherData, serde_json::Error> = serde_json::from_str(data);

    match result {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("Error transorming weather data to struct: {}", e);
            Err(e)
        }
    }
}
