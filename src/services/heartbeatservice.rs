use std::time::Duration;
use std::thread;

use crate::libs::settings::Settings;
use crate::libs::system::System;

pub struct HeartbeatService;

impl HeartbeatService {
    
    pub fn new() -> HeartbeatService {
        HeartbeatService {}
    }
    
    pub fn send(&self, settings: &Settings) -> Option<String> {
        
        let full_url = vec![settings.url.as_ref(), "api", "hosts", "heartbeat"].join("/");
        let auth_value = vec!["Bearer", settings.token.as_ref()].join(" ");

        let mut request = minreq::post(&full_url)
            .with_header("Authorization", auth_value);

        if settings.monitoring {
            let system = System::get_system_data();

            request = request.with_header("Content-Type", "application/json")
                .with_body(system.to_json(false));
        }

        let response = request.send();
            
        match response {
            Ok(r) => {
                if r.status_code != 204 && r.status_code != 200 {
                    return Some(format!("> Error sending heartbeat: the status code recevied is the next one {}", r.status_code))
                }
                
                return None;
            },
            Err(e) => return Some(format!("> Error sending heartbeat: see error here -> {}", e))
        };
        
    }
    
    pub fn send_as_daemon(&self, settings: &Settings) {
        
        let interval_in_seconds: f32 = settings.interval * 60.0;
        let duration: Duration = Duration::from_secs_f32(interval_in_seconds);
        
        loop {
            
            match self.send(&settings) {
                Some(error) => println!("{}", error),
                None => ()
            }
            
            thread::sleep(duration);
        }
        
    } 
}