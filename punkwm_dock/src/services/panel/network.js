async function emit_network_panel_open() {
    await IPC_Front_emit("Panel:Network", "Open");
}

async function emit_network_panel_refresh() {
    await IPC_Front_emit("Panel:Network", "Refresh");
}


async function emit_network_panel_connect_wifi(SSID, password) {
    await IPC_Front_emit("Panel:Network", "Connect", {"SSID":SSID, "Password": password });
}