const Notify_Content = document.getElementById("Notify_Content");
const Notify_bubble = document.getElementById("Notify_bubble");

const Notify = new Map();
function openNotify(){
    body.dataset.panel = "Open-Notify";
}

function ToggleNotify(){
    let Panel_Mode = body.getAttribute("data-panel");
    if (Panel_Mode == null){
        Panel.Open();
        openNotify();
    }else if (Panel_Mode != "Open-Notify") {
        openNotify();
    }else{
        panel_close();
    }


}

function newNotify(id, app,title, descp, hour, icon, acctions = null) {
    Notify_Content.innerHTML += `<div class="Notify" onclick="">
                <div class="header">
                    <button onclick="closeNotify(this, ${id})">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
                    </button>
                    <span class="App">${app}</span>
                        <span class="time">${hour}</span>
                </div>
                <div class="content">
                    <span class="title">${title}</span>
                    <span class="decp">${descp}</span>

                    <div>
                        <button onclick="">
                            <img src="${icon}">
                        </button>
                    </div>
                </div>
            </div>`;

    
}

function closeNotify(elem, id) {
    Notify_Content.removeChild(elem);
    Notify.delete(id);
}

function clearNotify() {
    Notify_Content.innerHTML = "";
    Notify.clear();
    updateNotifyBubble();
    
}

function appendNotify(id, app,title, descp, hour, icon, acctions = null) {
    let hour_f = formatHour(hour);
    Notify.set(id, {
        "App": app,
        "Title": title,
        "Descp": descp,
        "Hour": hour_f,
        "Icon": icon,
        "Acctions": acctions
    })

    newNotify(id, app, title, descp, hour, icon, acctions);
    updateNotifyBubble();

    
}


function formatHour(unix) {
    const d = new Date(unix * 1000);
    return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }); 
}


function updateNotifyBubble() {
    Notify_bubble.textContent = Notify.size < 100 ? Notify.size: "99+";
    if (Notify.size == 0) {
        Notify_bubble.style.display = "none";
    }else{
        Notify_bubble.style.display = "flex";
    }
    
}

// ToggleNotify()