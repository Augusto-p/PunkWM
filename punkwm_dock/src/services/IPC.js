const handlers = {
  Workspace: {
    Update: ({ data }) => updateWorkspaces(data),
  },

  Layout: {
    Set: ({ Layout }) => setLayout(Layout),
  },

  Battery: {
    Update: ({ charging, percentage }) => updateBattery(charging, percentage),
  },

  Network: {
    "Device:State": ({ state, level }) => update_network(state, level ?? -1),
  },
  System: {
    "Panel:Load": ({ dock_width, panel_width }) => panel_load(dock_width, panel_width),
    
  },
  "Panel:Home":{
    "Google:Daily": ({ events }) => Load_Diary(events),
    "System:Stats": ({ cpu, ram, disk, gpu }) => Load_stats(cpu, ram, disk, gpu),
    "Weather:Load": ({ temp, phrase, icon, wind_direction, wind_speed }) => Load_weather(temp, phrase, icon, wind_direction, wind_speed),
    "Google:Oauth:url": ({ Url }) => set_diary_login_mode(Url),
  },
  "Panel:Notify":{
    "New": ({ app, icon, title, message, now }) => appendNotify(`${app}_${now}`, app, title, message.split("\n").reverse()[0] , now, icon, ),
  }
};

const { event: TAURI_EVENT } = window.__TAURI__;

TAURI_EVENT.listen("ipc", (event) => {
  console.log("📥 IPC:", event.payload);
  const { category, name, data } = event.payload;

  handlers?.[category]?.[name]?.(data);

});

async function IPC_Front_emit(category, name, data = {}) {
  await TAURI_EVENT.emit("IPC-Front", {
    category,
    name,
    data,
  });
}
