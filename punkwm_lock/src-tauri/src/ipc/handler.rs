use crate::ipc::handlers::system::ipc_handler_system;
use crate::ipc::message::IpcMessage;

pub fn ipc_handler(msg: IpcMessage) {
    match msg.category.as_str() {
        "System" =>{
            ipc_handler_system(msg);
        },
        _ => {
            println!(
                "Categoria desconocido: [{}]",
                msg.category
            );
        }
    }
}
