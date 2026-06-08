class Layout extends Sender {
    static async Save(id){super.Emit("Layout", "Save", {"Id": id});}
    static Load(keymap){
        setLayout(keymap, false);
    }
}