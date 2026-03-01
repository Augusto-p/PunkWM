const clock = document.getElementById("clock");
const password_login = document.getElementById("password_login");
const user_name = document.getElementById("name");
const user_photo = document.getElementById("photo");
function Clock() {
    let dateTime = new Date()
    setTime(dateTime);
    function setTime(dateTime) {
        const name_day = new Intl.DateTimeFormat(undefined, { weekday: 'long' }).format(dateTime);
        const month = new Intl.DateTimeFormat(undefined, { month: 'long' }).format(dateTime);
        clock.innerHTML = `<span class="hour">${`0${dateTime.getHours()}`.slice(-2)}</span>
        <span class="hr">:</span>
        <span class="minute">${`0${dateTime.getMinutes()}`.slice(-2)}</span>
        <span class="hr">:</span>
        <span class="seconds">${`0${dateTime.getSeconds()}`.slice(-2)}</span>
        <span class="date">${name_day.charAt(0).toUpperCase() + name_day.slice(1)} ${dateTime.getDate()} ${month.charAt(0).toUpperCase() + month.slice(1)}</span>
        `;
    }
    setInterval(() => {
        let dateTime = new Date()
        setTime(dateTime)
    }, 999)
}
async function setWallpaerLockScreen(bg) {
    const contents = await window.__TAURI_PLUGIN_FS__.readFile(bg);
    const blob = new Blob([contents], { type: 'image/jpeg' });
    const blobUrl = URL.createObjectURL(blob);
    document.body.style.backgroundImage = `url('${blobUrl}')`;
    document.body.removeAttribute("data-mode")
}
async function setUserLockScreen(name, image) {
    user_name.textContent = name;
    const contents = await window.__TAURI_PLUGIN_FS__.readFile(image);
    const blob = new Blob([contents], { type: 'image/jpeg' });
    const blobUrl = URL.createObjectURL(blob);
    user_photo.src = `url('${blobUrl}')`;
}
function ToggleLoginPassword() {
    if (password_login.getAttribute("type") == "password") {
        password_login.setAttribute("type", "text");
    } else {
        password_login.setAttribute("type", "password");
    }
}
Clock()