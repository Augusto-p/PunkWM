class PanelHome extends Sender {
    static async Open(){super.EmitBridge("Panel:Home", "Open");}
}