
use base64::{engine::general_purpose, Engine as _};
use punkwm_lock_lib::print_in_tty;
use serde::{Deserialize,Serialize};
use std::ffi::{CString, CStr};
use libc::{getpwnam, passwd};
use std::fs;
use pam::Client;
use serde_json::Value;


#[derive(Debug,Deserialize,Serialize, Clone)]
pub struct UserInfo {
    username: String,
    comment: String,
    photo: String,
}

impl UserInfo{
    pub fn new(username: String, comment: String,photo: String) ->Self{
        Self{username, comment, photo}
    }



    pub fn get_users() -> Vec<UserInfo> {
        let shadow = match fs::read_to_string("/etc/shadow") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let mut users = Vec::new();

        for line in shadow.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() < 2 {
                continue;
            }

            let username = parts[0];
            let password = parts[1];

            // Solo usuarios con contraseña activa
            if password != "!" && password != "*" && !password.is_empty() && password != "!*" {
                let comment = Self::get_user(username);
                let photo = Self::get_photo(username);

                users.push(UserInfo::new(
                    username.to_string(),
                    comment,
                    photo,
                ));
            }
        }

        users
    }


    pub fn auth(user : &str, password : &str) -> bool{
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
    pub fn get_photo(username: &str) -> String {
        let path = Self::get_photo_path(username);

        let data = match fs::read(path) {
            Ok(d) => d,
            Err(_) => return "".to_string(),
        };

        format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(data))
    }

    pub fn get_photo_path(username: &str) -> String {
        let photo = (|| {
            let home = Self::get_user_home(username);
            let path = format!("{}/.config/punkwm/usr", home);
            let content = fs::read_to_string(path).ok()?;
            let data: Value = toml::from_str(&content).ok()?;

            data["usr"]["photo"].as_str().map(|s| s.to_string())
        })();

        photo.unwrap_or_default()
    }

    pub fn get_user(username: &str) -> String {
        if let Some(comment) = Self::get_user_comment(username) {
            comment
        } else {
            username.to_string()
        }
    }

    fn get_user_comment(username: &str) -> Option<String> {
        let c_username = CString::new(username).ok()?;

        unsafe {
            let pw: *mut passwd = getpwnam(c_username.as_ptr());
            if pw.is_null() {
                return Some(username.to_string());
            }

            let gecos = CStr::from_ptr((*pw).pw_gecos);
            Some(gecos.to_string_lossy().into_owned())
        }
    }


    fn get_user_home(username: &str) -> String {
        let c_username = match CString::new(username) {
            Ok(u) => u,
            Err(_) => return "".to_string(),
        };

        unsafe {
            let pw: *mut passwd = getpwnam(c_username.as_ptr());
            if pw.is_null() {
                return "".to_string();
            }

            let dir = CStr::from_ptr((*pw).pw_dir);
            dir.to_string_lossy().into_owned()
        }
    }
}
