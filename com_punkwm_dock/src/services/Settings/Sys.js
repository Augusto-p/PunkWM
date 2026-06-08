class System extends Sender {
    static async Close(){super.Emit("System", "Close");}
    static async Start(){super.Emit("System", "Start");}
}

System.Start()