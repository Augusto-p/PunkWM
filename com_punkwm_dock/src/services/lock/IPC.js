const handlers = {
  "System:Lock":{
    "Bg": ({bg}) => setWallpaerLockScreen(bg),
    "User": ({image, name})=> setUserLockScreen(name, image),

  },
};

const { event: TAURI_EVENT } = window.__TAURI__;

TAURI_EVENT.listen("ipc-lock", (event) => {
  console.log("📥 IPC:", event.payload);
  const { category, name, data } = event.payload;
  handlers?.[category]?.[name]?.(data);

});

class Sender {
  static async Emit(category, name, data = {}) {
    await TAURI_EVENT.emit("IPC-Front", {
      category:category,
      name:name,
      data:data,
    });
  }

  static async EmitBridge(category, name, data = {}) {
    await TAURI_EVENT.emit("IPC-Front", {
      category:category,
      name:name,
      data:data,
      Bridge:true,
    });
  }

}

