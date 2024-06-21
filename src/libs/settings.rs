use std::fmt;
use json::object;

pub struct Settings {
    pub url: String,
    pub token: String,
    pub interval: f32,
    pub monitoring: bool,
}

impl Settings {
    
    pub fn default() -> Settings {
        Settings { 
            url: String::from("https://hostbeatapi.rubenarrebola.pro"), 
            token: "".to_string(), 
            interval: 1.0,
            monitoring: false,
        }
    }

    pub fn from_json(json_data: &str) -> Settings {
        let mut conf = Settings::default();

        let operation = json::parse(&json_data);
        
        if operation.is_err() {
            panic!("Error loading data from json string");
        }

        let parsed = operation.unwrap();
        
        if parsed.has_key("url") {
            conf.url = parsed["url"].to_string();
        }

        if parsed.has_key("monitoring") {
            conf.monitoring = parsed["monitoring"].as_bool().unwrap();
        }

        if !parsed.has_key("token") {
            panic!("Error, json data does not contains key '{}'", "token");
        }

        if !parsed.has_key("interval") {
            panic!("Error, json data does not contains key '{}'", "interval");
        }

        conf.token = parsed["token"].to_string();
        conf.interval = parsed["interval"].as_f32().unwrap();

        return conf;
    }

    pub fn set_url(&mut self, url: String) -> &Self {
        self.url = url;
        self
    }

    pub fn set_token(&mut self, token: String) -> &Self {
        self.token = token;
        self
    }

    pub fn set_interval(&mut self, interval: f32) -> &Self {
        self.interval = interval;
        self
    }

    pub fn set_monitoring(&mut self, monitoring: bool) -> &Self {
        self.monitoring = monitoring;
        self
    }

    pub fn to_json(&self) -> String {
        let data = object!{
            url: self.url.to_string(),
            token: self.token.to_string(),
            interval: self.interval.clone(),
            monitoring: self.monitoring.clone(),
        };

        return json::stringify_pretty(data, 4);
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Settings: url -> '{}' , interval -> '{}' , token -> '{}', monitoring -> '{}'", self.url, self.interval, self.token, self.monitoring)
    }
}

impl Clone for Settings {
    fn clone(&self) -> Self {
        Self { url: self.url.clone(), token: self.token.clone(), interval: self.interval.clone(), monitoring: self.monitoring.clone() }
    }
}

impl PartialEq for Settings {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.token == other.token && self.interval == other.interval && self.monitoring == other.monitoring
    }
}