use serde::Deserialize;

const BASE_URL: &str = "https://api.weatherapi.com/v1/current.json";

#[derive(thiserror::Error, Debug)]
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
pub struct Response {
    location: Location,
    current: Current,
}

impl Response {
    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn current(&self) -> &Current {
        &self.current
    }
}

#[derive(Deserialize, Debug)]
pub struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
}

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
pub struct Condition {
    text: String,
    icon: String,
}

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
    pub fn new(api_key: &str, location: &str) -> WeatherAPI {
        WeatherAPI {
            api_key: api_key.to_string(),
            location: location.to_string(),
        }
    }

    fn prepare_url(&self) -> Result<String, Error> {
        let mut url: url::Url = url::Url::parse(BASE_URL)?;
        url.query_pairs_mut()
            .append_pair("key", &self.api_key)
            .append_pair("q", &self.location);

        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<Response, Error> {
        let url: String = self.prepare_url()?;
        let request: ureq::Request = ureq::get(&url);
        let response: ureq::Response = request.call()?;

        match response.status() {
            200 => {
                let json_response: Response = response.into_json()?;
                return Ok(json_response);
            }
            _ => {
                let response_err: serde_json::Value = response.into_json()?;
                let code: String = response_err["error"]["code"].to_string();

                return Err(map_response_err(Some(code)));
            }
        }
    }
}

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
