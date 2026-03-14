use crate::ipc::socket::socket_send;
use crate::IpcMessage;
use serde_json::json;
use crate::ipc_front::message::IpcFrontMessage;


pub fn ipc_front_handler_apps_panel(msg: IpcFrontMessage) {
     match msg.get_name().as_str() {
        "Open" =>{
            let command = IpcMessage::new("Panel:Apps", "Open", json!({}));
            let _ = socket_send(&command);
        },
        
        "Search" =>{
            let command = IpcMessage::new("Panel:Apps", "Search", msg.get_data());
            let _ = socket_send(&command);
        },

        "Load Apps" =>{
            let command = IpcMessage::new("Panel:Apps", "Load Apps", json!({}));
            let _ = socket_send(&command);
        },

        "Open App" =>{
            let command = IpcMessage::new("Panel:Apps", "Open App", msg.get_data());
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



