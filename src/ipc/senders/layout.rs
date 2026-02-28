// use crate::utils::config::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::WorkspaceManager;
use crate::ipc::socket::socket_send_dock;
use serde_json::json;
pub fn sender_layout_set(wm: &mut WorkspaceManager){
    let layout = wm.layouts[wm.current];
    let msg = IpcMessage::new("Layout","Set", json!({"layout": layout.id()}));
    let _ = socket_send_dock(&msg);
}