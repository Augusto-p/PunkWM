use crate::{ipc::message::IpcMessage};
use crate::apphandle::get_api_ipc;
// use punkwm_dock_lib::print_in_tty;

pub fn handler_apps(msg: IpcMessage) {
     match msg.name().as_str() {
        "Add App" =>{
            let name = msg.data()["Name"].to_string();
            let command = msg.data()["Command"].to_string();
            let api = get_api_ipc();
            api.with_config(|cfg| {
                cfg.apps().set_app(name, command);
            });

        },
        "Pop App" =>{
            let key = msg.data()["Key"].to_string();
            let api = get_api_ipc();
            api.with_config(|cfg| {
                cfg.apps().pop_app(key);
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