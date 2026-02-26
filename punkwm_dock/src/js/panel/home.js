const home_date_day = document.getElementById("date-Day");
const home_date_date = document.getElementById("date-Date");
const stats_Content = document.getElementById("stats");

function openHome(){
    body.dataset.panel = "Open-Home";
    const today = new Date();
    const name_day = new Intl.DateTimeFormat(undefined, {weekday: 'long'}).format(today);
    const month = new Intl.DateTimeFormat(undefined, {month: 'long'}).format(today);
    home_date_day.textContent = name_day.charAt(0).toUpperCase() + name_day.slice(1);
    home_date_date.textContent = `${today.getDate()} ${month.charAt(0).toUpperCase() + month.slice(1)}`;
    PanelHome.Open();
}

function ToggleHome(){
    let Panel_Mode = body.getAttribute("data-panel");
    if (Panel_Mode == null){
        Panel.Open();
        openHome();
    }else if (Panel_Mode != "Open-Home") {
        openHome();
    }else{
        panel_close();
    }


}
openHome()

