class Keybindings extends Sender {
    static async Save(Command, Key) { super.Emit("Keybindings", "Save", { "Command": Command, "Keys": Key }); }
    // static async popApp(key) { super.Emit("APPs", "Pop App", { "Key": key }); }
    static Load(Shortcuts) {
        Storage.setItem("Keybindings", JSON.stringify(Shortcuts));
        LoadKeybindings()
    }

}