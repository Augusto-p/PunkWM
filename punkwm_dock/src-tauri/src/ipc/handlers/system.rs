// use punkwm_dock_lib::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::apphandle::get_api_ipc;



pub fn handler_system(msg: IpcMessage) {
     match msg.name.as_str() {
        "Panel:Load" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());

        }
        
        "Panel:Open" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }

        "Panel:Close" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }

        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.category,
                msg.name
            );
        }
     }
    
}