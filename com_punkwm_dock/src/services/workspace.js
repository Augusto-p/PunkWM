class Workspace extends Sender {
    static async Set(e){
        let space = parseInt(e.dataset.workspace, 10);
        super.EmitBridge("Workspace", "Set", {"space": space});}
}