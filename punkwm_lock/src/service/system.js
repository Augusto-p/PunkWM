class System extends Sender {
    static async Poweroff(){super.Emit("System", "Poweroff");}
    static async Reboot(){super.Emit("System", "Reboot");}
    static async Start(){super.Emit("System", "Start");}
}
System.Start();
window.focus()