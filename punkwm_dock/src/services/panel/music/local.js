class PanelMusicLocal extends Sender {
    static async LoadSongs(){super.Emit("Panel:Music", "Local:Load:Song");}
    static async StartSong(path){super.Emit("Panel:Music", "Local:Start:Song", {"path": path});}
    static async PlaySong(){super.Emit("Panel:Music", "Local:Play:Song");}
    static async PauseSong(){super.Emit("Panel:Music", "Local:Pause:Song");}
    static async ResetSong(){super.Emit("Panel:Music", "Local:Reset:Song");}
    static async StopSong(){super.Emit("Panel:Music", "Local:Stop:Song");}
    static async SearchSong(query){super.Emit("Panel:Music", "Local:Search:Song", {"q": query});}
}