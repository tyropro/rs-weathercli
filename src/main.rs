use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherData {
    location: WeatherLocation,
    current: WeatherCurrent,
}

#[derive(Deserialize, Debug)]
struct WeatherLocation {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherCurrent {
    temp_c: f32,
    temp_f: f32,
    feelslike_c: f32,
    feelslike_f: f32,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: f32,
    wind_dir: String,
    condition: WeatherCondition,
    pressure_mb: f32,
    pressure_in: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherCondition {
    text: String,
    icon: String,
}

fn get_data(url: &str) -> Result<WeatherData, Box<dyn Error>> {
    // get the response from url
    let resp = ureq::get(url).call()?.into_string()?;

    // dbg!(&resp);

    // parse the response into a WeatherData struct
    let weather_data: WeatherData = serde_json::from_str(&resp)?;

    Ok(weather_data)
}

fn main() -> Result<(), Box<dyn Error>> {
    // loads .env file
    dotenv::dotenv().ok();

    // loads api key and location from .env
    let api_key = std::env::var("API_KEY")?;
    let location = std::env::var("LOCATION")?;

    // build the url with api key and location
    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        api_key, location,
    );

    // get the response from url
    let weather_data = get_data(&url)?;

    // print the response as debug
    println!("{:?}", weather_data);

    Ok(())
}
