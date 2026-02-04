async function emit_Open_Home_Panel() {
    await IPC_Front_emit("Panel:Home", "Open");
}
async function emit_Home_Panel_Refresh_Diary() {
    await IPC_Front_emit("Panel:Home", "Google:Diary:Refresh");
}

async function emit_Home_Panel_Google_Login(url) {
    await IPC_Front_emit("Panel:Home", "Google:Oauth:Login", {
        "URL": url
    });
}