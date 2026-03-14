// use crate::utils::config::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::ipc::socket::socket_send_dock;
use crate::google::calendar::CalenderEventEntry;
use crate::utils::system::SystemUsage;
use crate::utils::weather::Weather;

use serde_json::json;


pub fn sender_panel_home_google_calender_daily(daily: Vec<CalenderEventEntry>){
    let msg = IpcMessage::new("Panel:Home","Google:Daily", json!({"events": daily}));
    let _ = socket_send_dock(&msg);
}

pub fn sender_panel_home_system_stats(stats: SystemUsage){
    let msg = IpcMessage::new("Panel:Home","System:Stats", serde_json::to_value(&stats).unwrap());
    let _ = socket_send_dock(&msg);
}
pub fn sender_panel_home_weather_load(weather: Weather ){
    let msg = IpcMessage::new("Panel:Home","Weather:Load", serde_json::to_value(&weather).unwrap());
    let _ = socket_send_dock(&msg);
}

pub fn sender_panel_home_google_oauth_url(url: String){
    let msg = IpcMessage::new("Panel:Home","Google:Oauth:url", json!({"Url": url}));
    let _ = socket_send_dock(&msg);
}