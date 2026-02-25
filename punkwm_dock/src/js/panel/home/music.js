const Music_Art = document.getElementById("Music-image");
const Music_Title = document.getElementById("Music-Tittle");
const Music_Artist = document.getElementById("Music-Artist");
const Music_Album = document.getElementById("Music-Album");
const Music_Time = document.getElementById("Music-time");
const Music_P = document.getElementById("Music-P");

function Load_Song(index, Title,Artist, Album, Art, Duration, mode) {
    Music_Title.textContent = Title.trim();
    Music_Artist.textContent = Artist.trim();
    Music_Album.textContent = Album.trim();
    Music_Art.src = Art;
    Music_Time.textContent = `00:00/${formatTime(Duration)}`;
    Music_P.setAttribute("data-index", index);
}

function Song_Time_Update(seconds) {
    Music_Time.textContent = `${formatTime(seconds)}/${Music_Time.textContent.slice(Music_Time.textContent.indexOf("/") + 1)}`;
}

function formatTime(seconds) {
  const min = Math.floor(seconds / 60);
  const sec = Math.floor(seconds % 60);

  return `${min.toString()}:${sec.toString().padStart(2, "0")}`;

}
function mmssToInt(tiempo) {
  const [min, seg] = tiempo.split(":").map(Number);
  return (min * 60) + seg;
}

function PlayPauseSong() {
  if (Music_P.getAttribute("data-mode") == "play") {
      Music_P.setAttribute("data-mode", "pause")
      if (window.localStorage.getItem("MusicSourse") == "Local") {
          emit_Music_Panel_Local_Pause_Songs()
      }else if(window.localStorage.getItem("MusicSourse") == "YT_Music") {
        emit_Music_Panel_YTMusic_Pause_Song();
      }
  }else{
    Music_P.setAttribute("data-mode", "play")
    if (window.localStorage.getItem("MusicSourse") == "Local") {
          emit_Music_Panel_Local_Play_Songs()
    }else if(window.localStorage.getItem("MusicSourse") == "YT_Music") {
      emit_Music_Panel_YTMusic_Play_Song();
    }
  }
  
}


function PreSong() {
  if (window.localStorage.getItem("MusicSourse") == "Local") {
    if (Local_Music_Player.currentTime > 5) {
        emit_Music_Panel_Local_Reset_Songs();
    }else{
      let currentIndex = Music_P.getAttribute("data-index");
      Songs.childNodes[(currentIndex+1)%Songs.childNodes.length].click()
    }
  }  
}
function NextSong() {
  if (window.localStorage.getItem("MusicSourse") == "Local") {
    let currentIndex = Music_P.getAttribute("data-index");
    Songs.childNodes[(currentIndex+1)%Songs.childNodes.length].click()
  }  
}