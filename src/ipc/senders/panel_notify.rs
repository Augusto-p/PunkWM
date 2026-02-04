// use crate::utils::config::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::ipc::socket::socket_send_dock;
use serde_json::json;
use crate::utils::notifications::Notification;

pub fn sender_panel_notify_new(noty: Notification){
    let msg = IpcMessage::new("Panel:Notify","New", serde_json::to_value(&noty).unwrap());
    let _ = socket_send_dock(&msg);
}