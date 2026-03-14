class PanelHomeDaily extends Sender {
    static async Refresh(){super.Emit("Panel:Home", "Google:Diary:Refresh");}
    static async GoogleLogin(url){super.Emit("Panel:Home", "Google:Oauth:Login", {"URL": url});}
}
