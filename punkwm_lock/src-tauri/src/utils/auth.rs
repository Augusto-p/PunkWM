use pam::Client;
use std::process::Command;
use std::fs;
use std::path::Path;
use serde::Deserialize;
#[derive(Debug)]
pub struct UserInfo {
    pub username: String,
    pub comment: String,
    pub photo: Option<String>,
}
#[derive(Deserialize)]
struct PunkWMConfig {
    Photo: Option<String>,
}
pub fn login_auth(user : &str, password : &str) -> bool{
    let mut client = match Client::with_password("system-auth"){
        Ok(cli) => cli,
        Err(_) => return false
    };
    client.conversation_mut().set_credentials(user, password);
    match client.authenticate(){
        Ok(_)=> return true,
        Err(_)=> return false
    };
}
pub fn get_users_info() -> std::io::Result<Vec<UserInfo>> {
    let output = Command::new("passwd")
        .arg("-Sa")
        .output()?;
    if !output.status.success() {
        panic!("Error ejecutando passwd -Sa");
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let passwd_content = fs::read_to_string("/etc/passwd")?;
    let mut users = Vec::new();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "P" {
            let username = parts[0];
            if let Some((home, comment)) = get_home_and_comment(username, &passwd_content) {
                let config_path = Path::new(&home)
                    .join(".config/PunkWM/user");
                if config_path.exists() {
                    let photo = read_photo_from_toml(&config_path);
                    users.push(UserInfo {
                        username: username.to_string(),
                        comment,
                        photo,
                    });
                }
            }
        }
    }
    Ok(users)
}
fn get_home_and_comment(username: &str, passwd_content: &str) -> Option<(String, String)> {
    for line in passwd_content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 7 && parts[0] == username {
            return Some((
                parts[5].to_string(), // home
                parts[4].to_string(), // comentario (GECOS)
            ));
        }
    }
    None
}
fn read_photo_from_toml(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let parsed: PunkWMConfig = toml::from_str(&content).ok()?;
    parsed.Photo
}