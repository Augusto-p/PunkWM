const TAURI = window.__TAURI__.core;
class System extends Sender {
    static async Poweroff(){super.Emit("System", "Poweroff");}
    static async Reboot(){super.Emit("System", "Reboot");}
    static async Lock(){super.Emit("System", "Log Out");}
    static async Logout(){super.Emit("System", "Lock");}
    static async StartDock(){super.Emit("System", "Start Dock");}
    static async setVolume(volume){super.Emit("System", "Set Volume", {"Volume": volume});}
    static async setGlow(Glow){super.Emit("System", "Set Glow", {"Glow": Glow});}
}

System.StartDock();