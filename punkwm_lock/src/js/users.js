const password_login_user = document.getElementById("password_login_user");
const login_user_avatar = document.getElementById("login_user_avatar");
const users_list = document.getElementById("users_list");

function ToggleLoginUserPassword() {
    if (password_login_user.getAttribute("type") == "password") {
        password_login_user.setAttribute("type", "text");
        
    }else{
        password_login_user.setAttribute("type", "password");
    }   
}


function newUser(name, image) {
    let button = document.createElement("button");
    button.classList.add("user");
    button.innerHTML = `<div class="avatar" style="background-image: url('${image}')"></div><h3>${name}</h3>`;
    button.addEventListener("click", ()=>{
        password_login_user.setAttribute("data-user", name);
        login_user_avatar.style.backgroundImage = `url('${image}')`;
        document.body.setAttribute("data-mode", "user");
    });
    users_list.appendChild(button);
}
function load_users(users) {
    users_list.innerHTML = "";
    users.forEach(user => {
        newUser(user.name, user.image);
    });
}

function notList() {
    document.body.removeAttribute("data-mode");
}

function LoginUser() {
    let password = password_login_user.value;
    let user = password_login_user.getAttribute("data-user");
    IPC_emit("System", "Login", {"User": user, "Password": password});
}

password_login_user.addEventListener("keyup", (e)=>{
    if (e.key === "Enter") {
        LoginUser();
    }
});



load_users([
    {
        "image": 'https://gpticketstore.vshcdn.net/uploads/images/12172/teams-lineups-f1-max-verstapen.jpg',
        "name": "Max Vestappen",
    }, 
    {
        "image": 'https://imageio.forbes.com/specials-images/imageserve/6753832b78f53643f93a2dbc/0x0.jpg?format=jpg&crop=1263,1266,x300,y30,safe&height=416&width=416&fit=bounds',
        "name": "Charles Leclerc",
    },
    {
        "image": 'https://ichef.bbci.co.uk/ace/standard/972/cpsprodpb/aba6/live/7991d4d0-c5dd-11f0-bca0-d197f0674a9d.jpg',
        "name": "Lewis Hamilton",
    },
    {
        "image": 'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRjkBrZ2r4jRgICj3n2vF1Vhbbz4bvL2RoT3Q&s',
        "name": "Fernando Alonso",
    },
    {
        "image": 'https://media.gq.com.mx/photos/671bf1a17917c4a39b483898/master/w_1600%2Cc_limit/Sergio_Checo_Perez_gp_cdmx_2024-2180835921.jpg',
        "name": "Sergio \"Checo\" Pérez",
    },
    {
        "image": 'https://img2.51gt3.com/rac/racer/202503/f10f01a1704147ca90ab3a4325f38785.png?x-oss-process=style/_nhd_en',
        "name": "George Russell",
    },
    {
        "image": 'https://cdn-3.motorsport.com/images/amp/6xERX380/s1000/valtteri-bottas-mercedes.jpg',
        "name": "Valtteri Bottas",
    },
    {
        "image": 'https://media.formula1.com/image/upload/f_auto,c_limit,q_75,w_1320/content/dam/fom-website/drivers/2025Drivers/tsunoda',
        "name": "Yuki Tsunoda",
    },
    {
        "image": 'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRyyAJuMmdbU0LFiGAVI99n7NaczyqsDJX9bA&s',
        "name": "Nicholas \"The G.O.A.T\" Latifi",
    }
])