use pam::Client;
use libc::{getpwuid, geteuid};
use std::ffi::CStr;
use std::fs;
use std::env;
use toml::Value;


pub struct SysUser;

impl SysUser{

    pub fn get_user()->String{
        if let Some(comment) = SysUser::get_user_comment(){
            return comment
        }else{
            return whoami::username();
        }
    }

    pub fn get_photo() -> String {
        let photo = (|| {
            let home = env::var("HOME").ok()?;
            let path = format!("{}/.config/punkwm/usr", home);
            let content = fs::read_to_string(path).ok()?;
            let data: Value = toml::from_str(&content).ok()?;

            data["usr"]["photo"].as_str().map(|s| s.to_string())
        })();

        photo.unwrap_or_default()
    }
    

    fn get_user_comment() -> Option<String> {
        unsafe {
            let pw = getpwuid(geteuid());
            if pw.is_null() {
                return None;
            }

            let gecos = CStr::from_ptr((*pw).pw_gecos);
            Some(gecos.to_string_lossy().into_owned())
        }
    }

    pub fn auth(password: &str) -> bool {
        let user = whoami::username();

        let mut client = match Client::with_password("system-auth") {
            Ok(c) => c,
            Err(_) => return false,
        };

        client.conversation_mut().set_credentials(&user, password);
        client.authenticate().is_ok()
    }

}

