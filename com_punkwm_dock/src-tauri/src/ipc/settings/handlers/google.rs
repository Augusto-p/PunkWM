use crate::{ipc::message::IpcMessage, utils::file::LocalFile};
use crate::apphandle::get_api_ipc;
// use punkwm_dock_lib::print_in_tty;

pub fn handler_google(msg: IpcMessage) {
     match msg.name().as_str() {
        "Save Max Events View" =>{
            let api = get_api_ipc();
            let max: u16 = msg.data()["Max"].to_string().replace("\"", "").parse::<u16>().unwrap_or(5);
            api.with_config(|cfg| {
               cfg.google().set_events(max);
            });
        },
        "Add Scope" =>{
            let api = get_api_ipc();
            let scope = msg.data()["Scope"].to_string();
            api.with_config(|cfg| {
               cfg.google().add_scope(scope);
            });

        },
        "Pop Scope" =>{
            let api = get_api_ipc();
            let scope = msg.data()["Scope"].to_string();
            api.with_config(|cfg| {
               cfg.google().pop_scope(scope);
            });

        },
        "Save Credentials" =>{
            let api = get_api_ipc();
            let credentials = msg.data()["Credentials"].to_string();
            api.with_config(|cfg| {
                let path = LocalFile::save_json_text_file(format!("{}/Credentials",cfg.folder()), credentials);
                cfg.google().set_credentials(path);
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