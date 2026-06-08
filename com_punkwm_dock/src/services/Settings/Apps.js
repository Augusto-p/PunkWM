class APPs extends Sender {
    static async addApp(key, value) { super.Emit("APPs", "Add App", { "Name": key, "Command": value }); }
    static async popApp(key) { super.Emit("APPs", "Pop App", { "Key": key }); }
    static Load(apps) {
        Storage.setItem("Apps", JSON.stringify(apps));
        LoadApps();
    }

}