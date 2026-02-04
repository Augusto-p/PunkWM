// use punkwm_dock_lib::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::apphandle::get_api_ipc;


pub fn handler_panel_home(msg: IpcMessage) {
     match msg.name.as_str() {
        "System:Stats" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }
        "Panel:Load" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }
        "Weather:Load" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }
        "Google:Daily" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }
        "Google:Oauth:url" => {
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