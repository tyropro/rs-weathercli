use weatherapi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let api_key: String = std::env::var("API_KEY")?;
    let location: String = std::env::var("LOCATION")?;

    let weatherapi = weatherapi::WeatherAPI::new(&api_key, &location);

    let weatherapi_response = weatherapi.fetch()?;

    println!(
        "Location:\n  Name: {}\n  Region: {} \n  Country: {}\n",
        weatherapi_response.location().name(),
        weatherapi_response.location().region(),
        weatherapi_response.location().country()
    );

    let current_weather = weatherapi_response.current();

    println!(
        "Current Weather:\n  Temperature (C): {}\n  Temperature (F): {}\n  Feels Like (C): {}\n  Feels Like (F): {}\n  Wind (mph): {}\n  Wind (km/h): {}\n  Wind Direction: {}\n  Condition: {}\n  Pressure (mb): {}\n  Pressure (in): {}",
        current_weather.temp_c(), current_weather.temp_f(), current_weather.feelslike_c(), current_weather.feelslike_f(), current_weather.wind_mph(), current_weather.wind_kph(), current_weather.wind_dir(), current_weather.condition().text(), current_weather.pressure_mb(), current_weather.pressure_in()
    );

    Ok(())
}
