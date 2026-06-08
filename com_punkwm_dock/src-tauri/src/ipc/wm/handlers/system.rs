// use punkwm_dock_lib::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::apphandle::get_api_ipc;
use crate::utils::lockwindow::LockWin;


pub fn handler_system(msg: IpcMessage) {
     match msg.name().as_str() {
       "Lock:Valid" => {LockWin::close();}
        _ => {
            println!(
                "Nombre desconocido::: [{}:{}]",
                msg.category(),
                msg.name()
            );
        }
     }
    
}