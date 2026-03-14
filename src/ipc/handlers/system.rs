use crate::ipc::message::IpcMessage;
use crate::spawn;
use crate::utils::brightness::{Brightness};
use crate::utils::config::print_in_tty;
use crate::MainThreadNotifier;
use crate::custom_event::entity::CustomEvent;
use crate::utils::volume::Volume;


pub fn handler_system(msg: IpcMessage, notifier: &MainThreadNotifier) {
     match msg.get_name().as_str() {
        "Poweroff" => {
            spawn("poweroff");
        },
        "Reboot"=>{
            spawn("reboot");
        },
        "Log Out"=>{
            let _ = print_in_tty("WM Socket: Log Out");
        },
        

        "Auth"=>{
            if let Some(password) = msg.get_data().get("password") {
                notifier.send(CustomEvent::LockAuth(password.to_string()));
            }
        }
        "Start Lock"=>{notifier.send(CustomEvent::DockLock());}
        "Start Dock"=>{notifier.send(CustomEvent::StartDock());}
        "Open Panel"=>{notifier.send(CustomEvent::OpenPanel());}
        "Close Panel"=>{notifier.send(CustomEvent::ClosePanel());}

        "Set Volume"=>{
            if let Some(volume_value) = msg.get_data().get("Volume") {
                if let Some(volume) = volume_value.as_u64() {
                    Volume::set(volume as u8);
                }
            }
        },
        "Set Glow"=>{
            if let Some(glow_value) = msg.get_data().get("Glow") {
                if let Some(glow) = glow_value.as_u64() {
                    Brightness::set(glow as u8);
                }
            }
        },
        _ => {
            let t = format!("Nombre desconocido: [{}:{}]",
                msg.get_category(),
                msg.get_name()
            );
            let _ = print_in_tty(&t);
        }
     }
    
}