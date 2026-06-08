use crate::{ipc::message::IpcMessage};
use crate::apphandle::get_api_ipc;
// use punkwm_dock_lib::print_in_tty;

pub fn handler_keybindings(msg: IpcMessage) {
     match msg.name().as_str() {
        "Save" =>{
            let api = get_api_ipc();
            let command = msg.data()["Command"].to_string().replace("\"", "");
            let keys = msg.data()["Keys"].to_string().replace("\"", "");
            api.with_config(|cfg| {
               cfg.keybindings().set_keybinding(command, keys);
            });
        },
        
        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.category(),
                msg.name()
            );
        }
     }
    
}