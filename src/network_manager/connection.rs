// use crate::utils::config::print_in_tty;
use crate::NetworkManager;
use crate::utils::qrcode::get_qrcode_in_base64;

pub struct NetworkConnection;

impl NetworkConnection {

    pub fn connect(ssid: &str, password: &str) -> bool {
        NetworkManager::run(&[
            "device", "wifi", "connect", ssid, "password", password
        ]).is_ok()
    }

    pub fn connect_public(ssid: &str) -> bool {
        NetworkManager::run(&[
            "device", "wifi", "connect", ssid
        ]).is_ok()
    }

   pub fn disconnect() -> bool {
        let ssid_out = match NetworkManager::run(&[
            "-t","-f","ACTIVE,SSID","device","wifi"
        ]) {
            Ok(o) => o,
            Err(_) => return false,
        };

        let ssid = match ssid_out
            .lines()
            .find(|l| l.starts_with("yes:"))
            .and_then(|l| l.split(':').nth(1))
        {
            Some(s) if !s.is_empty() => s,
            _ => return false,
        };

        NetworkManager::run(&[
            "connection",
            "delete",
            ssid,
        ]).is_ok()
    }



    fn escape(s: &str) -> String {
        s.replace("\\", "\\\\")
            .replace(";", "\\;")
            .replace(",", "\\,")
            .replace(":", "\\:")
    }

    fn wifi_qr_string(ssid: &str, password: &str, secured: bool) -> String {
        let t = if secured { "WPA" } else { "nopass" };

        format!(
            "WIFI:T:{};S:{};P:{};;",
            t,
            Self::escape(ssid),
            Self::escape(password)
        )
    }

    

    pub fn share_wifi() -> Option<String> {

        let ssid_out = NetworkManager::run(&[
            "-t","-f","ACTIVE,SSID","device","wifi"
        ]).ok()?;

        let ssid = ssid_out
            .lines()
            .find(|l| l.starts_with("yes:"))?
            .split(':')
            .nth(1)?
            .to_string();

        if ssid.is_empty() {
            return None;
        }
        
        let pass_out = NetworkManager::run_sudo(&[
            "-s","-g","802-11-wireless-security.psk",
            "connection","show",&ssid
        ]).ok()?;
        let password = pass_out.trim().to_string();
        let secured = !password.is_empty();
        let data = Self::wifi_qr_string(&ssid, &password, secured);
        get_qrcode_in_base64(&data)
    }

}
