async function emit_panel_apps_search(query) {
    await IPC_Front_emit("Panel:Apps", "Search", {"q": query });
}

async function emit_panel_apps_open_app(package) {
    await IPC_Front_emit("Panel:Apps", "Open App", {"package": package});
}
async function emit_panel_apps_open() {
    await IPC_Front_emit("Panel:Apps", "Open");
}

async function emit_panel_apps_load_apps() {
    await IPC_Front_emit("Panel:Apps", "Load Apps");
}