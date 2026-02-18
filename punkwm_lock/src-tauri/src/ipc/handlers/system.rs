use punkwm_lock_lib::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::utils::auth::login_auth;
use serde_json::json;
use crate::utils::config::load_config;
use crate::apphandle::get_api_ipc;

pub fn ipc_handler_system(msg: IpcMessage) {
     match msg.name.as_str() {
        "Poweroff" =>{
            let _ = print_in_tty("SYSTEM:Poweroff");
        },

        "Reboot" =>{
            let _ = print_in_tty("SYSTEM:Reboot");
        },
        "Start" =>{
            let config = load_config();
            let message = IpcMessage::new("System", "Set Background", json!({"bg": config.lock_screen.bg}));
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(message.clone());
        },
        "Login" =>{
            let user = msg.data["User"].as_str().unwrap().to_string();
            let password = msg.data["Password"].as_str().unwrap().to_string();
            if login_auth(&user, &password) {
                let _ = print_in_tty(&format!("\nSYSTEM:\nUser: {}\nPassword: {}", user, password));
            } else {
                let _ = print_in_tty(&format!("\nSYSTEM Error:\nUser: {}\nPassword: {}", user, password));
            }
            
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