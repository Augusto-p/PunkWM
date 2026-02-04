// use crate::utils::config::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::ipc::socket::socket_send_dock;
use crate::DeviceState;
// use serde_json::json;

pub fn sender_network_deveice_state(state: DeviceState ){
    let msg = IpcMessage::new("Network","Device:State", serde_json::to_value(&state).unwrap());
    let _ = socket_send_dock(&msg);
}
