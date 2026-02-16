// use punkwm_dock_lib::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::apphandle::get_api_ipc;


pub fn handler_panel_network(msg: IpcMessage) {
     match msg.name.as_str() {
        "Load:WiFi" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }
        "Share:WiFi" => {
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