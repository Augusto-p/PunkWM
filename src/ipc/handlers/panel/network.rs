use crate::ipc::message::IpcMessage;
use crate::utils::config::print_in_tty;
use crate::custom_event::main_thread_notifier::MainThreadNotifier;
use crate::custom_event::entity::CustomEvent;

pub fn handler_network_panel(msg: IpcMessage,notifier: &MainThreadNotifier) {
     match msg.get_name().as_str() {
        "Open" => {
            notifier.send(CustomEvent::OpenNetworkPanel());
        }
        "Refresh" => {
            notifier.send(CustomEvent::NetworkPanelLoadWiFi());
        }
        "Connect Public WiFi" => {
            let ssid = msg.get_data()["SSID"].as_str().unwrap().to_string();
            notifier.send(CustomEvent::NetworkPanelConnectWiFiPublic(ssid));
        }
        "Disconnect WiFi" => {
            notifier.send(CustomEvent::NetworkPanelDisconnectWiFi());
        }
        "Connect WiFi" => {
            let ssid = msg.get_data()["SSID"].as_str().unwrap().to_string();
            let password = msg.get_data()["Password"].as_str().unwrap().to_string();
            notifier.send(CustomEvent::NetworkPanelConnectWiFi(ssid, password));
        }
        "Share WiFi" => {
            notifier.send(CustomEvent::NetworkPanelShareWiFi());
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