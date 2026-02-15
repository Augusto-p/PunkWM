// use crate::utils::config::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::ipc::socket::socket_send_dock;
use crate::DockDesktop;
use serde_json::json;


pub fn sender_panel_apps_load_apps(apps: Vec<DockDesktop>){
    let msg = IpcMessage::new("Panel:Apps","Load:Apps", json!({"Apps": apps}));
    let _ = socket_send_dock(&msg);
}