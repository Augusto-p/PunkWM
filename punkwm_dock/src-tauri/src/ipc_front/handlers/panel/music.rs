// use crate::apphandle::get_api_ipc;
use crate::ipc::socket::socket_send;
use crate::ipc_front::message::IpcFrontMessage;
use crate::utils::youtube::YTMusic;
use crate::IpcMessage;
// use punkwm_dock_lib::print_in_tty;
use serde_json::json;

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
        "Local:Search:Song" => {
            let command = IpcMessage::new("Panel:Music", "Local:Search:Song", msg.data);
            let _ = socket_send(&command);
        }

        _ => {
            println!("Nombre desconocido: [{}:{}]", msg.category, msg.name);
        }
    }
}

