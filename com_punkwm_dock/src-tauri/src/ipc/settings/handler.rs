use crate::ipc::message::IpcMessage;
use crate::ipc::settings::handlers::admin::handler_admin;
use crate::ipc::settings::handlers::apps::handler_apps;
use crate::ipc::settings::handlers::dock_pannel::handler_dock_pannel;
use crate::ipc::settings::handlers::google::handler_google;
use crate::ipc::settings::handlers::keybindings::handler_keybindings;
use crate::ipc::settings::handlers::language::handler_language;
use crate::ipc::settings::handlers::layout::handler_layout;
use crate::ipc::settings::handlers::music_folders::handler_music_folders;
use crate::ipc::settings::handlers::sys::handler_system;
use crate::ipc::settings::handlers::user::handler_user;
use crate::ipc::settings::handlers::wallpapers::handler_wallpapers;
use crate::ipc::settings::handlers::weather::handler_weather;

pub fn settings_handler(msg: IpcMessage) {
    if msg.bridge() {
        let msgwm: IpcMessage = msg.into();
        println!("{:?}", msgwm);
    } else {
        match msg.category().as_str() {
            "System" => {
                handler_system(msg);
            }
            "User" => {
                handler_user(msg);
            }
            "Language" => {
                handler_language(msg);
            }
            "Layout" => {
                handler_layout(msg);
            }
            "Wallpapers" => {
                handler_wallpapers(msg);
            }
            "Admin" => {
                handler_admin(msg);
            }
            "Weather" => {
                handler_weather(msg);
            }
            "DockPannel" => {
                handler_dock_pannel(msg);
            }
            "Google" => {
                handler_google(msg);
            }
            "Music Folders" => {
                handler_music_folders(msg);
            }
            "APPs" => {
                handler_apps(msg);
            }
            "Keybindings" => {
                handler_keybindings(msg);
            }
            _ => {
                println!("Categoria desconocido: [{}]", msg.category());
            }
        }
    }
}
