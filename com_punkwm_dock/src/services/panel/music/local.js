class PanelMusicLocal extends Sender {
    static async LoadSongs(){super.EmitBridge("Panel:Music", "Local:Load:Song");}
    static async StartSong(path){super.EmitBridge("Panel:Music", "Local:Start:Song", {"path": path});}
    static async PlaySong(){super.EmitBridge("Panel:Music", "Local:Play:Song");}
    static async PauseSong(){super.EmitBridge("Panel:Music", "Local:Pause:Song");}
    static async ResetSong(){super.EmitBridge("Panel:Music", "Local:Reset:Song");}
    static async StopSong(){super.EmitBridge("Panel:Music", "Local:Stop:Song");}
    static async SearchSong(query){super.EmitBridge("Panel:Music", "Local:Search:Song", {"q": query});}
}