use std::{path::PathBuf, fs, collections::HashMap};

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
            settings_dir: dirs::config_dir().unwrap().join("dontdie"), 
            settings_file: dirs::config_dir().unwrap().join("dontdie").join("settings.json"),
        }
    }

    pub fn load_settings(mut self) -> SettingsService {
    
        // Create config
        if !self.settings_dir.exists() {
            
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

    pub fn set_from(mut self, map: HashMap<String, String>) {

        self.settings = match map.get("--set-url") {
            Some(value) => self.settings.set_url(value.clone()),
            None => self.settings
        };

        self.settings = match map.get("--set-token") {
            Some(value) => self.settings.set_token(value.clone()),
            None => self.settings
        };

        self.settings = match map.get("--set-interval") {
            Some(value) => self.settings.set_interval(value.clone().parse().unwrap()),
            None => self.settings
        };

        if !self.settings_dir.exists() {
            
            match fs::create_dir_all(self.settings_dir) {
                Ok(_) => (),
                Err(e) => panic!("> Error creating settings directory, full error: {}", e)
            };

        }

        match fs::write(self.settings_file.clone(), self.settings.to_json()) {
            Ok(_) => (),
            Err(e) => panic!("> Error writing on settings file, full error: {}", e)
        }
        
    }

}

impl AsRef<SettingsService> for SettingsService {
    fn as_ref(&self) -> &SettingsService {
        &self
    }
}