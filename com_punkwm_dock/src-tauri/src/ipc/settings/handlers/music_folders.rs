use crate::{ipc::message::IpcMessage};
use crate::apphandle::get_api_ipc;
// use punkwm_dock_lib::print_in_tty;

pub fn handler_music_folders(msg: IpcMessage) {
     match msg.name().as_str() {
        "Add Folder" =>{
            let api = get_api_ipc();
            let folder = msg.data()["Folder"].to_string();
            api.with_config(|cfg| {
               cfg.styles().add_music_folders(folder);
            });

        },
        "Pop Folder" =>{
            let api = get_api_ipc();
            let folder = msg.data()["Folder"].to_string();
            api.with_config(|cfg| {
               cfg.styles().pop_music_folders(folder);
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