class PanelNetwork extends Sender {
    static async Open(){super.EmitBridge("Panel:Network", "Open");}
    static async Refresh(){super.EmitBridge("Panel:Network", "Refresh");}
    static async ShareWiFi(){super.EmitBridge("Panel:Network", "Share WiFi");}
    static async DisconnectWiFi(){super.EmitBridge("Panel:Network", "Disconnect WiFi");}
    static async ConnectWiFiPublic(SSID){super.EmitBridge("Panel:Network", "Connect PublicWiFi", {"SSID": SSID});}
    static async ConnectWiFi(SSID, Password){super.EmitBridge("Panel:Network", "Connect WiFi", {"SSID": SSID, "Password": Password});}
}