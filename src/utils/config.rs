use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::{self, OpenOptions}, io::{Write},  sync::OnceLock};
use std::path::Path;
use std::env;
pub const TTY: &str = "/dev/pts/0";

#[derive(Deserialize)]
pub struct Config {
    pub apps: HashMap<String, String>,
    pub keybindings: HashMap<String, String>,
    pub styles: Styles,
    pub weather: WeatherConfig,
    pub google: Google,
}
#[derive(Deserialize, Clone)]
pub struct Styles{
    pub bg: String,
    pub dock_width: u16,
    pub panel_width: u16,
    pub lang: String,
}

#[derive(Deserialize, Clone)]
pub struct Google{
    pub credentials_file: String,
    pub max_events_view: u16,
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct WeatherConfig{
    pub city: String,
    pub state: String,
    pub country: String,
    pub units: String,
}
pub(crate) fn load_config() -> Config {
    let mut path = format!(
        "{}/.config/punkwm/config.toml",
        std::env::var("HOME").unwrap()
    );
    if !Path::new(&path).exists(){
        path = "/etc/PunkWM/config.toml".to_string();
    }

    let data = fs::read_to_string(&path).expect("No se pudo leer config.toml");
    let mut cfg: Config = toml::from_str(&data).expect("Config TOML inválido");
    cfg.apps = cfg.apps.into_iter().map(|(k, v)| (k.to_uppercase(), v)).collect();
    cfg.expand_tilde()
}
fn expand_home(s: &mut String, home: &str) {
    if s.contains('~') {
        *s = s.replace("~", home);
    }
}

impl Config {
    pub fn expand_tilde(mut self) -> Self {
        let home = env::var("HOME").unwrap();

        // Apps
        for command in self.apps.values_mut() {
            expand_home(command, &home);
        }

        // Styles
        expand_home(&mut self.styles.bg, &home);

        // Google
        expand_home(&mut self.google.credentials_file, &home);

        self
    }

    pub fn get_app_command(&self, name: &str) -> String {
        self.apps.get(&name.to_uppercase()).cloned().unwrap_or_else(|| "".to_string()) // Maneja el caso de que no exista
    }
}


pub fn path_persistence() -> &'static str {
    static PATH: OnceLock<String> = OnceLock::new();
    PATH.get_or_init(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        format!("{}/.config/punkwm/persistence.bin", home)
    })
}


pub fn print_in_tty(texto: &str) {
    match OpenOptions::new().write(true).open(TTY) {
        Ok(mut tty) => {
            if let Err(e) = writeln!(tty, "=> {}\n", texto) {
                eprintln!("Error escribiendo en TTY: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error abriendo TTY: {}", e);
        }
    }
}
