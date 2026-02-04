function updateBattery(charge, percentage) {
    if (charge) {
        batteryBar.style.stroke = "#6266FF"
    } else if (percentage <= 20) {
        batteryBar.style.stroke = "#E06656"
    } else {
        batteryBar.style.stroke = "#9CD9A0"
    }
    batteryBar.style.setProperty("--Battery", percentage)


}