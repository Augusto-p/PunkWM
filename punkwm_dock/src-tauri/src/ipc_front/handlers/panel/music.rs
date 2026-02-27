use crate::apphandle::get_api_ipc;
use crate::ipc::socket::socket_send;
use crate::ipc_front::message::IpcFrontMessage;
use crate::utils::cookies::{parse_cookies, serialize_cookies, CookieSearch};
use crate::utils::youtube::YTMusic;
use crate::IpcMessage;
use punkwm_dock_lib::print_in_tty;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

pub fn ipc_front_handler_music_panel(msg: IpcFrontMessage) {
    match msg.name.as_str() {
        "YT:Quick picks" => {
            YTMusic::quick_picks(msg.data.clone());
        }

        "YT:Next Songs" => {
            YTMusic::next_songs(msg.data.clone());
        }

        "YT:Search" => {
            YTMusic::search(msg.data.clone());
        }

        "YT:Start Song" => {
            YTMusic::start_song(msg.data.clone());
        }

        "YT:Play Song" => {
            YTMusic::play();
        }

        "YT:Pause Song" => {
            YTMusic::pause();
        }

        "YT:Status" => {
            YTMusic::status();
        }

        "YT:Stop" => {
            YTMusic::stop();
        }

        "YT:Start" => {
            YTMusic::start();
        }

        "Local:Load:Song" => {
            let command = IpcMessage::new("Panel:Music", "Local:Load:Song", json!({}));
            let _ = socket_send(&command);
        }

        "Local:Start:Song" => {
            let command = IpcMessage::new("Panel:Music", "Local:Start:Song", msg.data);
            let _ = socket_send(&command);
        }
        "Local:Play:Song" => {
            let command = IpcMessage::new("Panel:Music", "Local:Play:Song", json!({}));
            let _ = socket_send(&command);
        }
        "Local:Pause:Song" => {
            let command = IpcMessage::new("Panel:Music", "Local:Pause:Song", json!({}));
            let _ = socket_send(&command);
        }
        "Local:Reset:Song" => {
            let command = IpcMessage::new("Panel:Music", "Local:Reset:Song", json!({}));
            let _ = socket_send(&command);
        }
        "Local:Stop:Song" => {
            let command = IpcMessage::new("Panel:Music", "Local:Stop:Song", json!({}));
            let _ = socket_send(&command);
        }

        _ => {
            println!("Nombre desconocido: [{}:{}]", msg.category, msg.name);
        }
    }
}

fn json_to_cookie_header(json_str: &str) -> String {
    let parsed: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };

    let mut cookies = Vec::new();

    if let Some(obj) = parsed.as_object() {
        for (key, value) in obj {
            if let Some(val_str) = value.as_str() {
                cookies.push(format!("{}={}", key, val_str));
            }
        }
    }

    cookies.join("; ")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YTSong {
    pub id: String,
    pub cover: String,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub mode: String,
    pub duration: Option<String>,
}
