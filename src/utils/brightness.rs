use std::fs;
use std::path::Path;

pub struct Brightness;

impl Brightness {
    fn get_device() -> Option<std::path::PathBuf> {
        let backlight_path = Path::new("/sys/class/backlight");

        fs::read_dir(backlight_path)
            .ok()?
            .next()?
            .ok()
            .map(|e| e.path())
    }

    pub fn get() -> u8 {
        let device = match Self::get_device() {
            Some(v) => v,
            None => return 0,
        };

        let brightness: u32 = match fs::read_to_string(device.join("brightness"))
            .ok()
            .and_then(|s| s.trim().parse().ok())
        {
            Some(v) => v,
            None => return 0,
        };

        let max_brightness: u32 = match fs::read_to_string(device.join("max_brightness"))
            .ok()
            .and_then(|s| s.trim().parse().ok())
        {
            Some(v) => v,
            None => return 0,
        };

        if max_brightness == 0 {
            return 0;
        }

        ((brightness * 100) / max_brightness) as u8
    }

    pub fn set(percent: u8) -> bool {
        let device = match Self::get_device() {
            Some(v) => v,
            None => return false,
        };

        let max_brightness: u32 = match fs::read_to_string(device.join("max_brightness"))
            .ok()
            .and_then(|s| s.trim().parse().ok())
        {
            Some(v) => v,
            None => return false,
        };

        let percent = percent.min(100) as u32;

        let value = (percent * max_brightness) / 100;

        fs::write(
            device.join("brightness"),
            value.to_string(),
        )
        .is_ok()
    }
}