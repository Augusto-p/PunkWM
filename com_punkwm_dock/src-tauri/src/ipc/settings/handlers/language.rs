use crate::ipc::message::IpcMessage;
use crate::apphandle::get_api_ipc;
// use punkwm_dock_lib::print_in_tty;

pub fn handler_language(msg: IpcMessage) {
     match msg.name().as_str() {
        "Save" =>{
            let api = get_api_ipc();
            let language = msg.data()["Id"].to_string();
            api.with_config(|cfg| {
                cfg.styles().set_lang(language)
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