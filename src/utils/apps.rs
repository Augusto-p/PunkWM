use freedesktop_desktop_entry::DesktopEntry;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize)]
pub struct Application {
    /// ID usado por gtk-launch
    /// ejemplo:
    /// firefox.desktop -> firefox
    pub id: String,

    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub terminal: bool,
    pub desktop_file: PathBuf,
}

#[derive(Debug)]
pub struct AppManager {
    apps: Vec<Application>,
    index: HashMap<String, usize>,
}

impl AppManager {
    pub fn new() -> Self {
        Self {
            apps: Vec::new(),
            index: HashMap::new(),
        }
    }

    pub fn load(&mut self) {
        self.apps.clear();
        self.index.clear();

        for dir in Self::application_dirs() {
            self.load_directory(&dir);
        }

        self.apps.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        });

        // reconstruir indice
        for (idx, app) in self.apps.iter().enumerate() {
            self.index.insert(app.id.clone(), idx);
        }
    }

    pub fn apps(&self) -> &[Application] {
        &self.apps
    }

    pub fn get(&self, id: &str) -> Option<&Application> {
        self.index
            .get(id)
            .and_then(|idx| self.apps.get(*idx))
    }

    pub fn search(&self, query: &str) -> Vec<&Application> {
        let query = query.to_lowercase();

        self.apps
            .iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&query)
                    || app.id.to_lowercase().contains(&query)
            })
            .collect()
    }

    /// Ejecuta usando gtk-launch y el ID
    pub fn launch(&self, id: &str) -> std::io::Result<()> {
        let app = self.get(id);
        if app.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "application not found",
            ));
        }

        // Command::new("gtk-launch")
        //     .arg(id)
        //     .spawn()?;

        Command::new(app.unwrap().exec.clone()).spawn()?;
        Ok(())
    }

    fn load_directory(&mut self, dir: &str) {
        if !Path::new(dir).exists() {
            return;
        }

        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) != Some("desktop") {
                continue;
            }

            let Ok(content) = fs::read_to_string(path) else {
                continue;
            };

            let Ok(desktop) = DesktopEntry::from_str(
                path,
                &content,
                None::<&[&str]>,
            ) else {
                continue;
            };

            // ignorar ocultas
            if desktop.hidden() || desktop.no_display() {
                continue;
            }
            let locales: [&str; 0] = [];
            let Some(name) = desktop.name(&locales) else {
                continue;
            };

            // ID real usado por gtk-launch
            let Some(stem) = path.file_stem() else {
                continue;
            };

            let id = stem.to_string_lossy().to_string();

            // evitar duplicados
            if self.index.contains_key(&id) {
                continue;
            }

            let exec = desktop.exec().unwrap_or("").to_string();

            let app = Application {
                id: id.clone(),
                name: name.to_string(),
                exec: Self::clean_exec(&exec),
                icon: desktop.icon().map(|v| v.to_string()),
                terminal: desktop.terminal(),
                desktop_file: path.to_path_buf(),
            };

            self.index.insert(id, self.apps.len());
            self.apps.push(app);
        }
    }

    fn application_dirs() -> Vec<String> {
        let mut dirs = vec![
            "/usr/share/applications".to_string(),
            "/usr/local/share/applications".to_string(),
        ];

        if let Ok(home) = std::env::var("HOME") {
            dirs.push(format!("{home}/.local/share/applications"));
        }

        dirs
    }

    fn clean_exec(exec: &str) -> String {
        exec
            .replace("%U", "")
            .replace("%u", "")
            .replace("%F", "")
            .replace("%f", "")
            .replace("%i", "")
            .replace("%c", "")
            .replace("%k", "")
            .trim()
            .to_string()
    }
}