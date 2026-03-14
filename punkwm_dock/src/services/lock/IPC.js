const handlers = {
  System: {

    "Set Background LockScreen": ({ bg }) => setWallpaerLockScreen(bg),
    "Set User LockScreen":({image, name}) => setUserLockScreen(name, image)
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
      category,
      name,
      data,
    });
  }

}

