const handlers = {
  "Language": {
    "Load": ({ Lang }) => Language.Load(Lang),
  },
  "DockPannel": {
    "Load": ({ Dock_Width, Pannel_Width }) => DockPannel.Load(Dock_Width, Pannel_Width),
  },
  "Layout": {
    "Load": ({ Keymap }) => Layout.Load(Keymap),
  },
  "Google": {
    "Load": ({ Events, Scopes, Credentials }) => Google.Load(Events, Scopes, Credentials),
  },
  "Music Folders": {
    "Load": ({ Folders }) => MusicFolder.Load(Folders),
  },
  "Weather": {
    "Load": ({ Units, City, State, Country }) => Weather.Load(Units, City, State, Country),
  },
  "Wallpapers": {
    "Load": ({ Main, Lock }) => Wallpapers.Load(Main, Lock),
  },
  "Apps": {
    "Load": ({ Apps }) => APPs.Load(Apps),
  },
  "Keybindings":{
    "Load": ({ Shortcuts }) => Keybindings.Load(Shortcuts),
  }
};

const { event: TAURI_EVENT } = window.__TAURI__;

TAURI_EVENT.listen("IPC_Settings", (event) => {
  console.log("📥 IPC:", event.payload);
  const { category, name, data } = event.payload;

  handlers?.[category]?.[name]?.(data);

});

async function IPC_Front_emit(category, name, data = {}) {
  await TAURI_EVENT.emit("IPC-Settings", {
    category,
    name,
    data,
  });
}



class Sender {
  static async Emit(category, name, data = {}) {
    await TAURI_EVENT.emit("IPC-Settings", {
      category,
      name,
      data,    
    });
  }

  static async EmitBridge(category, name, data = {}) {
    await TAURI_EVENT.emit("IPC-Settings", {
      category:category,
      name:name,
      data:data,
      Bridge:true,
    });
  }

}
