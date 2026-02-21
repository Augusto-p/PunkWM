const Songs = document.getElementById("Songs");
const Panel_Music = document.getElementById("Panel_Music");
const panel_music_searchbar = document.getElementById("panel_music_searchbar");
const Local_Music_Player = document.getElementById("Local_Music_Player");
function ToogleMusic(){
    let Panel_Mode = body.getAttribute("data-panel");
    if (Panel_Mode == null){
        emit_Open_Panel();
        openMusic();
    }else if (Panel_Mode != "Open-Music") {
        openMusic();
    }else{
        panel_close();
    }

}

function openMusic(){
    body.dataset.panel = "Open-Music";
    emit_panel_apps_open();
}

function newSong(id, title, artist, album, duration, cover, mode) {
    let div = document.createElement("div");
    div.classList.add("song");
    div.innerHTML = `<div class="img" style="background-image: url('${cover}');">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="m380-300 280-180-280-180v360ZM480-80q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q134 0 227-93t93-227q0-134-93-227t-227-93q-134 0-227 93t-93 227q0 134 93 227t227 93Zm0-320Z"/></svg>
                    <span>${duration}</span>
                </div>
                <span class="Title">${title}</span>
                <span class="Atrist">${artist}</span>
                <span class="Album">${album}</span>`;
    div.addEventListener("click", ()=>{
        if (mode == "Local") {
            Local_Music_Player.src = id;
            Local_Music_Player.play();
            Load_Song(title, artist, album, cover, mmssToInt(duration), mode)
        }
        console.log("Play: ", id);
        
    });
    Songs.appendChild(div)    
}
function load_Songs(songs) {
    Songs.innerHTML = "";
    songs.forEach(song => {
        newSong(song.id, song.title, song.artist, song.album, song.duration, song.cover, song.mode);
        
    });
}

function ToogleMusicSourse(mode) {
    Panel_Music.setAttribute("data-mode", mode)
}

panel_music_searchbar.addEventListener("keyup", (e)=>{
    if (e.key === "Enter") {
        console.log("Sarch: ", panel_music_searchbar.value);
    }
    
})






ToogleMusic()
newSong("FÍGARO - Angie Corine & Helf - Single.mp3", "Call Me Maybe", "Carly Rae Jepsen", "Kiss", "3:14", "https://lh3.googleusercontent.com/74iPDM13qn-WhH5hMEmsO3A0mXvE5fAYYjNfO0rzRvA4DvjwoGoZN3S3fp8yJVAJB5PwQqUvldC9U8S_=w544-h544-l90-rj", "Local");