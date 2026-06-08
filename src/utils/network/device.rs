use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum DeviceState {
    Disconnected,
    DisconnectedWithWifiAvailable,
    ConnectedEthernet,
    ConnectedWifi { level: u8 },
}

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq)]
pub struct NetworkDevice;

impl NetworkDevice {

    pub fn get_state() -> DeviceState {

        if Self::is_ethernet_connected() {
            return DeviceState::ConnectedEthernet;
        }

        if Self::is_wifi_connected() {

            let signal =
                Self::current_signal();

            return DeviceState::ConnectedWifi {
                level: Self::get_level(signal),
            };
        }

        if Self::wifi_available() {
            return DeviceState::DisconnectedWithWifiAvailable;
        }

        DeviceState::Disconnected
    }

    fn is_ethernet_connected() -> bool {

        for iface in Self::interfaces() {

            if iface == "lo" {
                continue;
            }

            let path = format!(
                "/sys/class/net/{}",
                iface
            );

            // ethernet NO tiene wireless/
            if std::path::Path::new(
                &format!("{}/wireless", path)
            )
            .exists()
            {
                continue;
            }

            let operstate =
                std::fs::read_to_string(
                    format!("{}/operstate", path)
                )
                .unwrap_or_default();

            if operstate.trim() == "up" {
                return true;
            }
        }

        false
    }

    fn is_wifi_connected() -> bool {

        for iface in Self::interfaces() {

            let path = format!(
                "/sys/class/net/{}",
                iface
            );

            // wifi tiene wireless/
            if !std::path::Path::new(
                &format!("{}/wireless", path)
            )
            .exists()
            {
                continue;
            }

            let operstate =
                std::fs::read_to_string(
                    format!("{}/operstate", path)
                )
                .unwrap_or_default();

            if operstate.trim() == "up" {
                return true;
            }
        }

        false
    }

    fn wifi_available() -> bool {

        for iface in Self::interfaces() {

            let path = format!(
                "/sys/class/net/{}",
                iface
            );

            if std::path::Path::new(
                &format!("{}/wireless", path)
            )
            .exists()
            {
                return true;
            }
        }

        false
    }

    fn current_signal() -> u8 {

        let content =
            std::fs::read_to_string(
                "/proc/net/wireless"
            )
            .unwrap_or_default();

        for line in content.lines() {

            if line.contains(':') {

                let parts: Vec<&str> =
                    line.split_whitespace()
                        .collect();

                if parts.len() > 2 {

                    let signal = parts[2]
                        .trim_end_matches('.')
                        .parse::<f32>()
                        .unwrap_or(0.0);

                    let percent =
                        ((signal / 70.0) * 100.0)
                            .clamp(0.0, 100.0);

                    return percent as u8;
                }
            }
        }

        0
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

    fn interfaces() -> Vec<String> {

        let mut interfaces = Vec::new();

        let entries =
            match std::fs::read_dir(
                "/sys/class/net"
            ) {
                Ok(entries) => entries,
                Err(_) => return interfaces,
            };

        for entry in entries.flatten() {

            interfaces.push(
                entry.file_name()
                    .to_string_lossy()
                    .to_string()
            );
        }

        interfaces
    }
}