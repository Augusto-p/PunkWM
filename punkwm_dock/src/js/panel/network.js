const share_wifi = document.getElementById("share_wifi");
const share_wifi_img = document.getElementById("share_wifi_img");
const Panel_wifi_list = document.getElementById("Panel_wifi_list");
const Panel_wifi_connected_wifi = document.getElementById("connected_wifi");
const panel_network_password = document.getElementById("panel_network_password");
const panel_network_connected_wifi_SSID_view = document.getElementById("connected_wifi_SSID_view")

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
    if (!share_wifi.classList.contains("view")) {
        emit_network_panel_share_wifi();
    }else{
        share_wifi.classList.remove("view");

    }
}
function openShareWiFi(QR) {
    share_wifi_img.src = QR;
    share_wifi.classList.add("view");
    share_wifi_img.onerror = () => {
        share_wifi_img.onerror = null; // evita loop si también falla la default
        share_wifi.classList.remove("view");
    };
}

function NewWifi(ssid, level, security, connected) {
    let div = document.createElement("div");
    div.classList.add("wifi");
    if (connected) {
        div.classList.add("In-Use");
    }

    div.innerHTML = WiFi_Icons[level];
    let span = document.createElement("span");
    span.textContent = ssid;
    let button = document.createElement("button");
    button.innerHTML = `<svg class="linked" xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="M440-280H280q-83 0-141.5-58.5T80-480q0-83 58.5-141.5T280-680h160v80H280q-50 0-85 35t-35 85q0 50 35 85t85 35h160v80ZM320-440v-80h320v80H320Zm200 160v-80h160q50 0 85-35t35-85q0-50-35-85t-85-35H520v-80h160q83 0 141.5 58.5T880-480q0 83-58.5 141.5T680-280H520Z"/></svg>
                    <svg class="ulinked" xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="m770-302-60-62q40-11 65-42.5t25-73.5q0-50-35-85t-85-35H520v-80h160q83 0 141.5 58.5T880-480q0 57-29.5 105T770-302ZM634-440l-80-80h86v80h-6ZM792-56 56-792l56-56 736 736-56 56ZM440-280H280q-83 0-141.5-58.5T80-480q0-69 42-123t108-71l74 74h-24q-50 0-85 35t-35 85q0 50 35 85t85 35h160v80ZM320-440v-80h65l79 80H320Z"/></svg>`;
    button.addEventListener("click", ()=>{
      if(connected){
        emit_network_panel_disconnect_wifi();
      }else{
        if (security) {
            OpenConnect(ssid);
        }else{
            emit_network_panel_connect_wifi_Public(ssid);
            
        }
      }
    })
    let innerdiv = document.createElement("div");
    innerdiv.innerHTML = `<svg class="Lock" xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="M240-80q-33 0-56.5-23.5T160-160v-400q0-33 23.5-56.5T240-640h40v-80q0-83 58.5-141.5T480-920q83 0 141.5 58.5T680-720v80h40q33 0 56.5 23.5T800-560v400q0 33-23.5 56.5T720-80H240Zm0-80h480v-400H240v400Zm296.5-143.5Q560-327 560-360t-23.5-56.5Q513-440 480-440t-56.5 23.5Q400-393 400-360t23.5 56.5Q447-280 480-280t56.5-23.5ZM360-640h240v-80q0-50-35-85t-85-35q-50 0-85 35t-35 85v80ZM240-160v-400 400Z"/></svg>
                        <svg class="Unlock" xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="M240-640h360v-80q0-50-35-85t-85-35q-50 0-85 35t-35 85h-80q0-83 58.5-141.5T480-920q83 0 141.5 58.5T680-720v80h40q33 0 56.5 23.5T800-560v400q0 33-23.5 56.5T720-80H240q-33 0-56.5-23.5T160-160v-400q0-33 23.5-56.5T240-640Zm0 480h480v-400H240v400Zm296.5-143.5Q560-327 560-360t-23.5-56.5Q513-440 480-440t-56.5 23.5Q400-393 400-360t23.5 56.5Q447-280 480-280t56.5-23.5ZM240-160v-400 400Z"/></svg>`;
    if (security) {
        innerdiv.classList.add("Lock");
    }

    div.appendChild(innerdiv);
    div.appendChild(span);
    div.appendChild(button);
    Panel_wifi_list.appendChild(div);


        
}

function Load_wifis(Wifis) {
    Panel_wifi_list.innerHTML = ""
    Wifis.forEach(wifi => {
        NewWifi(wifi.ssid, wifi.level, wifi.security, wifi.connected);
    });
    
}

function OpenConnect(SSID) {
    if (SSID != null) {
        Panel_wifi_connected_wifi.classList.add("view");
        panel_network_password.setAttribute("data-SSID", SSID);
        panel_network_connected_wifi_SSID_view.textContent = `Connected to: ${SSID}`
    }    
}
function CloseConnect(){
    Panel_wifi_connected_wifi.classList.remove("view");
    panel_network_password.removeAttribute("data-SSID");
    panel_network_password.value = "";
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
        let password = panel_network_password.value;
        let SSID = panel_network_password.getAttribute("data-SSID");
        emit_network_panel_connect_wifi(SSID, password);
        CloseConnect()
    }

})





// update_network("connected_wifi", 3)
// openWiFi()
