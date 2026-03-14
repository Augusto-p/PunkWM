use crate::ipc::message::IpcMessage;
use crate::utils::config::print_in_tty;
use crate::custom_event::main_thread_notifier::MainThreadNotifier;
use crate::custom_event::entity::CustomEvent;
pub fn handler_home_panel(msg: IpcMessage,notifier: &MainThreadNotifier) {
     match msg.get_name().as_str() {
        "Open" => {
            notifier.send(CustomEvent::OpenHomePanel());
        }
        "Google:Diary:Refresh" => {
            notifier.send(CustomEvent::HomePanelLoadDaily());
        }
        "Google:Oauth:Code" => {
            let code = msg.get_data()["code"].as_str().unwrap().to_string();
            notifier.send(CustomEvent::GoogleOauthLogin(code));
        }

        _ => {
            let t = format!("Nombre desconocido: [{}:{}]",
                msg.get_category(),
                msg.get_name()
            );
            let _ = print_in_tty(&t);
        }
     }
    
}