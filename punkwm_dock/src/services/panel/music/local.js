// async function emit_Music_Panel_Local_Load_Songs() {
//     await IPC_Front_emit("Panel:Music", "Local:Load:Song");
// }
// async function emit_Music_Panel_Local_Start_Song(path) {
//     await IPC_Front_emit("Panel:Music", "Local:Start:Song", {"path": path});
// }
// async function emit_Music_Panel_Local_Play_Songs() {
//     await IPC_Front_emit("Panel:Music", "Local:Play:Song");
// }
// async function emit_Music_Panel_Local_Pause_Songs() {
//     await IPC_Front_emit("Panel:Music", "Local:Pause:Song");
// }
// async function emit_Music_Panel_Local_Reset_Songs() {
//     await IPC_Front_emit("Panel:Music", "Local:Reset:Song");
// }
// async function emit_Music_Panel_Local_Stop_Songs() {
//     await IPC_Front_emit("Panel:Music", "Local:Stop:Song");
// }


class PanelMusicLocal extends Sender {
    static async LoadSongs(){super.Emit("Panel:Music", "Local:Load:Song");}
    static async StartSong(path){super.Emit("Panel:Music", "Local:Start:Song", {"path": path});}
    static async PlaySong(){super.Emit("Panel:Music", "Local:Play:Song");}
    static async PauseSong(){super.Emit("Panel:Music", "Local:Pause:Song");}
    static async ResetSong(){super.Emit("Panel:Music", "Local:Reset:Song");}
    static async StopSong(){super.Emit("Panel:Music", "Local:Stop:Song");}
}