use crate::ipc::socket::socket_send;
use crate::IpcMessage;
use serde_json::json;
use crate::ipc_front::message::IpcFrontMessage;


pub fn ipc_front_handler_network_panel(msg: IpcFrontMessage) {
     match msg.name.as_str() {
        "Open" =>{
            let command = IpcMessage::new("Panel:Network", "Open", json!({}));
            let _ = socket_send(&command);
        },
        
        "Refresh" =>{
            let command = IpcMessage::new("Panel:Network", "Refresh", json!({}));
            let _ = socket_send(&command);
        },

        "Connect Public WiFi" =>{
            let command = IpcMessage::new("Panel:Network", "Connect Public WiFi", msg.data);
            let _ = socket_send(&command);
        },

        "Disconnect WiFi" =>{
            let command = IpcMessage::new("Panel:Network", "Disconnect WiFi", json!({}));
            let _ = socket_send(&command);
        },

        "Connect WiFi" =>{
            let command = IpcMessage::new("Panel:Network", "Connect WiFi", msg.data);
            let _ = socket_send(&command);
        },

        "Share WiFi" =>{
            let command = IpcMessage::new("Panel:Network", "Share WiFi", json!({}));
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



