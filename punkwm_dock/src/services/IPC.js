const handlers = {
  Workspace: {
    Update: ({ data }) => updateWorkspaces(data),
  },

  Layout: {
        Set: ({ layout }) => setLayout(layout),
  },

  Battery: {
    Update: ({ charging, percentage }) => updateBattery(charging, percentage),
  },

  Network: {
    "Device:State": ({ state, level }) => update_network(state, level ?? -1),
  },
  System: {
    "Panel:Load": ({ dock_width, panel_width }) => panel_load(dock_width, panel_width),
    "Panel:Close": () => body.removeAttribute("data-panel"),
    "Set:Volume": ({ Volume })=>{SysVolume.setAttribute("data-Volume", Volume);SysVolume.style.setProperty("--value", Volume);},
    "Set:Glow": ({ Glow })=>{SysGlow.setAttribute("data-Glow", Glow);SysGlow.style.setProperty("--value", Glow);},
    "Set Background LockScreen": ({ bg }) => setWallpaerLockScreen(bg),
    "Set User LockScreen":({image, name}) => setUserLockScreen(name, image)
  },
  "Panel:Home": {
    "Google:Daily": ({ events }) => Load_Diary(events),
    "System:Stats": ({ cpu, ram, disk, gpu }) => Load_stats(cpu, ram, disk, gpu),
    "Weather:Load": ({ temp, phrase, icon, wind_direction, wind_speed }) => Load_weather(temp, phrase, icon, wind_direction, wind_speed),
    "Google:Oauth:url": ({ Url }) => set_diary_login_mode(Url),
  },
  "Panel:Notify": {
    "New": ({ app, icon, title, message, now }) => appendNotify(`${app}_${now}`, app, title, message.split("\n").reverse()[0], now, icon,),
  },
  "Panel:Apps": {
    "Load:Apps": ({ Apps }) => LoadApps(Apps),
  },
  "Panel:Network": {
    "Load:WiFi": ({ WiFis }) => Load_wifis(WiFis),
    "Share:WiFi": ({ QR }) => openShareWiFi(QR),
  },
  "Panel:Music": {
    "YT:Set Cookies": (cookies) => PanelMusicYT.saveCookies(cookies),
    "YT:Load Quik Picks": ({ songs }) => load_Songs(songs),
    "YT:Load Next Songs": ({ songs }) => load_Songs(songs),
    "YT:Load Search": ({ songs }) => load_Songs(songs),
    "YT:ViewStatus": ({ status }) => console.log(status),
    "YT:Load Current Time": ({ time }) => SongWidgetUpdateCurrentTime(time),
    "Local:Load Songs": ({ songs }) => load_Songs(songs),
    "Local:Current Time Song": ({ current_time }) => SongWidgetUpdateCurrentTime(current_time),
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



class Sender {
  static async Emit(category, name, data = {}) {
    await TAURI_EVENT.emit("IPC-Front", {
      category,
      name,
      data,
    });
  }

}
