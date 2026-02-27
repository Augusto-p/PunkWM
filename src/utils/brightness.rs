use std::fs;
use std::path::Path;
pub struct Brightness;

impl Brightness {
    pub fn get() -> u8 {
        let backlight_path = Path::new("/sys/class/backlight");

        let device = match fs::read_dir(backlight_path)
            .ok()
            .and_then(|mut entries| entries.next())
        {
            Some(Ok(entry)) => entry.path(),
            _ => return 0,
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

    pub fn set(present: u8){
        println!("hello {}", present)
    }
}
