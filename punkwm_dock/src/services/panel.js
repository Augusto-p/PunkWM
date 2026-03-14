class Panel extends Sender {
    static async Open(){super.Emit("System", "Open Panel");}
    static async Close(){super.Emit("System", "Close Panel");}
}