class MusicFolder extends Sender {
    static async addFolder(value){super.Emit("Music Folders", "Add Folder", {"Folder":value});}
    static async popFolder(value){super.Emit("Music Folders", "Pop Folder", {"Folder":value});}
    static Load(Folders){
        Storage.setItem("Music-Folders", JSON.stringify(Folders));
        loadMusicFolders();
    }
}