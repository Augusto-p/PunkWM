
use serde::{Deserialize, Serialize};

use crate::config::{apps::ConfigApps, google::ConfigGoogle, keybindings::ConfigKeybindings, styles::ConfigStyles, weather::ConfigWeather};

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct Config{
    folder: String,
    sudo: bool,
    weather: ConfigWeather,
    google: ConfigGoogle,
    styles: ConfigStyles,
    keybindings: ConfigKeybindings,
    apps: ConfigApps,

}
impl Config {
    
    pub fn load(mut folder:String)->Self{
        folder = Config::expand_home(&folder);
        let weather = ConfigWeather::load(folder.clone());
        let google = ConfigGoogle::load(folder.clone());
        let styles = ConfigStyles::load(folder.clone());
        let keybindings = ConfigKeybindings::load(folder.clone());
        let apps = ConfigApps::load(folder.clone());
        Self{
           folder: folder.clone(),
           weather: weather,
           google: google,
           styles: styles,
           keybindings: keybindings,
           apps: apps,
           sudo: false
        }   
    }

    pub fn weather(&mut self)->&mut ConfigWeather{
        &mut self.weather
    }

    pub fn google(&mut self)->&mut ConfigGoogle{
        &mut self.google
    }

    pub fn styles(&mut self)->&mut ConfigStyles{
        &mut self.styles
    }

    pub fn keybindings(&mut self)->&mut ConfigKeybindings{
        &mut self.keybindings
    }

    pub fn folder(&self)->&String{
        &self.folder
    }

    pub fn apps(&mut self)->&mut ConfigApps{
        &mut self.apps
    }

   pub fn expand_home(path: &str) -> String {
        if path.starts_with('~') {
            if let Some(home) = std::env::var_os("HOME") {
                return path.replacen('~', &home.to_string_lossy(), 1);
            }
        }
        path.to_string()
    }
}

// #[derive(Deserialize)]
// pub struct Config {
//     pub apps: HashMap<String, String>,
//     pub keybindings: HashMap<String, String>,
//     pub styles: Styles,
//     pub weather: WeatherConfig,
//     pub google: Google,
// }