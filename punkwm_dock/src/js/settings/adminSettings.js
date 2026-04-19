const LockScreenWallpaerView = document.getElementById("LockScreenWallpaerView");
const LockScreenWallpaerInput = document.getElementById("LockScreenWallpaerInput");

LockScreenWallpaerInput.addEventListener("change", ()=>{
    if (LockScreenWallpaerInput.files && LockScreenWallpaerInput.files[0]) {
        let reader = new FileReader();
        reader.onload = function (e) {
            LockScreenWallpaerView.style.backgroundImage = `url('${e.target.result}')`;
        };
        reader.readAsDataURL(LockScreenWallpaerInput.files[0]);
    }
})