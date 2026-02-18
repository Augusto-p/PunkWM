// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, fs::{self, OpenOptions}, io::{self, Write},  sync::OnceLock};
// use std::env;

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Config {
//     pub lock_screen: Lock_Screen,
// }
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Lock_Screen{
//     pub bg: String,
// }

// pub(crate) fn load_config() -> Config {
//     let path = format!(
//         "config.toml{}",
//         std::env::var("HOME").unwrap()
//     );
//     let data = fs::read_to_string(&path).expect("No se pudo leer config.toml");
//     let cfg: Config = toml::from_str(&data).expect("Config TOML inválido");

//     cfg
// }


