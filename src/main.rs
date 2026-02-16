mod custom_event;
mod utils;
mod wm;
mod ipc;
mod network_manager;
mod google;

use crate::utils::config::print_in_tty;
use crate::utils::notifications::listen_notifications;
use utils::battery::Battery;
use crate::utils::{tools::spawn, battery::BatteryManager,system::system_usage,weather::get_weather,
    desktops::{Desktop, get_all_desktops_paths, DockDesktop, search_docks,find_by_package, },
};
use x11rb::{connection::Connection, protocol::{Event, xproto::*},};
use crate::wm::manager::WorkspaceManager;
use std::{thread, time::Duration, sync::{mpsc, Arc},};
use crate::custom_event::{main_thread_notifier::MainThreadNotifier, entity::CustomEvent,};
use crate::network_manager::{NetworkManager, Device, DeviceState, wifi::get_wifi_networks, NetworkConnection};
use crate::ipc::{server::start_ipc_server,
    senders::{  layout::sender_layout_set, battery::sender_battery_update, workspace::sender_workspace_update, 
        network::sender_network_deveice_state, 
        system::{sender_system_load_panel, sender_system_panel_close},
        panel::home::{sender_panel_home_weather_load, sender_panel_home_system_stats,sender_panel_home_google_calender_daily, sender_panel_home_google_oauth_url},
        panel::apps::sender_panel_apps_load_apps,
        panel::network::{sender_panel_network_load_wifi, sender_panel_network_share_wifi},
    },
        
};
use crate::google::{
    credentials::read_credentials,
    oauth::{get_access_token, get_auth_url, exchange_code_for_token},
    calendar::get_daily,
};

    
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel::<CustomEvent>();
    thread::spawn(move || {
        if let Some(manager) = BatteryManager::new() {
            let mut last: Option<Battery> = None;
            loop {
                if let Some(battery_update) = manager.get() {
                    if last.as_ref() != Some(&battery_update) {
                        sender_battery_update(battery_update.clone());
                        last = Some(battery_update);
                    }
                }

                thread::sleep(Duration::from_secs(1));
            }
        }

    });
    thread::spawn(move || {
        let mut last: Option<DeviceState> = None;
        loop {
            let state = Device::get_state();
            if last.as_ref() != Some(&state) {
                sender_network_deveice_state(state.clone());
                last = Some(state);
            }

            thread::sleep(Duration::from_secs(1));
        }
    });


    let config = crate::utils::config::load_config();
    let (wm_conn, screen_num) = x11rb::connect(None)?;
    let (ipc_conn, _) = x11rb::connect(None)?;
    let mut wm = WorkspaceManager::new(wm_conn, config.styles.panel_width);
    let _ = wm.load_layouts();
    wm.apply_layout();

    let screen = &wm.conn.setup().roots[screen_num];
    let root = screen.root;


    // ───────── Cursor default ─────────
    let cursor_font = wm.conn.generate_id()?;
    wm.conn.open_font(cursor_font, b"cursor")?;

    let cursor = wm.conn.generate_id()?;
    wm.conn.create_glyph_cursor(
        cursor,
        cursor_font,
        cursor_font,
        68, // left_ptr
        69,
        0, 0, 0,
        65535, 65535, 65535,
    )?;

    wm.conn.change_window_attributes(
        root,
        &ChangeWindowAttributesAux::default().cursor(cursor),
    )?;

    wm.conn.close_font(cursor_font)?;
    wm.conn.flush()?;


    wm.init_dock(config.styles.dock_width, screen.height_in_pixels, config.apps.dock.clone());


    // ─────────────────────
    // Wake-up atom
    // ─────────────────────
    let wake_up_atom = wm
        .conn
        .intern_atom(false, b"PUNKWM_WAKEUP")?
        .reply()?
        .atom;

    // ─────────────────────
    // IPC notifier (usa OTRA conexión)
    // ─────────────────────
    
    let notifier = MainThreadNotifier {
        tx: tx.clone(),
        conn: Arc::new(ipc_conn),
        root,
        wake_up_atom,
    };

    let notifier_clone = notifier.clone();
    thread::spawn(move || {
        start_ipc_server(notifier_clone).unwrap();
    });


    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let _ = listen_notifications().await;
        });
    });

    // ─────────────────────
    // Registrar WM global
    // ─────────────────────
    let wm_protocols = wm
        .conn
        .intern_atom(false, b"WM_PROTOCOLS")?
        .reply()?
        .atom;

    let wm_delete = wm
        .conn
        .intern_atom(false, b"WM_DELETE_WINDOW")?
        .reply()?
        .atom;

    // Fondo
    std::process::Command::new("feh")
        .args(&["--bg-fill", &config.styles.bg])
        .spawn()
        .unwrap();

    // Root events
    wm.conn.change_window_attributes(
        root,
        &ChangeWindowAttributesAux::new().event_mask(
            EventMask::SUBSTRUCTURE_REDIRECT
                | EventMask::SUBSTRUCTURE_NOTIFY
                | EventMask::KEY_PRESS,
        ),
    )?;

    // Keybindings
    let bindings = crate::utils::config::parse_bindings(&config);

    for b in &bindings {
        wm.conn.grab_key(
            false,
            root,
            b.modifier,
            b.keycode,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
        )?;
    }

    for key in 10u8..=18u8 {
        wm.conn.grab_key(false, root, ModMask::M4, key, GrabMode::ASYNC, GrabMode::ASYNC)?;
        wm.conn.grab_key(
            false,
            root,
            ModMask::M4 | ModMask::SHIFT,
            key,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
        )?;
    }

    wm.conn.grab_key(
        false,
        root,
        ModMask::M1,
        46u8,
        GrabMode::ASYNC,
        GrabMode::ASYNC,
    )?;

    wm.conn.flush()?;

    // ─────────────────────
    // Apps Load
    // ─────────────────────

    let mut desktops = Vec::new();
    let paths_folder_desktops = vec![
        "/usr/share/applications".to_string(),
        "/usr/local/share/applications".to_string(),
        "~/.local/share/applications".to_string(),
    ];

    let paths_desktops = get_all_desktops_paths(paths_folder_desktops);
    for path in paths_desktops{
        if let Some(d) = Desktop::parse(&path) {
            desktops.push(DockDesktop::from_desktop(d, &config.styles.lang.clone()));
        }
    }




    println!("punkwm iniciado");

    // ─────────────────────
    // Main event loop
    // ─────────────────────
    loop {
        let event = wm.conn.wait_for_event()?;

        match event {
            Event::ClientMessage(e) if e.type_ == wake_up_atom => {
                while let Ok(custom) = rx.try_recv() {
                    match custom {
                        CustomEvent::SwitchTo(i) => {wm.switch_to(i)},
                        CustomEvent::ToggleLayout() => {wm.toggle_layout_persist()},
                        CustomEvent::StartDock() => {
                            sender_layout_set(&mut wm);
                            sender_workspace_update(&mut wm);
                            if let Some(manager) = BatteryManager::new() {
                                manager.update();
                            }                
                            sender_network_deveice_state(Device::get_state());
                            sender_system_load_panel(config.styles.dock_width.into(), config.styles.panel_width.into());
                            
                        },
                        CustomEvent::OpenPanel() =>{
                            if !wm.panel.is_open(){
                                wm.panel.open();
                                if let Some(ref mut dock) = wm.dock {
                                    dock.panel_open(&wm.conn);
                                }                                    
                                wm.apply_layout();
                            }
                        },
                        CustomEvent::OpenHomePanel()=>{
                            let notifier_clone = notifier.clone();
                            sender_panel_home_system_stats(system_usage());
                            notifier_clone.send(CustomEvent::HomePanelLoadDaily());
                            notifier_clone.send(CustomEvent::HomePanelLoadWeather());
                            
                        },

                        CustomEvent::HomePanelLoadDaily()=>{
                            if let Some(credenciales) = read_credentials(config.google.credentials_file.clone()) {
                                let scopes = config.google.scopes.clone();
                                std::thread::spawn(move || {
                                let rt = tokio::runtime::Runtime::new().unwrap();

                                rt.block_on(async move {
                                    if let Some(access_token) = get_access_token(&credenciales).await {
                                        let daily = get_daily(&access_token, config.google.max_events_view).await;
                                        sender_panel_home_google_calender_daily(daily);
                                        
                                    } else {
                                        sender_panel_home_google_oauth_url(get_auth_url(&credenciales, scopes));
                                    }
                                    });
                                });
                                                               
                            }

                        },
                        CustomEvent::HomePanelLoadWeather()=>{
                            let weather_cfg = config.weather.clone();
                            let lang_cfg = config.styles.lang.clone();
                            std::thread::spawn(move || {
                                let rt = tokio::runtime::Runtime::new().unwrap();
                                rt.block_on(async move {
                                    if let Some(weather) = get_weather(
                                        weather_cfg.city, 
                                        weather_cfg.state, 
                                        weather_cfg.country, 
                                        lang_cfg, 
                                        weather_cfg.units).await{
                                            sender_panel_home_weather_load(weather);

                                        }
                                       
                                });
                            });
                        },
                        CustomEvent::GoogleOauthLogin(code)=>{
                            if let Some(credenciales) = read_credentials(config.google.credentials_file.clone()) {
                                let notifier_clone = notifier.clone();
                                std::thread::spawn(move || {
                                    let rt = tokio::runtime::Runtime::new().unwrap();
                                    rt.block_on(async move {
                                        let _ = exchange_code_for_token(code.clone(), &credenciales).await;
                                        notifier_clone.send(CustomEvent::HomePanelLoadDaily());
                                   });

                                });
                            }
                        },


                        CustomEvent::OpenAppsPanel()=>{
                            let notifier_clone = notifier.clone();
                            notifier_clone.send(CustomEvent::AppsPanelLoadApps());
                        },

                        CustomEvent::AppsPanelLoadApps()=>{
                            sender_panel_apps_load_apps(desktops.clone());
                        },

                        CustomEvent::AppsPanelSearch(q)=>{
                            let searchs = search_docks(&desktops.clone(), &q);
                            sender_panel_apps_load_apps(searchs.clone());
                        },

                        CustomEvent::AppsPanelOpenApp(package)=>{
                            let notifier_clone = notifier.clone();
                            if let Some(dock) = find_by_package(&desktops.clone(), &package) {
                                spawn(&clean_exec(&dock.exec));
                                notifier_clone.send(CustomEvent::ClosePanel());
                                notifier_clone.send(CustomEvent::AppsPanelLoadApps());
                            }
                        },
                        
                        CustomEvent::OpenNetworkPanel()=>{
                            let notifier_clone = notifier.clone();
                            notifier_clone.send(CustomEvent::NetworkPanelLoadWiFi());
                        },

                        CustomEvent::NetworkPanelLoadWiFi()=>{
                            let wifis = get_wifi_networks();
                            sender_panel_network_load_wifi(wifis);
                        },
                        
                        CustomEvent::NetworkPanelConnectWiFi(ssid, password)=>{
                            if NetworkManager::connect(&ssid, &password){
                                let notifier_clone = notifier.clone();
                                notifier_clone.send(CustomEvent::NetworkPanelLoadWiFi());
                            }
                        },
        
                        CustomEvent::NetworkPanelConnectWiFiPublic(ssid)=>{
                            if NetworkManager::connect_public(&ssid){
                                let notifier_clone = notifier.clone();
                                notifier_clone.send(CustomEvent::NetworkPanelLoadWiFi());
                            }
                        },
        
                        CustomEvent::NetworkPanelShareWiFi()=>{
                            if let Some(qr_base64) = sNetworkManager::hare_wifi() {
                                sender_panel_network_share_wifi(qr_base64);
                            }
                        },
        
                        CustomEvent::NetworkPanelDisconnectWiFi()=>{
                            if NetworkManager::disconnect(&ssid){
                                let notifier_clone = notifier.clone();
                                notifier_clone.send(CustomEvent::NetworkPanelLoadWiFi());
                            }
                        },


                        CustomEvent::ClosePanel() =>{
                            if wm.panel.is_open(){
                                    wm.panel.close();
                                if let Some(ref mut dock) = wm.dock {
                                    dock.panel_close(&wm.conn);
                                }        
                                sender_system_panel_close();
                                wm.apply_layout();
                            }
                        },
                        
                    }
                }
            }

            Event::MapRequest(e) => {
                wm.manage_window(e.window);
            }

            Event::DestroyNotify(e) => {
                wm.unmanage_window(e.window);
            }

            Event::ConfigureRequest(e) => {
                let aux = ConfigureWindowAux::from_configure_request(&e);
                wm.conn.configure_window(e.window, &aux)?;
                wm.conn.flush()?;
            }

            
            Event::KeyPress(e) => {
                let state = e.state;

                if state.contains(KeyButMask::MOD4 | KeyButMask::SHIFT)
                    && (10..=18).contains(&e.detail)
                {
                    wm.move_focus_window((e.detail - 10) as usize);
                    continue;
                }

                if state.contains(KeyButMask::MOD4) && (10..=18).contains(&e.detail) {
                    wm.switch_to((e.detail - 10) as usize);
                    continue;
                }

                if state.contains(KeyButMask::MOD1) && e.detail == 46 {
                    wm.toggle_layout_persist();
                    continue;
                }

                for b in &bindings {
                    if e.detail == b.keycode && state.contains(b.modifier) {
                        match b.action.as_str() {
                            "focus_next" => wm.focus_next(),
                            "focus_prev" => wm.focus_prev(),
                            "close" => wm.close_focused(wm_protocols, wm_delete),
                            "terminal" => spawn(&config.apps.terminal),
                            "browser" => spawn(&config.apps.browser),
                            "filemanager" => spawn(&config.apps.filemanager),
                            "editor" => spawn(&config.apps.editor),
                            _ => {}
                        }
                    }
                }
            }

            _ => {}
        }
    }
}

fn clean_exec(exec: &str) -> String {
    let tokens = ["%u", "%U", "%f", "%F", "%i", "%c", "%k"];

    let mut s = exec.to_string();
    for t in tokens {
        s = s.replace(t, "");
    }

    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
