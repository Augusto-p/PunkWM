use crate::ipc::socket::socket_send;
use crate::IpcMessage;
use crate::ipc_front::message::IpcFrontMessage;

pub fn ipc_front_handler_workspace(msg: IpcFrontMessage) {
     match msg.get_name().as_str(){
        "Set" =>{
            let command = IpcMessage::new("Workspace", "Set", msg.get_data());
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