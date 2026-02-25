// use punkwm_dock_lib::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::apphandle::get_api_ipc;

pub fn handler_panel_music(msg: IpcMessage) {
     match msg.name.as_str() {
        "Local:Load Songs" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }
        "Local:Current Time Song" => {
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