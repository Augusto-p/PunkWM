class DockPannel extends Sender {
    static async SaveWidthPannel(Width){super.Emit("DockPannel", "Save Width Pannel", {"Width":Width});}
    static async SaveWidthDock(Width){super.Emit("DockPannel", "Save Width Dock", {"Width": Width});}
    static Load(Dock, Pannel){
        setDockWidth(Dock);
        setPannelWidth(Pannel);
        DockWidth.value = getDockWidth();
        PannelWidth.value = getPannelWidth();
    }
    
}