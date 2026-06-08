// use crate::apphandle::get_api_ipc;
use crate::utils::youtube::YTMusic;
use crate::IpcMessage;
// use crate::ipc::message::IpcMessage;

// use punkwm_dock_lib::print_in_tty;
// use serde_json::json;

pub fn handler_music_panel(msg: IpcMessage) {
    match msg.name().as_str() {
        "YT:Quick picks" => {
            YTMusic::quick_picks(msg.data());
        }

        "YT:Next Songs" => {
            YTMusic::next_songs(msg.data());
        }

        "YT:Search" => {
            YTMusic::search(msg.data());
        }

        "YT:Start Song" => {
            YTMusic::start_song(msg.data());
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

        _ => {
            println!("Nombre desconocido: [{}:{}]", msg.category(), msg.name());
        }
    }
}

