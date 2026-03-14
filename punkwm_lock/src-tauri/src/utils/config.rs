use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    lock_screen: LockScreen,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LockScreen{
    bg: String,
}

impl LockScreen{
    fn get_bg(self)->String{
        self.bg.clone()
    }
}

impl Config{
    pub fn get_bg(self)->String{
        self.lock_screen.get_bg()
    }

}


pub(crate) fn load_config() -> Config {
    let path = "/home/augus/PunkWM/tols/config.toml";
    let data = fs::read_to_string(&path).expect("No se pudo leer config.toml");
    let cfg: Config = toml::from_str(&data).expect("Config TOML inválido");
    cfg
}


