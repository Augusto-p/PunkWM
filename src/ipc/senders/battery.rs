// use crate::utils::config::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::ipc::socket::socket_send_dock;
use serde_json::json;
use crate::utils::battery::Battery;
pub fn sender_battery_update(battery: Battery ){
    let msg = IpcMessage::new("Battery","Update", json!({"charging": battery.charging, "percentage": battery.percentage}));
    let _ = socket_send_dock(&msg);
}
