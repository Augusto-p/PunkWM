pub enum CustomEvent {
    SwitchTo(usize),
    ToggleLayout(),
    StartDock(),
    OpenPanel(),
    OpenHomePanel(),
    ClosePanel(),
    HomePanelLoadDaily(),
    HomePanelLoadWeather(),
    GoogleOauthLogin(String),
    OpenAppsPanel(),
    AppsPanelLoadApps(),
    AppsPanelSearch(String),
    AppsPanelOpenApp(String),
    OpenNetworkPanel(),
    NetworkPanelLoadWiFi(),
    
    NetworkPanelConnectWiFi(String, String),
    NetworkPanelConnectWiFiPublic(String),
    NetworkPanelDisconnectWiFi(),
    NetworkPanelShareWiFi(),
    SongsLocalLoad(),
}

