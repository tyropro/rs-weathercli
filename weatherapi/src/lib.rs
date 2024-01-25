use serde::Deserialize;

// base url for api
const BASE_URL: &str = "https://api.weatherapi.com/v1/current.json";

#[derive(thiserror::Error, Debug)]
/// The Error enum represents all possible error cases that can occur when
/// interacting with the API. This provides a clean way to handle errors in
/// a structured way.
pub enum Error {
    #[error("Url parsing failed")]
    UrlParsing(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    #[error("Failed fetching articles")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),
    #[error("Data parsing failed")]
    DataParseFailed(#[from] serde_json::Error),
}

#[derive(Deserialize, Debug)]
/// Response contains the location and current weather data from the API
pub struct Response {
    location: Location,
    current: Current,
}

/// Getters for the `location` and `current` fields of the `Response` struct.
///
/// Returns a reference to the `Location` struct containing location data.
///
/// Returns a reference to the `Current` struct containing current weather data.
impl Response {
    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn current(&self) -> &Current {
        &self.current
    }
}

#[derive(Deserialize, Debug)]
/// Response from weatherapi under json value `location`.
/// Contains location data
pub struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
}

/// Getters for the `Location` struct containing location data.
impl Location {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn lat(&self) -> f32 {
        self.lat
    }

    pub fn lon(&self) -> f32 {
        self.lon
    }
}

#[derive(Deserialize, Debug)]
/// Response from weatherapi under json value `current`.
/// Contains current weather data
pub struct Current {
    temp_c: f32,
    temp_f: f32,
    feelslike_c: f32,
    feelslike_f: f32,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: f32,
    wind_dir: String,
    condition: Condition,
    pressure_mb: f32,
    pressure_in: f32,
}

/// Provides getter methods for the various fields of the `Current` struct.
impl Current {
    pub fn temp_c(&self) -> f32 {
        self.temp_c
    }

    pub fn temp_f(&self) -> f32 {
        self.temp_f
    }

    pub fn feelslike_c(&self) -> f32 {
        self.feelslike_c
    }

    pub fn feelslike_f(&self) -> f32 {
        self.feelslike_f
    }

    pub fn wind_mph(&self) -> f32 {
        self.wind_mph
    }

    pub fn wind_kph(&self) -> f32 {
        self.wind_kph
    }

    pub fn wind_degree(&self) -> f32 {
        self.wind_degree
    }

    pub fn wind_dir(&self) -> &str {
        &self.wind_dir
    }

    pub fn condition(&self) -> &Condition {
        &self.condition
    }

    pub fn pressure_mb(&self) -> f32 {
        self.pressure_mb
    }

    pub fn pressure_in(&self) -> f32 {
        self.pressure_in
    }
}

#[derive(Deserialize, Debug)]
/// Condition represents the current weather condition
/// Contains the textual description of the weather condition and the name of an icon representing the weather condition.
pub struct Condition {
    text: String,
    icon: String,
}

/// Provides getter methods for the `text` and `icon` fields of a `Condition` struct.
impl Condition {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn icon(&self) -> &str {
        &self.icon
    }
}

pub struct WeatherAPI {
    api_key: String,
    location: String,
}

impl WeatherAPI {
    // initialiser for WeatherAPI
    // api_key & location required
    pub fn new(api_key: &str, location: &str) -> WeatherAPI {
        WeatherAPI {
            api_key: api_key.to_string(),
            location: location.to_string(),
        }
    }

    // prepare url for request
    fn prepare_url(&self) -> Result<String, Error> {
        let url: url::Url =
            url::Url::parse_with_params(BASE_URL, [("key", &self.api_key), ("q", &self.location)])?;

        Ok(url.to_string())
    }

    // perform fetch request
    pub fn fetch(&self) -> Result<Response, Error> {
        let url: String = self.prepare_url()?;
        let request: ureq::Request = ureq::get(&url);
        let response: ureq::Response = request.call()?;

        match response.status() {
            // if status code is 200, return response
            200 => {
                let json_response: Response = response.into_json()?;
                return Ok(json_response);
            }
            // if status code is not 200, find error code + return error
            _ => {
                let response_err: serde_json::Value = response.into_json()?;
                let code: String = response_err["error"]["code"].to_string();

                return Err(map_response_err(Some(code)));
            }
        }
    }
}

// error mapping
fn map_response_err(code: Option<String>) -> Error {
    if let Some(code) = code {
        match code.as_str() {
            "1002" => Error::BadRequest("API key not provided"),
            "1003" => Error::BadRequest("Parameter 'q' not provided"),
            "1005" => Error::BadRequest("API request url is invalid"),
            "1006" => Error::BadRequest("No location found matching parameter 'q'"), 
            "2006" => Error::BadRequest("API key provided is invalid"),
            "2007" => Error::BadRequest("API key has exceeded calls per month quota"),
            "2008" => Error::BadRequest("API key has been disabled"),
            "2009" => Error::BadRequest("API key does not have access to the resource. Please check pricing page for what is allowed in your API subscription plan"),
            "9000" => Error::BadRequest("Json body passed in bulk request is invalid. Please make sure it is valid json with utf-8 encoding"),
            "9001" => Error::BadRequest("Json body contains too many locations for bulk request. Please keep it below 50 in a single request"),
            "9999" => Error::BadRequest("Internal application error"),
            _ => Error::BadRequest("Unknown error"),
        }
    } else {
        Error::BadRequest("Unknown error")
    }
}
