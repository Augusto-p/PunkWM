const password_login = document.getElementById("password_login");
const user_login = document.getElementById("user_login");

function ToggleLoginPassword() {
    if (password_login.getAttribute("type") == "password") {
        password_login.setAttribute("type", "text");
        
    }else{
        password_login.setAttribute("type", "password");
    }   
}

function Login() {
    let password = password_login.value;
    let user = user_login.value;
    IPC_emit("System", "Login", {"User": user, "Password": password});   
}

password_login.addEventListener("keyup", (e)=>{
    if (e.key === "Enter") {
        Login();
    }
});