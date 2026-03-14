const layout_btn = document.getElementById("Layout")
let Layout = 0;
const Layouts = {
    //CornerNE
    0: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <rect width="25.508108" height="14.080662" x="47.881504" y="-16.453354" transform="matrix(0,1.3133079,-1.4203878,0,0.01057566,0.46886035)" />
 <rect width="20.558773" height="14.080662" x="25.038422" y="-16.453354" transform="matrix(0,1.3133079,-1.4203878,0,0.01057566,0.46886035)" />
 <rect width="20.558773" height="14.080662" x="2.195339" y="-16.453354" transform="matrix(0,1.3133079,-1.4203878,0,0.01057566,0.46886035)" />
 <rect width="70" height="70" x="27.5" y="3.5" />
 <rect width="33.5" height="20" x="27.5" y="76.5" />
 <rect width="33.5" height="20" x="63.5" y="76.5" />
</svg>`,
    // CornerNW
    1: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <rect width="33.5" height="20" x="63.352016" y="-97.366089" transform="rotate(90)" />
 <rect width="27" height="20" x="33.352016" y="-97.380714" transform="rotate(90)" />
 <rect width="27" height="20" x="3.3520164" y="-97.380714" transform="rotate(90)" />
 <rect width="70" height="70" x="3.5" y="3.5" />
 <rect width="33.5" height="20" x="3.5" y="76.5" />
 <rect width="33.5" height="20" x="40" y="76.5" />
</svg>`,
    // CornerSE
    2: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <rect width="33.5" height="20" x="3.366086" y="-23.380714" transform="rotate(90)" />
 <rect width="27" height="19.999828" x="70.366089" y="-23.365913" transform="rotate(90)" />
 <rect width="27" height="20" x="40.366085" y="-23.380714" transform="rotate(90)" />
 <rect width="70" height="70" x="27.5" y="27.5" />
 <rect width="33.5" height="20" x="26.5" y="3.5" />
 <rect width="33.5" height="20" x="64" y="3.5" />
</svg>`,
    //CornerSW
    3: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <rect width="33.5" height="20" x="3.366086" y="-97.380714" transform="rotate(90)" />
 <rect width="27" height="19.999828" x="70.366089" y="-97.365913" transform="rotate(90)" />
 <rect width="27" height="20" x="40.366085" y="-97.380714" transform="rotate(90)" />
 <rect width="70" height="70" x="3.5" y="27.5" />
 <rect width="33.5" height="20" x="3.5" y="3.5" />
 <rect width="33.5" height="20" x="39.5" y="3.5" />
</svg>`,

    //TileR
    4: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <rect width="45.5" height="94" x="3.5" y="3.5" />
 <rect width="45.5" height="29.333" x="52" y="3.5" />
 <rect width="45.5" height="29.333" x="52" y="35.833332" />
 <rect width="45.5" height="29.333" x="52" y="68.166664" />
</svg>`,
    //TileB
    5: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <g transform="rotate(90,50.5,50.5)">
  <rect width="45.5" height="94" x="3.5" y="3.5" />
  <rect width="45.5" height="29.333" x="52" y="3.5" />
  <rect width="45.5" height="29.333" x="52" y="35.833332" />
  <rect width="45.5" height="29.333" x="52" y="68.166664" />
 </g>
</svg>`,
    //TileL
    6: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <g transform="rotate(180,50.5,50.5)">
  <rect width="45.5" height="94" x="3.5" y="3.5" />
  <rect width="45.5" height="29.333" x="52" y="3.5" />
  <rect width="45.5" height="29.333" x="52" y="35.833332" />
  <rect width="45.5" height="29.333" x="52" y="68.166664" />
 </g>
</svg>`,
    //TileT
    7: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <g transform="rotate(-90,50.5,50.5)">
 <rect width="45.5" height="94" x="3.5" y="3.5" />
 <rect width="45.5" height="29.333" x="52" y="3.5" />
 <rect width="45.5" height="29.333" x="52" y="35.833332" />
 <rect width="45.5" height="29.333" x="52" y="68.166664" />
 </g>
</svg>`,
    //FairH
    8: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <rect width="61.5" height="29.333" x="3.4998331" y="-97.500168" transform="rotate(90)" />
 <rect width="61.5" height="29.333" x="3.4998331" y="-32.833504" transform="rotate(90)" />
 <rect width="61.5" height="29.333" x="3.4998331" y="-65.166832" transform="rotate(90)" />
 <rect width="29.333334" height="45.5" x="68.166496" y="-97.500168" transform="rotate(90)" />
 <rect width="29.333334" height="45.5" x="68.166832" y="-49.000168" transform="rotate(90)" />
</svg>`,
    //FairV
    9: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <rect width="61.5" height="29.333" x="3.5" y="3.5"/>
 <rect width="61.5" height="29.333" x="3.5" y="68.166664"/>
 <rect width="61.5" height="29.333" x="3.5" y="35.833332"/>
 <rect width="29.333334" height="45.5" x="68.166664" y="3.5"/>
 <rect width="29.333334" height="45.5" x="68.167" y="52"/>
</svg>`,
    //FullScreen
    10: `<svg   viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <path d="M 3.5 3.5 L 3.5 50.5 L 50.5 3.5 L 3.5 3.5 z M 50.5 3.5 L 97.5 50.5 L 97.5 3.5 L 50.5 3.5 z M 97.5 50.5 L 50.5 97.5 L 97.5 97.5 L 97.5 50.5 z M 50.5 97.5 L 3.5 50.5 L 3.5 97.5 L 50.5 97.5 z " />
</svg>`,
    //Dwindle
    11: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <rect width="45.5" height="94" x="3.5" y="3.5" />
 <rect width="45.5" height="45.5" x="52" y="3.5" />
 <rect width="22" height="45.5" x="52" y="52" />
 <rect width="22" height="22" x="75.5" y="52" />
 <rect width="22" height="22" x="75.5" y="75.5" />
</svg>
`,
    //Magnifier
    12: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <path d="M 3.5 3.5 L 3.5 97.5 L 97.5 97.5 L 97.5 3.5 L 3.5 3.5 z M 13 14.5 L 88 14.5 L 88 89.5 L 13 89.5 L 13 14.5 z " />
 <rect width="60" height="60" x="20.5" y="20.5" />
</svg>`,
    //Max
    13: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <path d="M 5.6210938 3.5 L 50.5 48.378906 L 95.378906 3.5 L 5.6210938 3.5 z M 3.5 5.6210938 L 3.5 95.378906 L 48.378906 50.5 L 3.5 5.6210938 z M 97.5 5.6210938 L 52.621094 50.5 L 97.5 95.378906 L 97.5 5.6210938 z M 50.5 52.621094 L 5.6210938 97.5 L 95.378906 97.5 L 50.5 52.621094 z " />
</svg>`,
    //Spiral
    14: `<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
 <rect width="45.5" height="94" x="3.5" y="3.5" />
 <rect width="45.5" height="45.5" x="52" y="3.5" />
 <rect width="22" height="45.5" x="75.5" y="52" />
 <rect width="22" height="22" x="52" y="75.5" />
 <rect width="10.5" height="22" x="52" y="52" />
 <rect width="10.5" height="11" x="63.5" y="52" />
 <rect width="4.75" height="11" x="69" y="63.5" />
 <rect width="4.75" height="4.75" x="63.5" y="69.5" />
</svg>`,
};

function setLayout(layout) {
    Layout = layout % 15;
    layout_btn.innerHTML = Layouts[Layout]
}

function toggleLayout() {
    LayoutClsss.Toggle();
    
    Layout = (Layout +1) % 15;
    layout_btn.innerHTML = Layouts[Layout]
    //IPC
}

