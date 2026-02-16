async function emit_network_panel_open() {
    await IPC_Front_emit("Panel:Network", "Open");
}

async function emit_network_panel_refresh() {
    await IPC_Front_emit("Panel:Network", "Refresh");
}

async function emit_network_panel_connect_wifi(SSID, password) {
    await IPC_Front_emit("Panel:Network", "Connect WiFi", {"SSID":SSID, "Password": password });
}

async function emit_network_panel_connect_wifi_Public(SSID) {   
    await IPC_Front_emit("Panel:Network", "Connect PublicWiFi", {"SSID": SSID});
}
async function emit_network_panel_disconnect_wifi() {
    await IPC_Front_emit("Panel:Network", "Disconnect WiFi");
}

async function emit_network_panel_share_wifi() {
    await IPC_Front_emit("Panel:Network", "Share WiFi");
}