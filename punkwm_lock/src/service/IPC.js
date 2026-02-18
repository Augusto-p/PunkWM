const handlers = {
  "System":{
    "Set Background": ({ bg }) => setWallpaer(bg),
  },

};

const { event: TAURI_EVENT, core: TAURI_CORE } = window.__TAURI__;

TAURI_EVENT.listen("ipc", (event) => {
  console.log("📥 IPC:", event.payload);
  const { category, name, data } = event.payload;
  handlers?.[category]?.[name]?.(data);

});

async function IPC_emit(category, name, data = {}) {
  await TAURI_EVENT.emit("IPC", {
    category,
    name,
    data,
  });
}


