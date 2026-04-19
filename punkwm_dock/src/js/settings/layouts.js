const Layouts = ["al" ,"cn" ,"gb" ,"md" ,"ro", "at" ,"de" ,"gh" ,"ml" ,"se", "au" ,"dk" ,"ie" ,"ng" ,"sn", "be" ,"dz" ,"it" ,"nl" ,"tg", "br" ,"ee" ,"ke" ,"no" ,"tw", "bw" ,"fi" ,"kr" ,"ph" ,"us", "ch" ,"fo" ,"latam" ,"pl" ,"za", "cm" ,"fr" ,"lv" ,"pt"]
const main_Layouts = document.getElementById("main_Keyboard_Layout");

function getLayout() {
    return window.localStorage.getItem("LAYOUT") ?? "us";
}

function setLayout(layout) {
    window.localStorage.setItem("LAYOUT", layout);
}

function LoadLayouts() {
   let allLayouts = Layouts.slice().sort(); // mejor que copyWithin()
    let myLayout = getLayout();

    let index = allLayouts.indexOf(myLayout);
    if (index !== -1) {
        allLayouts.splice(index, 1);
    }
    

    let scroller = document.createElement("section");
    scroller.classList.add("scroller");
    scroller.appendChild(newLayout(myLayout, true));
    
    allLayouts.forEach(layout=>{
        scroller.appendChild(newLayout(layout));
    })
    main_Layouts.innerHTML = ""
    main_Layouts.appendChild(scroller)
    
}

function newLayout(layout, my = false) {
    let div = document.createElement("div");
    div.classList.add("layout");
    if (my) {
        div.classList.add("active");
    }
    div.style.setProperty("--bg", `url('../../assets/settings/Keyboard Layouts/${layout}.svg')`);
    div.innerHTML = `<span class="Name">${layout.toLocaleUpperCase()}</span>`;
    div.addEventListener("click", ()=>{
        setLayout(layout);
        LoadLayouts()
        // LoadLangs()
    })
    return div;
    
}
