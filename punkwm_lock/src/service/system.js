class System extends Sender {
    static async Poweroff(){super.Emit("System", "Poweroff");}
    static async Reboot(){super.Emit("System", "Reboot");}
    static async Start(){super.Emit("System", "Start");}
    static async LoadUsers(){super.Emit("System", "Load Users");}
}
System.Start();
System.LoadUsers();
window.focus()