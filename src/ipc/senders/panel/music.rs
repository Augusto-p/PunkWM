// use crate::utils::config::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::ipc::socket::socket_send_dock;
use crate::utils::song::Song;
use serde_json::json;


pub fn sender_panel_music_local_load_songs(songs: Vec<Song>){
    let msg = IpcMessage::new("Panel:Music","Local:Load Songs", json!({"songs": songs}));
    let _ = socket_send_dock(&msg);
}

pub fn sender_panel_music_local_current_time_song(current_time: u128){
    let msg = IpcMessage::new("Panel:Music","Local:Current Time Song", json!({"current_time": current_time}));
    let _ = socket_send_dock(&msg);
}