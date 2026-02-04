use crate::ipc::socket::socket_send;
use crate::IpcMessage;
use crate::ipc_front::message::IpcFrontMessage;

pub fn ipc_front_handler_workspace(msg: IpcFrontMessage) {
     match msg.name.as_str() {
        "Set" =>{

            let command = IpcMessage::new("Workspace", "Set", msg.data);
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