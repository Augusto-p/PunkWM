class Panel extends Sender {
    static async Open(){super.EmitBridge("System", "Open Panel");}
    static async Close(){super.EmitBridge("System", "Close Panel");}
}