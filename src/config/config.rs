
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy; // Añade once_cell = "1.18" a tu Cargo.toml
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

    pub fn google(&self)->&ConfigGoogle{
        &self.google
    }

    pub fn google_mut(&mut self)->&mut ConfigGoogle{
        &mut self.google
    }

    pub fn styles(&self)->&ConfigStyles{
        &self.styles
    }

    pub fn styles_mut(&mut self)->&mut ConfigStyles{
        &mut self.styles
    }

    pub fn keybindings_mut(&mut self)->&mut ConfigKeybindings{
        &mut self.keybindings
    }
    pub fn keybindings(&self)->&ConfigKeybindings{
        &self.keybindings
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



// Definimos la global como pub para que otros archivos la vean
pub static GLOBAL_CFG: Lazy<Arc<RwLock<Config>>> = Lazy::new(|| {
    // Esto se ejecutará la primera vez que alguien acceda a GLOBAL_CFG
    let cfg_folder = String::from("~/.config/PunkWM");
    Arc::new(RwLock::new(Config::load(cfg_folder)))
});