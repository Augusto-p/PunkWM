// use crate::utils::config::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::ipc::socket::socket_send_dock;
use serde_json::json;

pub fn sender_system_load_panel(dock_width: u32, panel_width: u32) {
    let msg = IpcMessage::new(
        "System",
        "Panel:Load",
        json!({"dock_width": dock_width, "panel_width": panel_width}),
    );
    let _ = socket_send_dock(&msg);
}

pub fn sender_system_volume(volume: u8) {
    let msg = IpcMessage::new("System", "Set:Volume", json!({"Volume": volume}));
    let _ = socket_send_dock(&msg);
}
pub fn sender_system_glow(glow: u8) {
    let msg = IpcMessage::new("System", "Set:Glow", json!({"Glow": glow}));
    let _ = socket_send_dock(&msg);
}

// pub fn sender_system_panel_open(){
//     let msg = IpcMessage::new("System","Panel:Open", json!({}));
//     let _ = socket_send_dock(&msg);
// }

pub fn sender_system_panel_close() {
    let msg = IpcMessage::new("System", "Panel:Close", json!({}));
    let _ = socket_send_dock(&msg);
}
