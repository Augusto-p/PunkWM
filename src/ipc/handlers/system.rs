use crate::ipc::message::IpcMessage;
use crate::spawn;
use crate::utils::config::print_in_tty;
use crate::MainThreadNotifier;
use crate::custom_event::entity::CustomEvent;

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
            let _ = print_in_tty("WM Socket: Set Volume");
        },
        "Set Glow"=>{
            let _ = print_in_tty("WM Socket: Set Glow");
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