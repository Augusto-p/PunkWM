use punkwm_lock_lib::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::utils::auth::login_auth;
use serde_json::json;
use crate::utils::config::load_config;
use crate::apphandle::get_api_ipc;
use crate::utils::wm::launch_wm_with_env;
use crate::utils::tools::spawn;

pub fn ipc_handler_system(msg: IpcMessage) {
     match msg.name.as_str() {
        "Poweroff" =>{
            spawn("poweroff");
        },

        "Reboot" =>{
            spawn("reboot");
        },
        "Start" =>{
            let config = load_config();
            let message = IpcMessage::new("System", "Set Background", json!({"bg": config.lock_screen.bg}));
            let api_ipc = get_api_ipc();
            let _ = api_ipc.emit(message.clone());
        },
        "Login" =>{
            let username = msg.data["User"].as_str().unwrap().to_string();
            let password = msg.data["Password"].as_str().unwrap().to_string();
            if login_auth(&username, &password) {
                match nix::unistd::User::from_name(&username) {
                    Ok(Some(user)) => {
                        let uid = user.uid.as_raw();
                        launch_wm_with_env(uid, "/home/augus/PunkWM/punk_wm");
                    }

                    Ok(None) => {
                        let _ = print_in_tty(
                            "SYSTEM ERROR: Usuario no encontrado en /etc/passwd"
                        );
                    }

                    Err(e) => {
                        let _ = print_in_tty(&format!(
                            "SYSTEM ERROR (nix): {}",
                            e
                        ));
                    }
                }
            } else {
                let _ = print_in_tty("SYSTEM: Login incorrecto");
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