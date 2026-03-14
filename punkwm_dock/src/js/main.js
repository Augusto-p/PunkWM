const body = document.body
const workspaces = document.getElementById("workspaces")
const batteryBar = document.getElementById("BatteryBar")
const clock = document.getElementById("Clock")
const network = document.getElementById("Network")




function getRandomInt(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min) + min);
}



function Clock() {
    let dateTime = new Date()
    setTime(dateTime.getHours(), dateTime.getMinutes())

    function setTime(hour, minute) {
        clock.innerHTML = `<span>${`0${hour}`.slice(-2)}</span><span>${`0${minute}`.slice(-2)}</span>`

    }

    setInterval(() => {
        let dateTime = new Date()
        if (dateTime.getSeconds() != 0) {
            return
        }
        setTime(dateTime.getHours(), dateTime.getMinutes())

    }, 999)
}








Clock()
// updateWorkspaces([-1, 0, 0, 0, 0, 0, 0, 0, 0])