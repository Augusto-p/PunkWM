let weather_city = "Francorchamps";
let weather_state = "Liege";
let weather_country = "Belgium";
let weather_lang = "en";
let weather_units = "metric"; // imperial
function panel_load(dock_width, panel_width) {
    document.head.innerHTML += `
    <style>
        body {--dock_vw: ${dock_width/100}px;    --panel_vw: ${panel_width/100}px;}
        body[data-panel^="Open-"]{grid-template-columns: ${dock_width}px ${panel_width}px;}
        body{grid-template-columns: ${dock_width}px;}
        
    </style>`;
}

panel_load(80,300)
function panel_close() {
    body.removeAttribute("data-panel");
    emit_Close_Panel();
    
}
