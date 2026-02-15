const Apps_grid = document.getElementById("Apps_grid");
const panel_apps_searchbar = document.getElementById("panel_apps_searchbar")
function ToogleApps(){
    let Panel_Mode = body.getAttribute("data-panel");
    if (Panel_Mode == null){
        emit_Open_Panel();
        openApps();
    }else if (Panel_Mode != "Open-Apps") {
        openApps();
    }else{
        panel_close();
    }


}

function openApps(){
    body.dataset.panel = "Open-Apps";
    emit_panel_apps_open();
}

function NewApp(Name, Image, Package) {
    let div = document.createElement("div");
    div.classList.add("app");

    div.addEventListener("click", () => {
        emit_panel_apps_open(Package);
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
        emit_panel_apps_search(panel_apps_searchbar.value);
    }
    clearTimeout(panel_apps_searchbar_timer);
    panel_apps_searchbar_timer = setTimeout(() => {
        emit_panel_apps_search(panel_apps_searchbar.value);
    }, 1000);
})



// LoadApps([{
//     "Name": "Firefox",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/a/a0/Firefox_logo%2C_2019.svg",
//     "Package": "org.mozila.firefox"
// },
// {
//     "Name": "Google Chrome",
//     "Image": "http://upload.wikimedia.org/wikipedia/commons/e/e1/Google_Chrome_icon_%28February_2022%29.svg",
//     "Package": "com.google.chrome"
// },
// {
//     "Name": "GIMP",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/4/45/The_GIMP_icon_-_gnome.svg",
//     "Package": "org.gnome.gimp"
// },
// {
//     "Name": "Python",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/c/c3/Python-logo-notext.svg",
//     "Package": "org.python"
// },
// {
//     "Name": "Visual Studio Code",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/9/9a/Visual_Studio_Code_1.35_icon.svg",
//     "Package": "com.mirosoft.code"
// },
// {
//     "Name": "VLC Media Player",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/e/e6/VLC_Icon.svg",
//     "Package": "org.vlc"
// },
// {
//     "Name": "Brave",
//     "Image": "https://brave.com/static-assets/images/brave-logo-sans-text.svg",
//     "Package": "com.brave.brave"
// },
// {
//     "Name": "Android Studio",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/c/c1/Android_Studio_icon_%282023%29.svg",
//     "Package": "com.android.studio"
// },{
//     "Name": "Nautilus",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/6/65/GNOME_Files_icon_2019.svg",
//     "Package": "org.gnome.files"
// },
// {
//     "Name": "Microsoft Word",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/1/19/Microsoft_Office_Word_%282019%E2%80%932025%29.svg",
//     "Package": "org.libreoffice.writer"
// },
// {
//     "Name": "Microsoft Excel",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/e/e3/Microsoft_Office_Excel_%282019%E2%80%932025%29.svg",
//     "Package": "org.libreoffice.calc"
// },
// {
//     "Name": "Microsoft PowerPoint",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/2/25/Microsoft_Office_PowerPoint_%282019%E2%80%932025%29.svg",
//     "Package": "org.libreoffice.impress"
// },
// {
//     "Name": "Postman",
//     "Image": "https://voyager.postman.com/logo/postman-logo-icon-orange.svg",
//     "Package": "com.postman.postman"
// },{
//     "Name": "TeXstudio",
//     "Image": "https://upload.wikimedia.org/wikipedia/commons/2/2a/TeXstudio_Logo.svg",
//     "Package": "org.texstudio.texstudio"
// }
// ])

// openApps()