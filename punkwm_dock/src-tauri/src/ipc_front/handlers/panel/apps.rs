use crate::ipc::socket::socket_send;
use crate::IpcMessage;
use serde_json::json;
use crate::ipc_front::message::IpcFrontMessage;


pub fn ipc_front_handler_apps_panel(msg: IpcFrontMessage) {
     match msg.name.as_str() {
        "Open" =>{
            let command = IpcMessage::new("Panel:Apps", "Open", json!({}));
            let _ = socket_send(&command);
        },
        
        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.category,
                msg.name
            );
        }
     }
    
}



