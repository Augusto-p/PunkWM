
use crate::ipc::message::IpcMessage;
use crate::utils::config::print_in_tty;
use crate::custom_event::main_thread_notifier::MainThreadNotifier;
use crate::custom_event::entity::CustomEvent;
use crate::localaudio::entity::LocalAudioCommand;
pub fn handler_music_panel(msg: IpcMessage,notifier: &MainThreadNotifier) {
     match msg.name.as_str() {
        "Local:Load:Song" => {
            notifier.send(CustomEvent::SongsLocalLoad());
        }
        "Local:Start:Song" => {
            let path = msg.data["path"].as_str().unwrap().to_string();
            notifier.send(LocalAudioCommand::Load(path));
        }
        "Local:Play:Song" => {
            notifier.send(LocalAudioCommand::Play());
        }
        "Local:Pause:Song" => {
            notifier.send(LocalAudioCommand::Pause());
        }
        "Local:Reset:Song" => {
            notifier.send(LocalAudioCommand::Reset());
        }
        "Local:Stop:Song" => {
            notifier.send(LocalAudioCommand::Stop());
        }
        






        
        _ => {
            let t = format!("Nombre desconocido: [{}:{}]",
                msg.category,
                msg.name
            );
            let _ = print_in_tty(&t);
        }
     }
    
}