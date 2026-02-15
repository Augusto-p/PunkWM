const share_wifi = document.getElementById("share_wifi");
const share_wifi_img = document.getElementById("share_wifi_img");
const Panel_wifi_list = document.getElementById("Panel_wifi_list");
const panel_network_password = document.getElementById("panel_network_password");
function ToogleWiFi(){
    let Panel_Mode = body.getAttribute("data-panel");
    if (Panel_Mode == null){
        emit_Open_Panel();
        openWiFi();
    }else if (Panel_Mode != "Open-WiFi") {
        openWiFi();
    }else{
        panel_close();
    }
}

function openWiFi(){
    body.dataset.panel = "Open-WiFi";
    emit_network_panel_open()
}

function ToogleShareWifi() {
    share_wifi.classList.toggle("view");
}
function NewWifi(Signal, Name, connected = false) {
    let clas = "";
    let fun = "ToggleConnect(${Name})";
    if (connected) {
        clas = "connected";
        fun = "";
    }

    Panel_wifi_list.innerHTML = `<div class="wifi ${clas}">
                ${WiFi_Icons[Signal]}
                <span>${Name}</span>
                <button onclick="${fun}">
                    <svg class="linked" xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="M440-280H280q-83 0-141.5-58.5T80-480q0-83 58.5-141.5T280-680h160v80H280q-50 0-85 35t-35 85q0 50 35 85t85 35h160v80ZM320-440v-80h320v80H320Zm200 160v-80h160q50 0 85-35t35-85q0-50-35-85t-85-35H520v-80h160q83 0 141.5 58.5T880-480q0 83-58.5 141.5T680-280H520Z"/></svg>
                    <svg class="ulinked" xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="m770-302-60-62q40-11 65-42.5t25-73.5q0-50-35-85t-85-35H520v-80h160q83 0 141.5 58.5T880-480q0 57-29.5 105T770-302ZM634-440l-80-80h86v80h-6ZM792-56 56-792l56-56 736 736-56 56ZM440-280H280q-83 0-141.5-58.5T80-480q0-69 42-123t108-71l74 74h-24q-50 0-85 35t-35 85q0 50 35 85t85 35h160v80ZM320-440v-80h65l79 80H320Z"/></svg>
                </button>
            </div>`
    
}

function Load_wifis(Wifis) {
    Panel_wifi_list.innerHTML = ""
    Wifis.forEach(wifi => {
        NewWifi(wifi.Signal, wifi.name, wifi.connected);
    });
    
}

function ToggleConnect(SSID) {
    if (SSID == null) {
        panel_network_password.classList.remove("view");
    }else{
        if (panel_network_password.classList.contains("view")) {
            panel_network_password.classList.remove("view");
        }else{
            panel_network_password.classList.add("view");
            panel_network_password.setAttribute("data-SSID", SSID)

        }
    }
    
}

function TogglePassword() {
    let type = panel_network_password.type;
    if (type == "password") {
        panel_network_password.type = "text";
        panel_network_password.placeholder = "Password :"
    }else{
        panel_network_password.type = "password";
        panel_network_password.placeholder = "********"

    }
}

panel_network_password.addEventListener("keyup", (e)=>{
    if (e.key === "Enter") {
        let SSID = panel_network_password.getAttribute("data-SSID");
        let password = panel_network_password.value;
        emit_network_panel_connect_wifi(SSID, password);
    }

})





// update_network("connected_wifi", 3)
// openWiFi()