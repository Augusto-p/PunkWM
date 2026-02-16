// use crate::utils::config::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::ipc::socket::socket_send_dock;
use crate::WiFiNetwork;
use serde_json::json;


pub fn sender_panel_network_load_wifi(wifis: Vec<WiFiNetwork>){
    let msg = IpcMessage::new("Panel:Network","Load:WiFi", json!({"WiFis": wifis}));
    let _ = socket_send_dock(&msg);
}

pub fn sender_panel_network_share_wifi(qr: String){
    let msg = IpcMessage::new("Panel:Network",":WiFi", json!({"QR": qr}));
    let _ = socket_send_dock(&msg);
}