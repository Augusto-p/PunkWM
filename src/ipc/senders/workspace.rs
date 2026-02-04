use crate::ipc::message::IpcMessage;
use crate::WorkspaceManager;
use crate::ipc::socket::socket_send_dock;
use serde_json::json;
pub fn sender_workspace_update(wm: &mut WorkspaceManager){
    let mut lista: Vec<i32> = Vec::new();
    for workspace in &wm.workspaces {
        lista.push(workspace.len() as i32);
    }
    lista[wm.current] = -1;
    let msg = IpcMessage::new("Workspace","Update", json!({"data": lista}));
    let _ = socket_send_dock(&msg);
}