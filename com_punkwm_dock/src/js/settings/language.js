const main_Language = document.getElementById("main_Language");
async function getLanguagesDir() {
    let response = await fetch(`assets/settings/JSONs/Langs.json`, { method: "GET",});
    let data = await response.json();
    return data
}

function getLang() {
    return Storage.getItem("LANG")?? "English";
}

function setLang(lang, save = true) {
    Storage.setItem("LANG", lang);
    Language.Save(lang);
    if (save) {
        LoadLagUI();
    }
}

async function LoadLangs() {
    const langs = await getLanguagesDir()
    let allLangs = Object.assign({}, langs);;
    let myLang = allLangs[getLang()];
    delete allLangs[getLang()];

    let scroller = document.createElement("section");
    scroller.classList.add("scroller");
    scroller.appendChild(newLang(getLang(),myLang, true));
    
    Object.keys(allLangs).forEach(key=>{
        const lang = allLangs[key];
        scroller.appendChild(newLang(key,lang));
    })
    main_Language.innerHTML = ""
    main_Language.appendChild(scroller)
    
}

function newLang(id, lang, my = false) {
    let div = document.createElement("div");
    div.classList.add("lang");
    if (my) {
        div.classList.add("active");
    }
    div.style.setProperty("--bg", `url('./${lang.flag}')`);
    div.innerHTML = `<span class="Name">${lang.name}</span><span class="NameEN">${lang.nameEN}</span>`;
    div.addEventListener("click", ()=>{
        setLang(id);
        LoadLangs()
    })
    return div;
    
}
