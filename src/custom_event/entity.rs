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
    
}