use crate::NetworkManager;
pub struct Device;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum DeviceState {
    Disconnected,
    DisconnectedWithWifiAvailable,
    ConnectedEthernet,
    ConnectedWifi { level: u8 },
}


impl Device{

    pub fn get_state() -> DeviceState {

        if Self::is_ethernet_connected(){
            return DeviceState::ConnectedEthernet;
        }

        if Self::is_wifi_connected(){
            let signal = Self::current_signal();
            if signal < 101{
                return DeviceState::ConnectedWifi{
                    level : Self::get_level(signal)
                }
            }
            return DeviceState::DisconnectedWithWifiAvailable
        }

        if Self::wifi_available(){
            return DeviceState::DisconnectedWithWifiAvailable
        }
        DeviceState::Disconnected
    }

    fn is_ethernet_connected() -> bool {
        let out = match NetworkManager::run(&["-t", "-f", "TYPE,STATE", "device"]) {
            Ok(out) => out,
            Err(_) => return false,
        };

        out.lines().any(|line| {
            let mut parts = line.split(':');
            matches!(
                (parts.next(), parts.next()),
                (Some("ethernet"), Some("connected"))
            )
        })
    }

    fn is_wifi_connected() -> bool {
        let out = match NetworkManager::run(&["-t", "-f", "TYPE,STATE", "device"]) {
            Ok(out) => out,
            Err(_) => return false,
        };

        out.lines().any(|line| {
            let mut parts = line.split(':');
            matches!(
                (parts.next(), parts.next()),
                (Some("wifi"), Some("connected"))
            )
        })
    }

    fn wifi_available() -> bool {
        let out = match NetworkManager::run(&["-t", "-f", "TYPE,STATE", "device"]) {
            Ok(out) => out,
            Err(_) => return false,
        };

        out.lines().any(|line| {
            let mut parts = line.split(':');
            matches!(
                (parts.next(), parts.next()),
                (Some("wifi"), Some("disconnected"))
            )
        })
    }
 
    fn current_signal() -> u8 {
        let out = match NetworkManager::run(&["-t", "-f", "IN-USE,SIGNAL", "dev", "wifi", "list"]) {
            Ok(out) => out,
            Err(_) => return 255,
        };

        for line in out.lines() {
            let mut parts = line.split(':');
            if let (Some("*"), Some(signal)) = (parts.next(), parts.next()) {
                if let Ok(value) = signal.parse::<u8>() {
                    return value;
                }
            }
        }

        255
    }

     pub fn get_level(signal: u8) -> u8 {
        match signal {
            0..=9 => 0,
            10..=29 => 1,
            30..=49 => 2,
            50..=74 => 3,
            _ => 4,
        }
    }

    
    
}