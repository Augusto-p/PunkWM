use std::process::Command;
pub struct Volume;

impl Volume {
    pub fn set(percent:u8){
        let percent = percent.min(100);
        let volume = format!("{}%", percent);
        if let Ok(status) = Command::new("pactl").args(["set-sink-volume", "@DEFAULT_SINK@", &volume]).status(){
            if status.success() {return;}
        }
        let _ = Command::new("amixer").args(["sset", "Master", &volume]).status();
    }

    pub fn get()->u8{
        if let Ok(output) = Command::new("pactl").args(["get-sink-volume", "@DEFAULT_SINK@"]).output(){
            if let Ok(text) = String::from_utf8(output.stdout) {
                if let Some(percent) = Volume::extract_percentage(&text) {
                    return percent;
                }
            }
        }

        if let Ok(output) = Command::new("amixer").args(["sget", "Master"]).output(){
            if let Ok(text) = String::from_utf8(output.stdout) {
                if let Some(percent) = Volume::extract_percentage(&text) {
                    return percent;
                }
            }
        }

        0
    }

    pub fn unmute(){
        if let Ok(status) = Command::new("pactl").args(["set-sink-mute", "@DEFAULT_SINK@", "0"]).status(){
            if status.success() {
                return;
            }
        }
        let _ = Command::new("amixer").args(["sset", "Master", "unmute"]).status();
    }

    fn extract_percentage(text: &str) -> Option<u8> {
    for word in text.split_whitespace() {
        if word.ends_with('%') {
            let clean = word.trim_matches(|c| c == '%' || c == '[' || c == ']');
            if let Ok(value) = clean.parse::<u8>() {
                return Some(value.min(100));
            }
        }
    }
    None
}
}   
