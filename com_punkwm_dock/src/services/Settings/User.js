class User extends Sender {
    static async SaveName(name){super.Emit("User", "Save Name", {"Name": name});}
    static async SaveImage(imgB64){super.Emit("User", "Save Image", {"Img": imgB64});}
}