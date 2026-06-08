use crate::AnvilState;
use crate::config::config::GLOBAL_CFG;
use crate::state::Backend;
use serde_json::json;
use crate::ipc::message::{IpcMessage,IpcMode};
use crate::utils::{
    system_usage::SystemUsage,
};


impl<BackendData: Backend + 'static> AnvilState<BackendData> {

    pub fn battery_update_sender(&self){
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"Battery","Update", serde_json::to_value(self.battery.clone()).unwrap());
        self.punk_ipc.send(msg);
    }

    pub fn network_status_update_sender(&self){
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"Network","Device:State", serde_json::to_value(&self.network_status).unwrap());
        self.punk_ipc.send(msg);
    }

    pub fn layout_set_sender(&self){
        let layout = self.current_workspace().layout;
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"Layout","Set", json!({"layout": layout.id()}));
        self.punk_ipc.send(msg);
    }

    pub fn pannel_load_sender(&self){
        let cfg = GLOBAL_CFG.read().expect("No se pudo bloquear para lectura");
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"System","Panel:Load",json!({"dock_width": cfg.styles().get_dock_width(), "panel_width": cfg.styles().get_pannel_width()}));
        self.punk_ipc.send(msg);
    }

    pub fn workspace_update_sender(&self){
        let mut workspaces: Vec<i32> = Vec::new();
        for workspace in &self.workspaces {
            workspaces.push(workspace.windows.len() as i32);
        }
        workspaces[self.current_workspace().id as usize] = -1;
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"Workspace","Update", json!({"data": workspaces}));
        self.punk_ipc.send(msg);
    }

    pub fn stats_home_pannel_sender(&self, stats:SystemUsage){
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"Panel:Home","System:Stats", serde_json::to_value(&stats).unwrap());
        self.punk_ipc.send(msg);
    }

    pub fn volume_system_sender(&self,volume:u8){
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"System", "Set Volume", json!({"Volume": volume}));
        self.punk_ipc.send(msg);
    }

    pub fn glow_system_sender(&self,glow:u8){
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"System", "Set Glow", json!({"Glow": glow}));
        self.punk_ipc.send(msg);
    }

 
    pub fn apps_load_sender(&self){
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"Panel:Apps","Load:Apps", json!({"Apps": self.apps_manager.apps()}));
        self.punk_ipc.send(msg);
    }

    pub fn apps_search_sender(&self, q :&str){
        let msg = IpcMessage::new(Some(IpcMode::Bridge),"Panel:Apps","Load:Apps", json!({"Apps": self.apps_manager.search(q)}));
        self.punk_ipc.send(msg);
    }
}