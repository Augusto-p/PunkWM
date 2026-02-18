function emit_system_poweroff() {
    IPC_emit("System", "Poweroff");
}
function emit_system_reboot() {
    IPC_emit("System", "Reboot");
}