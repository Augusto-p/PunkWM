use crate::NetworkManager;
use crate::Device;

use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WiFiNetwork{
    pub ssid: String,
    pub security: bool,
    pub level: u8,
    pub connected: bool
}


pub fn get_wifi_networks() -> Vec<WiFiNetwork> {
    let out = match NetworkManager::run(&[
        "-t",
        "-f",
        "IN-USE,SSID,SECURITY,SIGNAL",
        "device",
        "wifi",
        "list",
    ]) {
        Ok(out) => out,
        Err(_) => return Vec::new(),
    };

    let mut nets = Vec::new();

    for line in out.lines() {
        // formato: *:MiWifi:WPA2:78
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            continue;
        }

        let connected = parts[0] == "*";
        let ssid = parts[1].to_string();
        let security = !parts[2].trim().is_empty();
        let level = Device::get_level(parts[3].parse::<u8>().unwrap_or(0));
        
        if ssid.is_empty() {
            continue;
        }

        nets.push(WiFiNetwork {
            ssid,
            security,
            level,
            connected,
        });
    }

    nets
}
