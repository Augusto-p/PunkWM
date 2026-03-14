class PanelNetwork extends Sender {
    static async Open(){super.Emit("Panel:Network", "Open");}
    static async Refresh(){super.Emit("Panel:Network", "Refresh");}
    static async ShareWiFi(){super.Emit("Panel:Network", "Share WiFi");}
    static async DisconnectWiFi(){super.Emit("Panel:Network", "Disconnect WiFi");}
    static async ConnectWiFiPublic(SSID){super.Emit("Panel:Network", "Connect PublicWiFi", {"SSID": SSID});}
    static async ConnectWiFi(SSID, Password){super.Emit("Panel:Network", "Connect WiFi", {"SSID": SSID, "Password": Password});}
}