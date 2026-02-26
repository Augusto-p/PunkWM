class PanelApps extends Sender {
    static async Open(){super.Emit("Panel:Apps", "Open");}
    static async LoadApps(){super.Emit("Panel:Apps", "Load Apps");}
    static async OpenApp(package){super.Emit("Panel:Apps", "Open App", {"package": package});}
    static async SearchApps(query){super.Emit("Panel:Apps", "Search", {"q": query });}
}