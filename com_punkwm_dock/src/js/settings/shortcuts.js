const Shortcuts = document.getElementById("Shortcuts")
let keybindings = []

async function getKeybindingsDir() {
    let response = await fetch(`assets/settings/JSONs/Shortcuts.json`, { method: "GET", });
    let data = await response.json();
    keybindings = data;
    LoadKeybindings()
}


function ViewShortcut(keys) {
    return keys.replace("CTRL", "⌘").replace("SHIFT", "⇧").replace("ALT", "⌥").replace("SUPER", "⊞").replace("RETURN", "⏎")
            .replace("BACKSPACE", "⌫").replace("ALT GR", "⎇").replace("SPACE", "␣")
            .replaceAll("+", " + ").trim()
    
}



function newShortcut(command, Shortcut) {
    let div = document.createElement("div");
    div.classList.add("row");
    let Name = "";
    let Desc = "";
    if (command.startsWith("Open:")) {
        Name = Lang["Keyboard Shortcut"]["Open:APP"]["Name"].replace("%APP%", command.replace("Open:", ""));
        Desc = Lang["Keyboard Shortcut"]["Open:APP"]["Description"].replace("%APP%", command.replace("Open:", ""));
    } else {
        Name = Lang["Keyboard Shortcut"][command]["Name"];
        Desc = Lang["Keyboard Shortcut"][command]["Description"];

    }


    div.innerHTML = `<span class="Name">${Name}</span>
                    <span class="Command">${command}</span>
                    <span class="Description">${Desc}</span>`;
    let span = document.createElement("span");
    span.classList.add("Shortcut");
    span.innerHTML = `<c>[</c>${ViewShortcut(Shortcut) ?? ""}<c>]</c>`
    let input = document.createElement("input");
    input.type = "text";
    let keycommand = "";
    input.addEventListener("keydown", (e) => {
        e.preventDefault();
        const keys = [];
        // 1. Detectar Modificadores
        if (e.ctrlKey) keys.push('Ctrl');
        if (e.shiftKey) keys.push('Shift');
        if (e.altKey) keys.push('Alt');
        if (e.metaKey) keys.push('Super');
        const keyName = e.key;
        const isModifier = ['Control', 'Shift', 'Alt', 'Meta'].includes(keyName);

        if (!isModifier) {
            // Limpieza estética de la tecla (ej: " " -> "Espacio")
            let displayKey = keyName;
            if (keyName === " ") displayKey = "Space";
            if (keyName === "AltGraph") displayKey = "Alt Gr";
            if (keyName === "ArrowUp") displayKey = "↑";
            if (keyName === "ArrowDown") displayKey = "↓";
            if (keyName === "ArrowLeft") displayKey = "←";
            if (keyName === "ArrowRight") displayKey = "→";
            if (keyName === "ENTER") displayKey = "RETURN";

            keys.push(displayKey.length === 1 ? displayKey.toUpperCase() : displayKey);
        }
        // 3. Renderizar en el input
        // Solo actualizamos si hay al menos una tecla presionada
        if (keys.length > 0) {
            span.innerHTML = `<c>[</c>${ViewShortcut(keys.join(' + ').toUpperCase())}<c>]</c>`
            keycommand = keys.join('+').toUpperCase();
            
        }
    })

    input.addEventListener("blur", ()=>{
        shortcuts = JSON.parse(Storage.getItem("Keybindings"));
        shortcuts[command] = keycommand;
        Keybindings.Save(command, keycommand);
        
    })

    div.appendChild(input)
    div.appendChild(span);
    Shortcuts.appendChild(div)
}

function LoadKeybindings() {
    Object.keys(getApps()).forEach(app => {
        if (!keybindings.includes(`Open:${app}`)) {
            keybindings.push(`Open:${app}`);
        }
    })

    shortcuts = JSON.parse(Storage.getItem("Keybindings"));
    Shortcuts.innerHTML = "";
    keybindings.sort().forEach(command => {
        newShortcut(command, shortcuts[command])
    });
}

