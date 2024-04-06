use std::{path::PathBuf, fs, collections::HashMap};
use regex::Regex;

use crate::libs::settings::Settings;

pub struct SettingsService {
    pub settings: Settings,
    settings_dir: PathBuf,
    settings_file: PathBuf,
}

impl SettingsService {

    pub fn new() -> SettingsService {
        SettingsService {
            settings: Settings::default(),
            settings_dir: dirs::config_dir().unwrap().join("hostbeat"), 
            settings_file: dirs::config_dir().unwrap().join("hostbeat").join("settings.json"),
        }
    }

    pub fn load_settings(mut self, create_when_not_found: bool) -> SettingsService {
    
        // Create config
        if !self.settings_dir.exists() && create_when_not_found {
            
            match fs::create_dir_all(self.settings_dir.clone()) {
                Ok(_) => (),
                Err(e) => panic!("> Error creating settings directory, full error: {}", e)
            };
            match fs::write(self.settings_file.clone(), self.settings.to_json()) {
                Ok(_) => (),
                Err(e) => panic!("> Error creating settings file, full error: {}", e)
            }
            
            return self;
        }
    
        // Load current config
        match fs::read_to_string(self.settings_file.clone()) {
            Ok(content) => self.settings = Settings::from_json(&content),
            Err(e) => panic!("> Error reading settings file, full error: {}", e)
        };

        return self;
    }
    
    pub fn set_to_file_from(&mut self, map: HashMap<String, String>) -> Option<String> {
        
        let url_value: Option<&String> = map.get("--set-url");
        
        if url_value.is_some() {
            
            if !self.url_is_valid(&url_value.unwrap()) {
                return Some(format!("> Invalid URL error: the url '{}' is not in a correct format", &url_value.unwrap())); 
            }
            
            self.settings.set_url(url_value.unwrap().clone());
        }
        
        let token_value:Option<&String> = map.get("--set-token");
        
        if token_value.is_some() {
            
            if token_value.unwrap().is_empty() || token_value.unwrap().len() < 15 {
                return Some(format!("> Invalid token error: the token can't be empty or too much little"));
            }
            
            self.settings.set_token(token_value.unwrap().clone());
        }
        
        let interval_value = map.get("--set-interval");
        
        if interval_value.is_some() {
            
            let parsed: f32 = match interval_value.unwrap().parse() {
                Ok(value) => value,
                Err(_) => return Some(format!("> Interval conversion error: interval should be in decimal format like -> 1.5"))
            };
            
            if parsed < 0.5 || parsed > 3.0 {
                return Some("> Invalid interval error: interval should be bettween 0.5 and 3 minutes".to_string());
            }
            
            self.settings.set_interval(parsed);
        }
        
        match fs::write(self.settings_file.clone(), self.settings.to_json()) {
            Ok(_) => return None,
            Err(e) => return Some(format!("> Writing settings error: can't write settings into the file, for more detail: {}", e)),
        }
    }
    
    pub fn set_to_memory_from(&mut self, map: HashMap<String, String>) -> Option<String> {
        
        let url_value: Option<&String> = map.get("--use-url");
        
        if url_value.is_some() {
            
            if !self.url_is_valid(&url_value.unwrap()) {
                return Some(format!("> Invalid URL error: the url '{}' is not in a correct format", &url_value.unwrap())); 
            }
            
            self.settings.set_url(url_value.unwrap().clone());
        }
        
        let token_value:Option<&String> = map.get("--use-token");
        
        if token_value.is_some() {
            
            if token_value.unwrap().is_empty() || token_value.unwrap().len() < 15 {
                return Some(format!("> Invalid token error: the token can't be empty or too much little"));
            }
            
            self.settings.set_token(token_value.unwrap().clone());
        }
        
        let interval_value = map.get("--use-interval");
        
        if interval_value.is_some() {
            
            let parsed: f32 = match interval_value.unwrap().parse() {
                Ok(value) => value,
                Err(_) => return Some(format!("> Interval conversion error: interval should be in decimal format like -> 1.5"))
            };
            
            if parsed < 0.5 || parsed > 3.0 {
                return Some("> Invalid interval error: interval should be bettween 0.5 and 3 minutes".to_string());
            }
            
            self.settings.set_interval(parsed);
        }
        
        return None;
    }
    
    fn url_is_valid(&self, url: &String) -> bool {
        let regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap();
        return  regex.is_match(url);
    }

}

impl AsRef<SettingsService> for SettingsService {
    fn as_ref(&self) -> &SettingsService {
        &self
    }
}

impl Clone for SettingsService {
    fn clone(&self) -> Self {
        Self { 
            settings: self.settings.clone(), 
            settings_dir: self.settings_dir.clone(), 
            settings_file: self.settings_file.clone() 
        }
    }
}