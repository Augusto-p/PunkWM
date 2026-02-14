async function emit_Open_Panel() {
    await IPC_Front_emit("System", "Open Panel");
}

async function emit_Close_Panel() {
    await IPC_Front_emit("System", "Close Panel");
}