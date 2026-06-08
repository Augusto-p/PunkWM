class PanelApps extends Sender {
    static async Open(){super.EmitBridge("Panel:Apps", "Open");}
    static async LoadApps(){super.EmitBridge("Panel:Apps", "Load Apps");}
    static async OpenApp(pack){super.EmitBridge("Panel:Apps", "Open App", {"package": pack});}
    static async SearchApps(query){super.EmitBridge("Panel:Apps", "Search", {"q": query });}
}