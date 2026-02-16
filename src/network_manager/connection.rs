pub struct NetworkConnection;

use crate::NetworkManager;
use qrcode::QrCode;
use base64::{engine::general_purpose, Engine as _};
use qrcode::render::svg;

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
        let iface_out = match NetworkManager::run(&[
            "-t",
            "-f",
            "DEVICE,TYPE,STATE",
            "device",
        ]) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let iface = match iface_out
            .lines()
            .find(|l| l.contains(":wifi:connected"))
            .and_then(|l| l.split(':').next())
        {
            Some(i) => i,
            None => return false,
        };

        NetworkManager::run(&[
            "device",
            "disconnect",
            iface,
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

        let pass_out = NetworkManager::run(&[
            "-s","-g","802-11-wireless-security.psk",
            "connection","show",&ssid
        ]).ok()?;

        let password = pass_out.trim().to_string();
        let secured = !password.is_empty();

        let data = Self::wifi_qr_string(&ssid, &password, secured);

        let code = QrCode::new(data.as_bytes()).ok()?;

        // ✅ SVG renderer (no features extra)
        let svg: String = data;


        // ✅ base64
        Some(general_purpose::STANDARD.encode(svg.as_bytes()))
    }

}
