use serde::{Deserialize, Serialize};
use x11rb::protocol::xproto::ModMask;
use std::{collections::HashMap, fs::{self, OpenOptions}, io::{self, Write},  sync::OnceLock};

use crate::utils::keys::{keycode_from_name, parse_mods};
pub const TTY: &str = "/dev/pts/0";
#[derive(Deserialize, Clone)]
pub struct Apps {
    pub terminal: String,
    pub browser: String,
    pub filemanager: String,
    pub editor: String,
    pub dock: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub apps: Apps,
    pub keybindings: HashMap<String, String>,
    pub styles: Styles,
    pub weather: WeatherConfig,
    pub google: Google,
}
#[derive(Deserialize)]
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

#[derive(Clone)]
pub struct Binding {
    pub modifier: ModMask,
    pub keycode: u8,
    pub action: String,
}

use std::env;

fn expand_home(s: &mut String, home: &str) {
    if s.contains('~') {
        *s = s.replace("~", home);
    }
}

impl Config {
    pub fn expand_tilde(mut self) -> Self {
        let home = env::var("HOME").unwrap();

        // Apps
        expand_home(&mut self.apps.terminal, &home);
        expand_home(&mut self.apps.browser, &home);
        expand_home(&mut self.apps.filemanager, &home);
        expand_home(&mut self.apps.editor, &home);
        expand_home(&mut self.apps.dock, &home);

        // Styles
        expand_home(&mut self.styles.bg, &home);

        // Google
        expand_home(&mut self.google.credentials_file, &home);

        self
    }
}


pub fn path_persistence() -> &'static str {
    static PATH: OnceLock<String> = OnceLock::new();
    PATH.get_or_init(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        format!("{}/.config/punkwm/persistence.bin", home)
    })
}
pub(crate) fn load_config() -> Config {
    let path = format!(
        "{}/.config/punkwm/config.toml",
        std::env::var("HOME").unwrap()
    );

    let data = fs::read_to_string(&path).expect("No se pudo leer config.toml");
    let cfg: Config = toml::from_str(&data).expect("Config TOML inválido");
    cfg.expand_tilde()
}

pub(crate) fn parse_bindings(cfg: &Config) -> Vec<Binding> {
    let mut bindings = Vec::new();

    for (k, action) in &cfg.keybindings {
        let parts: Vec<&str> = k.split('_').collect();
        if parts.len() != 2 {
            continue;
        }

        let modifier = parse_mods(parts[0]);
        if modifier == ModMask::from(0u16) {
            continue;
        }

        if let Some(code) = keycode_from_name(parts[1]) {
            bindings.push(Binding {
                modifier,
                keycode: code,
                action: action.clone(),
            });
        }
    }

    bindings
}


pub fn print_in_tty(texto: &str) -> io::Result<()> {
    let mut tty = OpenOptions::new()
        .write(true)
        .open(TTY)?;

    writeln!(tty, "=> {}\n", texto)?;
    Ok(())
}
