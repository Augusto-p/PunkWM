pub struct NetworkConnection;
use crate::NetworkManager;

impl NetworkConnection {
    
    pub fn connect(ssid: &str, password: &str) -> bool {
        match NetworkManager::run(&[
            "device", "wifi", "connect", ssid, "password", password
        ]) {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }


    pub fn connect_public(ssid: &str) -> bool {
        match NetworkManager::run(&[
            "device", "wifi", "connect", ssid
        ]) {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    pub fn disconnect() -> bool {
        // obtener interfaz wifi activa
        let iface_out = match NetworkManager::run(&[
            "-t",
            "-f",
            "DEVICE,TYPE,STATE",
            "device",
        ]) {
            Ok(o) => o,
            Err(_) => return false,
        };

        let iface = iface_out
            .lines()
            .find(|l| l.contains(":wifi:connected"))
            .and_then(|l| l.split(':').next());

        let iface = match iface {
            Some(i) => i,
            None => return false,
        };

        match NetworkManager::run(&[
            "device",
            "disconnect",
            iface,
        ]) {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    
    use qrcode::QrCode;
use image::{Luma, ImageOutputFormat};
use base64::{engine::general_purpose, Engine as _};
use std::io::Cursor;

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
        escape(ssid),
        escape(password)
    )
}

pub fn share_wifi() -> Option<String> {
    // 1️⃣ obtener SSID activo
    let ssid_out = NetworkManager::run(&[
        "-t",
        "-f",
        "ACTIVE,SSID",
        "device",
        "wifi",
    ]).ok()?;

    let ssid = ssid_out
        .lines()
        .find(|l| l.starts_with("yes:"))
        .and_then(|l| l.split(':').nth(1))?
        .to_string();

    if ssid.is_empty() {
        return None;
    }

    // 2️⃣ obtener password guardada
    let pass_out = NetworkManager::run(&[
        "-s",
        "-g",
        "802-11-wireless-security.psk",
        "connection",
        "show",
        &ssid,
    ]).ok()?;

    let password = pass_out.trim().to_string();
    let secured = !password.is_empty();

    // 3️⃣ construir QR data
    let data = wifi_qr_string(&ssid, &password, secured);

    // 4️⃣ generar QR
    let code = QrCode::new(data.as_bytes()).ok()?;

    let image = code.render::<Luma<u8>>()
        .min_dimensions(300, 300)
        .build();

    // 5️⃣ PNG → base64
    let mut buf = Cursor::new(Vec::new());
    image.write_to(&mut buf, ImageOutputFormat::Png).ok()?;

    Some(general_purpose::STANDARD.encode(buf.into_inner()))
}

}