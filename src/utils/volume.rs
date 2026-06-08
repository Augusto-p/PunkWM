use std::process::Command;

pub struct Volume;

impl Volume {
    pub fn set(percent: u8) {
        let percent = percent.min(100);
        let volume = format!("{percent}%");

        if Self::run(
            "pactl",
            &["set-sink-volume", "@DEFAULT_SINK@", &volume],
        ) {
            return;
        }

        let _ = Self::run("amixer", &["sset", "Master", &volume]);
    }

    pub fn get() -> u8 {
        if let Some(v) = Self::get_from_pactl() {
            return v;
        }

        if let Some(v) = Self::get_from_amixer() {
            return v;
        }

        0
    }

    pub fn mute() {
        if Self::run(
            "pactl",
            &["set-sink-mute", "@DEFAULT_SINK@", "1"],
        ) {
            return;
        }

        let _ = Self::run("amixer", &["sset", "Master", "mute"]);
    }

    pub fn unmute() {
        if Self::run(
            "pactl",
            &["set-sink-mute", "@DEFAULT_SINK@", "0"],
        ) {
            return;
        }

        let _ = Self::run("amixer", &["sset", "Master", "unmute"]);
    }

    pub fn is_muted() -> bool {
        if let Ok(output) = Command::new("pactl")
            .args(["get-sink-mute", "@DEFAULT_SINK@"])
            .output()
        {
            if let Ok(text) = String::from_utf8(output.stdout) {
                return text.contains("yes");
            }
        }

        false
    }

    fn get_from_pactl() -> Option<u8> {
        let output = Command::new("pactl")
            .args(["get-sink-volume", "@DEFAULT_SINK@"])
            .output()
            .ok()?;

        let text = String::from_utf8(output.stdout).ok()?;

        Self::extract_percentage(&text)
    }

    fn get_from_amixer() -> Option<u8> {
        let output = Command::new("amixer")
            .args(["sget", "Master"])
            .output()
            .ok()?;

        let text = String::from_utf8(output.stdout).ok()?;

        Self::extract_percentage(&text)
    }

    fn extract_percentage(text: &str) -> Option<u8> {
        for token in text.split_whitespace() {
            let token = token.trim_matches(|c| {
                c == '[' || c == ']' || c == '%'
            });

            if let Ok(v) = token.parse::<u8>() {
                return Some(v.min(100));
            }
        }

        None
    }

    fn run(cmd: &str, args: &[&str]) -> bool {
        Command::new(cmd)
            .args(args)
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}