const SongWidget = document.getElementById("Music");
const SongWidgetTittle = document.getElementById("MusicTittle");
const SongWidgetArtist = document.getElementById("MusicArtist");
const SongWidgetAlbum = document.getElementById("MusicAlbum");
const SongWidgetMusicCurrentTimeSvg = document.getElementById("MusicCurrentTimeSvg");
const SongWidgetMusicCurrentTime = document.getElementById("MusicCurrentTime");
const SongWidgetMusicDuration = document.getElementById("MusicDuration");
const SongWidgetMusicPlayPause = document.getElementById("MusicPlayPause");

const SysVolume = document.getElementById("VolumeBox");
const SysGlow = document.getElementById("GlowBox");


function SongWidgetUpdateCurrentTimeSvg(percent) {
  degrees = percent * 3.6;
  SongWidgetMusicCurrentTimeSvg.innerHTML = "";
  const cx = 100;
  const cy = 100;
  const r = 80;
  const radians = (degrees - 90) * Math.PI / 180;
  const x = cx + r * Math.cos(radians);
  const y = cy + r * Math.sin(radians);
  const largeArcFlag = degrees > 180 ? 1 : 0;
  const pathData = `
    M ${cx} ${cy}
    L ${cx} ${cy - r}
    A ${r} ${r} 0 ${largeArcFlag} 1 ${x} ${y}
    Z
  `;
  const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
  path.setAttribute("d", pathData);
  SongWidgetMusicCurrentTimeSvg.appendChild(path);
}

function SongWidgetUpdateCurrentTime(currentTime) {
  SongWidgetMusicCurrentTime.textContent = formatTime(currentTime);
  let duration = parseInt(SongWidget.getAttribute("data-duration"), 10);
  let percent = currentTime * 100 / duration;
  SongWidgetUpdateCurrentTimeSvg(percent);
  SongWidget.setAttribute("data-currentTime", currentTime);
  if (duration == currentTime) {
    SongWidgetNext();
  }
}

function SongWidgetLoad(position, title, artist, album, cover, duration, mode) {
  SongWidget.setAttribute("data-duration", duration);
  SongWidget.setAttribute("data-mode", mode);
  SongWidget.setAttribute("data-position", position);
  SongWidget.style.backgroundImage = `url("${cover}")`;
  SongWidgetTittle.textContent = title;
  SongWidgetArtist.textContent = artist;
  SongWidgetAlbum.textContent = album;
  SongWidgetMusicDuration.textContent = duration;
}

function SongWidgetNext() {
  if (window.localStorage.getItem("MusicSourse") == "Local") {
    let position = SongWidget.getAttribute("data-position");
    let songsChildNodesLength = Songs.childNodes.length;
    Songs.childNodes[( position + 1) % songsChildNodesLength].click();
  }
}

function SongWidgetPre() {
  if (window.localStorage.getItem("MusicSourse") == "Local") {
    let currentTime = SongWidget.getAttribute("data-currentTime");
    if (currentTime > 5) {
      // emit_Music_Panel_Local_Reset_Songs();
      PanelMusicLocal.ResetSong();
    } else {
      let position = SongWidget.getAttribute("data-position");
      let songsChildNodesLength = Songs.childNodes.length;
      Songs.childNodes[(position - 1) % songsChildNodesLength].click()
    }
  }
}

function SongWidgetPlayPause() {
  if (SongWidgetMusicPlayPause.getAttribute("data-mode") == "play") {
    SongWidgetMusicPlayPause.setAttribute("data-mode", "pause");
    if (window.localStorage.getItem("MusicSourse") == "Local") {
      // emit_Music_Panel_Local_Pause_Songs()
      PanelMusicLocal.PauseSong();
    } else if (window.localStorage.getItem("MusicSourse") == "YT_Music") {
      emit_Music_Panel_YTMusic_Pause_Song();
    }
  } else {
    SongWidgetMusicPlayPause.setAttribute("data-mode", "play");
    if (window.localStorage.getItem("MusicSourse") == "Local") {
      // emit_Music_Panel_Local_Play_Songs()
      PanelMusicLocal.PlaySong();
    } else if (window.localStorage.getItem("MusicSourse") == "YT_Music") {
      emit_Music_Panel_YTMusic_Play_Song();
    }
  }

}

SysVolume.addEventListener("click", (e)=>{
  let percent = roundToMultiple(Math.round(100 - e.layerY * 100 / SysVolume.offsetHeight));
  SysVolume.setAttribute("data-Volume", percent);
  SysVolume.style.setProperty("--value", percent);
  //UpdateVolumenBack 
})


SysGlow.addEventListener("click", (e)=>{
  let percent = roundToMultiple(Math.round(100 - e.layerY * 100 / SysGlow.offsetHeight));
  SysGlow.setAttribute("data-Glow", percent);
  SysGlow.style.setProperty("--value", percent);
  //UpdateGlowBack 
})

function roundToMultiple(x) {
  let x2 = Math.round(x / 2) * 2;
  let x5 = Math.round(x / 5) * 5;
  if (Math.abs(x - x2) < Math.abs(x - x5)){
    return x2;
  }
  return x5
  
}