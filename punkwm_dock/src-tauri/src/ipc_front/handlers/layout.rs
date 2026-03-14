use crate::ipc::socket::socket_send;
use crate::IpcMessage;
use serde_json::json;
use crate::ipc_front::message::IpcFrontMessage;

pub fn ipc_front_handler_layout(msg: IpcFrontMessage) {
     match msg.get_name().as_str() {
        "Toggle" =>{
            let command = IpcMessage::new("Layout", "Toggle", json!({}));
            let _ = socket_send(&command);
        },
        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.get_category(),
                msg.get_name()
            );
        }
     }
    
}