// use crate::apphandle::get_api_ipc;
use crate::ipc::socket::socket_send;
use crate::ipc_front::message::IpcFrontMessage;
use crate::utils::youtube::YTMusic;
use crate::IpcMessage;
// use punkwm_dock_lib::print_in_tty;
use serde_json::json;

pub fn ipc_front_handler_music_panel(msg: IpcFrontMessage) {
    match msg.get_name().as_str() {
        "YT:Quick picks" => {
            YTMusic::quick_picks(msg.get_data());
        }

        "YT:Next Songs" => {
            YTMusic::next_songs(msg.get_data());
        }

        "YT:Search" => {
            YTMusic::search(msg.get_data());
        }

        "YT:Start Song" => {
            YTMusic::start_song(msg.get_data());
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
            let command = IpcMessage::new("Panel:Music", "Local:Start:Song", msg.get_data());
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
            let command = IpcMessage::new("Panel:Music", "Local:Search:Song", msg.get_data());
            let _ = socket_send(&command);
        }

        _ => {
            println!("Nombre desconocido: [{}:{}]", msg.get_category(), msg.get_name());
        }
    }
}

