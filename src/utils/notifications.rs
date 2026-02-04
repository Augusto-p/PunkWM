use crate::utils::config::print_in_tty;
use zbus::{ConnectionBuilder, interface};
use std::collections::HashMap;
use zbus::zvariant::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::ipc::senders::panel_notify::sender_panel_notify_new;
use serde::{Deserialize, Serialize};

struct NotificationServer;

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Notification {
    pub app: String,
    pub icon: String,
    pub title: String, 
    pub message: String,
    pub now: u64,
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationServer {

    fn get_capabilities(&self) -> Vec<&str> {
        vec!["body", "actions"]
    }

    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("RustNotify", "Rust", "1.0", "1.2")
    }

    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        _actions: Vec<&str>,
        _hints: HashMap<&str, Value>,
        expire_timeout: i32,
    ) -> u32 {

        // let _ = print_in_tty(&format!("🔔 NOTIFICACIÓN"));
        // let _ = print_in_tty(&format!("App: {}", app_name));
        // let _ = print_in_tty(&format!("ID: {}", replaces_id));
        // let _ = print_in_tty(&format!("Icon: {}", app_icon));
        // let _ = print_in_tty(&format!("Título: {}", summary));
        // let _ = print_in_tty(&format!("Mensaje: {}", body));
        // let _ = print_in_tty(&format!("Timeout: {}", expire_timeout));
        // let _ = print_in_tty(&format!("------------------------"));
        let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(), // Devuelve u64
            Err(_) => 0,          // Devuelve u64 (ahora sí coinciden)
        };
        let noty:Notification = Notification{
                app: app_name.to_string(),
                icon: app_icon.to_string(),
                title: summary.to_string(),
                message: body.to_string(), 
                now: time
        };
        sender_panel_notify_new(noty);
    

        1
    }

    fn close_notification(&self, id: u32) {
        let _ = print_in_tty(&format!("Cerrar notificación {}", id));
    }
}

pub async fn listen_notifications() -> zbus::Result<()> {

    let _conn = ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at(
            "/org/freedesktop/Notifications",
            NotificationServer
        )?
        .build()
        .await?;

    // let _ = print_in_tty(&format!("✅ Servidor de notificaciones activo"));

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
