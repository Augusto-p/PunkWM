const TAURI = window.__TAURI__.core;

async function emit_Poweroff() {
    await IPC_Front_emit("System", "Poweroff");
}

async function emit_Reboot() {
    await IPC_Front_emit("System", "Reboot");
}
async function emit_LogOut() {
    await IPC_Front_emit("System", "Log Out");
}
async function emit_Lock() {
    await IPC_Front_emit("System", "Lock");
}

async function emit_Start_Dock() {
    await IPC_Front_emit("System", "Start Dock");
}



emit_Start_Dock()