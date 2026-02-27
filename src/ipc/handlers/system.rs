use crate::ipc::message::IpcMessage;
use crate::spawn;
use crate::utils::brightness::{Brightness};
use crate::utils::config::print_in_tty;
use crate::MainThreadNotifier;
use crate::custom_event::entity::CustomEvent;
use crate::utils::volume::Volume;

pub fn handler_system(msg: IpcMessage, notifier: &MainThreadNotifier) {
     match msg.name.as_str() {
        "Poweroff" => {
            spawn("poweroff");
        },
        "Reboot"=>{
            spawn("reboot");
        },
        "Log Out"=>{
            let _ = print_in_tty("WM Socket: Log Out");
        },
        "Lock"=>{
            let _ = print_in_tty("WM Socket: Lock");
        },

        "Start Dock"=>{notifier.send(CustomEvent::StartDock());}
        "Open Panel"=>{notifier.send(CustomEvent::OpenPanel());}
        "Close Panel"=>{notifier.send(CustomEvent::ClosePanel());}

        "Set Volume"=>{
            if let Some(volume_value) = msg.data.get("Volume") {
                if let Some(volume) = volume_value.as_u64() {
                    Volume::set(volume as u8);
                }
            }
        },
        "Set Glow"=>{
            if let Some(glow_value) = msg.data.get("Glow") {
                if let Some(glow) = glow_value.as_u64() {
                    Brightness::set(glow as u8);
                }
            }
        },
        _ => {
            let t = format!("Nombre desconocido: [{}:{}]",
                msg.category,
                msg.name
            );
            let _ = print_in_tty(&t);
        }
     }
    
}