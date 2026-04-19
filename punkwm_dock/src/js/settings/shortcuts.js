const Shortcuts = document.getElementById("Shortcuts")

function newShortcut(command, Shortcut) {
    let div = document.createElement("div");
    div.classList.add("row");
    div.innerHTML = `<span class="Name">${Lang["Keyboard Shortcut"][command]["Name"]}</span>
                    <span class="Command">${command}</span>
                    <span class="Description">${Lang["Keyboard Shortcut"][command]["Description"]}</span>`;
    let span = document.createElement("span");
    span.classList.add("Shortcut");
    span.innerHTML = `<c>[</c>${Shortcut}<c>]</c>`
    let input = document.createElement("input");
    input.type = "text";
    input.addEventListener("keydown", (e)=>{
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
            if (keyName === " ") displayKey = "Espacio";
            if (keyName === "AltGraph") displayKey = "Alt Gr";
            if (keyName === "ArrowUp") displayKey = "↑";
            if (keyName === "ArrowDown") displayKey = "↓";
            if (keyName === "ArrowLeft") displayKey = "←";
            if (keyName === "ArrowRight") displayKey = "→";
            
            keys.push(displayKey.length === 1 ? displayKey.toUpperCase() : displayKey);
        }
        // 3. Renderizar en el input
        // Solo actualizamos si hay al menos una tecla presionada
        if (keys.length > 0) {
            span.innerHTML =  `<c>[</c>${keys.join(' + ')}<c>]</c>`
        }
    })

    
    div.appendChild(input)
    div.appendChild(span);
    Shortcuts.appendChild(div)
}

