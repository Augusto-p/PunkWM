async function emit_layout_toggle() {
    await IPC_Front_emit("Layout", "Toggle");
}