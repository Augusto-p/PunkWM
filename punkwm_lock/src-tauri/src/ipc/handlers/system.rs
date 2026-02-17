use crate::ipc::message::IpcMessage;
// use serde_json::json;
use punkwm_lock_lib::print_in_tty;

pub fn ipc_handler_system(msg: IpcMessage) {
     match msg.name.as_str() {
        "Poweroff" =>{
            let _ = print_in_tty("SYSTEM:Poweroff");
        },

        "Reboot" =>{
            let _ = print_in_tty("SYSTEM:Reboot");
        },

        "Login" =>{
            let user = msg.data["User"].as_str().unwrap().to_string();
            let password = msg.data["Password"].as_str().unwrap().to_string();
            let _ = print_in_tty(&format!("SYSTEM:\nUser: {}\nPassword: {}", user, password));
        },
        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.category,
                msg.name
            );
        }
     }
    
}