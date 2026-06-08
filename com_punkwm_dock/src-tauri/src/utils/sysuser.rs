// use pam::Client;
use libc::{getpwuid, geteuid};
use std::ffi::CStr;
use std::fs;
use std::env;
use toml::Value;
use crate::get_api_ipc;

pub struct SysUser;

impl SysUser{

    pub fn get_user()->String{
        if let Some(comment) = SysUser::get_user_comment(){
            return comment
        }else{
            return whoami::username().unwrap();
        }
    }

    pub fn get_photo() -> String {
        let api = get_api_ipc();
        return api.with_config(|cfg| {
            let photo = (|| {
                let path = format!("{}/usr", cfg.folder());
                let content = fs::read_to_string(path).ok()?;
                let data: Value = toml::from_str(&content).ok()?;

                data["usr"]["photo"].as_str().map(|s| s.to_string())
            })();

            return photo.unwrap_or_default();
        });
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

}
