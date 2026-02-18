use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub lock_screen: LockScreen,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LockScreen{
    pub bg: String,
}

pub(crate) fn load_config() -> Config {
    // let path = format!(
    //     "config.toml{}",
    //     std::env::var("HOME").unwrap()
    // );
    let path = "/home/augus/PunkWM/tols/config.toml";
    let data = fs::read_to_string(&path).expect("No se pudo leer config.toml");
    let cfg: Config = toml::from_str(&data).expect("Config TOML inválido");

    cfg
}


