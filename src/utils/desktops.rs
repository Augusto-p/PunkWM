use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use std::{
    collections::HashMap,
    fs,
};

#[derive(Debug,Clone)]
enum Value {U(String),M(Vec<String>),}
impl Value {
    fn as_string(&self) -> String {
        match self {
            Value::U(s) => s.clone(),
            Value::M(v) => v.join(" "),
        }
    }
}
#[derive(Clone)]
enum Mode {Entry,Action,}

#[derive(Debug , Clone)]
pub struct Actions {
    pub name: String,
    pub entry: HashMap<String, Value>,
}
impl Actions {
    pub fn new(name: String, entry: HashMap<String, Value>) -> Self {
        Self { name, entry }
    }
}

#[derive(Debug, Clone)]
pub struct Desktop {
    pub package: String,
    pub entry: Option<HashMap<String, Value>>,
    pub actions: Vec<Actions>,
}
impl Desktop {
    pub fn new(package: String) -> Self {
        Self {
            package,
            entry: None,
            actions: Vec::new(),
        }
    }

    pub fn parse(path: &str) -> Option<Self> {
        let text = fs::read_to_string(path).ok()?;

        let mut desktop = Desktop::new(path.replace(".desktop", ""));

        let mut in_section = false;
        let mut action_name = String::new();
        let mut map: HashMap<String, Value> = HashMap::new();
        let mut mode = Mode::Entry;

        for raw in text.lines() {
            let line = raw.trim();

            if line.starts_with('[') {
                // guardar sección previa
                if in_section {
                    desktop.save_section(&mode, &mut action_name, map);
                }

                map = HashMap::new();

                if line == "[Desktop Entry]" {
                    mode = Mode::Entry;
                } else if line.starts_with("[Desktop Action ") {
                    mode = Mode::Action;
                    action_name = line
                        .trim_start_matches("[Desktop Action ")
                        .trim_end_matches(']')
                        .to_string();
                }

                in_section = true;
                continue;
            }

            if !in_section || line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim().to_string();

                if v.contains(';') {
                    let list = v
                        .split(';')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();

                    map.insert(key, Value::M(list));
                } else {
                    map.insert(key, Value::U(v.trim().to_string()));
                }
            }
        }

        // guardar última sección
        if in_section {
            desktop.save_section(&mode, &mut action_name, map);
        }

        Some(desktop)
    }

    fn save_section(
        &mut self,
        mode: &Mode,
        action_name: &mut String,
        map: HashMap<String, Value>,
    ) {
        match mode {
            Mode::Entry => self.entry = Some(map),
            Mode::Action => {
                self.actions.push(Actions::new(action_name.clone(), map));
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DockActions {
    pub name: String,
    pub exec: String,
}
impl DockActions {
    pub fn new(name: String, exec: String) -> Self {
        Self { name, exec }
    }
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DockDesktop {
    pub package: String,
    pub name: String,
    pub icon: String,
    pub exec: String,
    pub q: String,
    pub actions: Vec<DockActions>,
}
impl DockDesktop {
    pub fn new(package: String,name: String,icon: String,exec: String,q: String,actions: Vec<DockActions>,) -> Self {
        Self {package,name,icon,exec,q,actions,}
    }

    pub fn from_desktop(d: Desktop, lang: &str) -> Self {
        let entry = d.entry.unwrap_or_default();

        let name = get_localized(&entry, "Name", lang);
        let icon = entry.get("Icon").map(|v| v.as_string()).unwrap_or_default();
        let exec = entry.get("Exec").map(|v| v.as_string()).unwrap_or_default();
        let categories = entry.get("Categories").map(|v| v.as_string()).unwrap_or_default();

        let all_name = collect_localized(&entry, "Name");
        let all_generic = collect_localized(&entry, "GenericName");
        let all_comment = collect_localized(&entry, "Comment");
        let all_keywords = collect_localized(&entry, "Keywords");

        let q = format!(
            "{} {} {} {} {}",
            categories, all_comment, all_generic, all_keywords, all_name
        );

        let actions = d.actions
            .into_iter()
            .map(|a| DockActions {
                name: get_localized(&a.entry, "Name", lang),
                exec: a.entry.get("Exec").map(|v| v.as_string()).unwrap_or_default(),
            })
            .collect();

        DockDesktop {
            package: d.package,
            name,
            icon,
            exec,
            q,
            actions,
        }
    }
}

fn get_localized(map: &HashMap<String, Value>, key: &str, lang: &str) -> String {
    if let Some(v) = map.get(&format!("{}[{}]", key, lang)) {
        return v.as_string();
    }

    map.get(key)
        .map(|v| v.as_string())
        .unwrap_or_default()
}

fn collect_localized(map: &HashMap<String, Value>, key: &str) -> String {
    map.iter()
        .filter(|(k, _)| *k == key || k.starts_with(&format!("{}[", key)))
        .map(|(_, v)| v.as_string())
        .collect::<Vec<_>>()
        .join(" ")
}



pub fn get_all_desktops_paths(paths: Vec<String>) -> Vec<String> {
    let mut desktop_paths = Vec::new();

    for base in paths {
        let base = shellexpand::tilde(&base).to_string();

        for entry in WalkDir::new(base)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry
                .path()
                .extension()
                .map(|e| e == "desktop")
                .unwrap_or(false)
            {
                if let Some(p) = entry.path().to_str() {
                    desktop_paths.push(p.to_string());
                }
            }
        }
    }

    desktop_paths
}
