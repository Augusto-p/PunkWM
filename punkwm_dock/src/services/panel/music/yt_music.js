class PanelMusicYT extends Sender {
    static async getCookies(){return JSON.parse(window.sessionStorage.getItem("YT:Cookies"));}
    static async start(){super.Emit("Panel:Music", "YT:Start");}
    static async quickPicks(){super.Emit("Panel:Music", "YT:Quick picks", {"cookies": PanelMusicYT.getCookies()});}
    static async nextSongs(id){super.Emit("Panel:Music", "YT:Next Songs", {"cookies": PanelMusicYT.getCookies(), "songid": id});}
    static async search(query){super.Emit("Panel:Music", "YT:Search", {"cookies": PanelMusicYT.getCookies(), "q": query});}
    static async startSong(id){super.Emit("Panel:Music", "YT:Start Song", {"cookies": PanelMusicYT.getCookies(), "songid": id});}
    static async pause(){super.Emit("Panel:Music", "YT:Pause Song");}
    static async play(){super.Emit("Panel:Music", "YT:Play Song");}
    static async saveCookies(cookies){
        window.sessionStorage.setItem("YT:Cookies", JSON.stringify(cookies));
        PanelMusicYT.QuickPicks();}
}
