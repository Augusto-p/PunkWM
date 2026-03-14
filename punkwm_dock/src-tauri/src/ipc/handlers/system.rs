// use punkwm_dock_lib::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::apphandle::get_api_ipc;
use punkwm_dock_lib::print_in_tty;
use crate::utils::lockwindow::LockWin;


pub fn handler_system(msg: IpcMessage) {
     match msg.get_name().as_str() {
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

        "Set Volume" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }
        "Set Glow" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(msg.clone());
        }

        "Set User LockScreen" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit_lock(msg.clone());
        }

        "Set Background LockScreen" => {
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit_lock(msg.clone());
        }

        "Lock:Valid" => {
            print_in_tty("helllo");
            LockWin::close();
        }
        _ => {
            println!(
                "Nombre desconocido::: [{}:{}]",
                msg.get_category(),
                msg.get_name()
            );
        }
     }
    
}