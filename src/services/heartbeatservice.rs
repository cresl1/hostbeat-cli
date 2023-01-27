use crate::libs::settings::Settings;

pub struct HeartbeatService;

impl HeartbeatService {
    
    pub fn new() -> HeartbeatService {
        HeartbeatService {}
    }
    
    pub fn send(&self, settings: &Settings) -> Option<String> {
        let full_url = vec![settings.url.as_ref(), "api", "hosts", "heartbeat"].join("/");
        let auth_value = vec!["Bearer", settings.token.as_ref()].join(" ");
        let response = minreq::post(&full_url)
            .with_header("Authorization", auth_value)
            .send();
            
        match response {
            Ok(r) => {
                if r.status_code != 204 && r.status_code != 200 {
                    return Some(format!("Error sending heartbeat: the status code recevied is the next one {}", r.status_code))
                }
                
                return None;
            },
            Err(e) => return Some(format!("Error sending heartbeat: see error here -> {}", e))
        };
        
    }
}