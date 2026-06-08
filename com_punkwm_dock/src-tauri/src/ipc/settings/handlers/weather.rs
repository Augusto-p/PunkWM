use crate::{ipc::message::IpcMessage};
// use serde_json::Value;
use crate::apphandle::get_api_ipc;
// use punkwm_dock_lib::print_in_tty;

pub fn handler_weather(msg: IpcMessage) {
     match msg.name().as_str() {
        "Save Units" =>{
            let units = msg.data()["Mode"].to_string();
            let api = get_api_ipc();
            api.with_config(|cfg| {
                cfg.weather().set_units(units);
            });
        },
        "Save Location" => {
            let api = get_api_ipc();
            let city = msg.data()["City"].to_string();
            let state = msg.data()["State"].to_string();
            let country = msg.data()["Country"].to_string();
            api.with_config(|cfg| {
                cfg.weather().set_city(city);
                cfg.weather().set_state(state);
                cfg.weather().set_country(country);
            });
            
        },

        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.category(),
                msg.name()
            );
        }
     }
    
}