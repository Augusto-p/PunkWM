const Apps_grid = document.getElementById("Apps_grid");
const panel_apps_searchbar = document.getElementById("panel_apps_searchbar")
function ToogleApps(){
    let Panel_Mode = body.getAttribute("data-panel");
    if (Panel_Mode == null){
        Panel.Open();
        openApps();
    }else if (Panel_Mode != "Open-Apps") {
        openApps();
    }else{
        panel_close();
    }
    panel_apps_searchbar.value = "";

}

function openApps(){
    body.dataset.panel = "Open-Apps";
    PanelApps.Open();
    
}

function NewApp(Name, Image, Package) {
    let div = document.createElement("div");
    div.classList.add("app");

    div.addEventListener("click", () => {
        App_Open(Package)
    });

    let img = document.createElement("img");
    img.src = `assets/AppsIcons/${Image.toLowerCase()}.svg`;

    // fallback si no existe la imagen
    img.onerror = () => {
        img.onerror = null; // evita loop si también falla la default
        img.src = "assets/AppsIcons/default.svg";
    };

    let span = document.createElement("span");
    span.textContent = Name;

    div.appendChild(img);
    div.appendChild(span);

    Apps_grid.appendChild(div);
}


function LoadApps(Apps) {
    Apps_grid.innerHTML = "";
    Apps.forEach(app => {
        NewApp(app.name, app.icon, app.package);
    });    
}
let panel_apps_searchbar_timer = null;
panel_apps_searchbar.addEventListener("keyup", (e)=>{
    if (e.key === "Enter") {
        if (panel_apps_searchbar.value == "") {
            PanelApps.LoadApps();
        }else{
            PanelApps.SearchApps(panel_apps_searchbar.value);
            
        }
    }
    clearTimeout(panel_apps_searchbar_timer);
    panel_apps_searchbar_timer = setTimeout(() => {
        if (panel_apps_searchbar.value == "") {
            PanelApps.LoadApps();
        }else{
            PanelApps.SearchApps(panel_apps_searchbar.value);
        }
    }, 1000);
})


function App_Open(Package){
    panel_apps_searchbar.value = "";
    PanelApps.OpenApp(Package);
    

}
