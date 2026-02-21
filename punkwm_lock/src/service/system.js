function emit_system_poweroff() {
    IPC_emit("System", "Poweroff");
}
function emit_system_reboot() {
    IPC_emit("System", "Reboot");
}

function emit_system_start() {
    IPC_emit("System", "Start");
}

emit_system_start();
window.focus()