class Wallpapers extends Sender {
    static async SaveMain(ImgB64){super.Emit("Wallpapers", "Save Main", {"Img": ImgB64});}
    static async SaveLock(ImgB64){super.Emit("Wallpapers", "Save Lock", {"Img": ImgB64});}
    static Load(Main, Lock){
        mainWallpeperView.style.backgroundImage = `url('${Main}')`;
        lockWallpeperView.style.backgroundImage = `url('${Lock}')`;
    }
}