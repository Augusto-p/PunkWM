class PanelHomeDaily extends Sender {
    static async Refresh(){super.EmitBridge("Panel:Home", "Google:Diary:Refresh");}
    static async GoogleLogin(url){super.Emit("Panel:Home", "Google:Oauth:Login", {"URL": url});}
}
