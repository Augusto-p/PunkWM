const TAURI = window.__TAURI__.core;
class System extends Sender {
    static async Poweroff(){super.EmitBridge("System", "Poweroff");}
    static async Reboot(){super.EmitBridge("System", "Reboot");}
    static async Logout(){super.Emit("System", "Lock");}
    static async StartDock(){super.EmitBridge("System", "Start Dock");}
    static async setVolume(volume){super.EmitBridge("System", "Set Volume", {"Volume": volume});}
    static async setGlow(Glow){super.EmitBridge("System", "Set Glow", {"Glow": Glow});}
    static async Auth(password){super.EmitBridge("System", "Auth", {"password":password})};
    static async Lock(){super.Emit("System", "Log Out");}
}

