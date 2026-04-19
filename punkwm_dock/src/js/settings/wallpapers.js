const mainWallpeperView = document.getElementById("WallpaerView");
const lockWallpeperView = document.getElementById("LockWallpaerView");
const mainWallpeperInput = document.getElementById("WallpaersInput");
const lockWallpeperInput = document.getElementById("LockWallpaerInput");



mainWallpeperInput.addEventListener("change", ()=>{
    if (mainWallpeperInput.files && mainWallpeperInput.files[0]) {
        let reader = new FileReader();
        reader.onload = function (e) {
            mainWallpeperView.style.backgroundImage = `url('${e.target.result}')`;
        };
        reader.readAsDataURL(mainWallpeperInput.files[0]);
    }
})


lockWallpeperInput.addEventListener("change", ()=>{
    if (lockWallpeperInput.files && lockWallpeperInput.files[0]) {
        let reader = new FileReader();
        reader.onload = function (e) {
            lockWallpeperView.style.backgroundImage = `url('${e.target.result}')`;
        };
        reader.readAsDataURL(lockWallpeperInput.files[0]);
    }
})