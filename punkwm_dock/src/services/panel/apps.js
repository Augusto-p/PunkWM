async function emit_panel_apps_search(query) {
    await IPC_Front_emit("Panel:Apps", "Search", {"q": query });
}

async function emit_panel_apps_open(package) {
    await IPC_Front_emit("Panel:Apps", "Open", {"package": package});
}