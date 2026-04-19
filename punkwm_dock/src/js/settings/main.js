const window_bar_name = document.getElementById("window_bar_name")

function SetMain(element) {
    document.body.setAttribute("data-mode", element.dataset.mode);
    let name = window_bar_name.textContent.split(":", 2)
    name[1] = Lang["UI"]["Menu"][element.dataset.mode]
    window_bar_name.textContent = name.join(": ");    
    switch (element.dataset.mode) {
        case "Language":
            LoadLangs()
            break;
    
        default:
            break;
    }
    
}
let Lang = ""
getLangDir(getLang()).then(data =>{
    Lang = data;
    StartSettings();
})

function StartSettings() {
    LoadLagUI();
    LoadLayouts()
    
}
