const MusicFoldersList = document.getElementById("MusicFoldersList");


function NewMusicFolder(name, path) {
    let div = document.createElement("div");
    div.classList.add("folder");
    let color = get_Music_Folder_Color(name);
    div.innerHTML = `<svg style="fill: ${color} ;" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 720 720">
                <path
                    d="M 148.70312 87.144531 C 132.54801 87.144531 119.3471 99.340363 117.76367 115.06445 L 100.59375 115.06445 C 82.028747 115.06445 66.774728 128.5646 63.962891 146.31445 L 55.673828 146.32422 A 33.994992 33.994992 0 0 0 21.849609 183.28711 L 57.035156 584.69336 A 38.095903 38.095903 0 0 0 94.828125 619.46289 L 633.86914 621.70312 A 40.11485 40.11485 0 0 0 674.01172 584.92578 L 707.57031 182.92188 A 34.485144 34.485144 0 0 0 673.16406 145.56836 L 662.69531 145.58203 C 659.58561 128.19306 644.4859 115.06445 626.17578 115.06445 L 260.42773 115.06445 C 258.8443 99.340363 245.6434 87.144531 229.48828 87.144531 L 148.70312 87.144531 z M 497.06445 221.9707 L 497.06445 442.17578 C 497.06445 460.97813 491.14916 477.03835 479.31836 490.35547 C 467.49044 503.67583 453.2271 510.33594 436.52734 510.33594 C 419.83047 510.33594 405.56517 503.67583 393.73438 490.35547 C 381.90646 477.03835 375.99414 460.97813 375.99414 442.17578 C 375.99414 423.37668 381.90646 407.31645 393.73438 393.99609 C 405.56517 380.67898 419.83047 374.02148 436.52734 374.02148 C 443.15766 374.02148 449.36574 375.1315 455.1543 377.35156 C 460.94285 379.57487 466.26459 382.9082 471.11914 387.35156 L 471.11914 303.08594 L 323.60156 335.07227 L 323.60156 481.125 C 323.60156 499.92735 317.68627 515.98562 305.85547 529.30273 C 294.02467 542.62309 279.76133 549.2832 263.06445 549.2832 C 246.36758 549.2832 232.10424 542.62309 220.27344 529.30273 C 208.44264 515.98562 202.52734 499.92735 202.52734 481.125 C 202.52734 462.3259 208.44264 446.26567 220.27344 432.94531 C 232.10424 419.6282 246.36758 412.96875 263.06445 412.96875 C 269.69477 412.96875 275.9048 414.07877 281.69336 416.29883 C 287.47903 418.52213 292.80077 421.85546 297.6582 426.29883 L 297.6582 260.91992 L 497.06445 221.9707 z " />
            </svg> 
            <span>${name}</span>`;
    div.addEventListener("click", ()=>{
        console.log(path);
        
    })

    let close = document.createElement("button");
    close.classList.add("close");
    close.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>`
    close.addEventListener("click", ()=>{
        console.log("delete ", name);
        
    })

    div.appendChild(close)
    MusicFoldersList.appendChild(div)

    
}

function get_Music_Folder_Color(name) {
    let NameCode ="";
    name.split("").forEach(leter => {
        NameCode += leter.charCodeAt(0)
        
    });

    let n = Math.floor( NameCode.length / 3)

    let PartR = NameCode.substring(0, n) != "" ? (parseInt(NameCode.substring(0, n)) % 256).toString(16) : "0";
    let PartB = NameCode.substring(n,2*n)  != "" ? (parseInt(NameCode.substring(n,2*n)) % 256).toString(16): "0";
    let PartG = NameCode.substring(2*n, NameCode.length) != "" ?  (parseInt(NameCode.substring(2*n, NameCode.length)) % 256).toString(16): "0";
    return `#${PartR}${PartB}${PartG}`;
}


MusicFoldersList.addEventListener("dragover", (e) => {
  e.preventDefault();
});

MusicFoldersList.addEventListener("drop", async (e) => {
  e.preventDefault();

  for (const file of e.dataTransfer.files) {
    console.log("Ruta:", file);

    // 👉 En Tauri esto SÍ existe
    if (file.path) {
      console.log("Elemento dropeado:", file.path);
    }
  }
});

NewMusicFolder("CAR", "")
NewMusicFolder("Music", "")