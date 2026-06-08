const UserImageInput = document.getElementById("UserImageInput");
const UserImageView = document.getElementById("UserImage");
const UserNameInput = document.getElementById("UserName");


UserImageInput.addEventListener("change", ()=>{
    if (UserImageInput.files && UserImageInput.files[0]) {
        let reader = new FileReader();
        reader.onload = function (e) {
            User.SaveImage(e.target.result);
            UserImageView.style.backgroundImage = `url('${e.target.result}')`;
        };
        reader.readAsDataURL(UserImageInput.files[0]);
    }
})
