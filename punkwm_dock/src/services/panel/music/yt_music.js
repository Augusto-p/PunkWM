
function get_Cookies_YT() {
    let cookies = window.sessionStorage.getItem("YT:Cookies");
    return JSON.parse(cookies);
}

async function emit_Music_Panel_YTMusic_Mode_Start() {
    await IPC_Front_emit("Panel:Music", "YT:Start");
}

function Save_Cookies_YT(cookies) {
    window.sessionStorage.setItem("YT:Cookies", JSON.stringify(cookies));
    emit_Music_Panel_YTMusic_Get_Quick_Picks();
}

async function emit_Music_Panel_YTMusic_Get_Quick_Picks() {
    await IPC_Front_emit("Panel:Music", "YT:Quick picks", {"cookies": get_Cookies_YT()});
}

async function emit_Music_Panel_YTMusic_Get_Next_Songs(id) {
    await IPC_Front_emit("Panel:Music", "YT:Next Songs", {"cookies": get_Cookies_YT(), "songid": id});
}

async function emit_Music_Panel_YTMusic_Search(q) {
    await IPC_Front_emit("Panel:Music", "YT:Search", {"cookies": get_Cookies_YT(), "q": q});
}

async function emit_Music_Panel_YTMusic_Start_Song(id) {
    await IPC_Front_emit("Panel:Music", "YT:Start Song", {"cookies": get_Cookies_YT(), "songid": id});
}

async function emit_Music_Panel_YTMusic_Play_Song() {
    await IPC_Front_emit("Panel:Music", "YT:Play Song");
}
async function emit_Music_Panel_YTMusic_Pause_Song() {
    await IPC_Front_emit("Panel:Music", "YT:Pause Song");
}

// emit_Music_Panel_YTMusic_Mode_Start();