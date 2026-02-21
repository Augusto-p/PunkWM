const Music_Art = document.getElementById("Music-image");
const Music_Title = document.getElementById("Music-Tittle");
const Music_Artist = document.getElementById("Music-Artist");
const Music_Album = document.getElementById("Music-Album");
const Music_Time = document.getElementById("Music-time");

function Load_Song(Title,Artist, Album, Art, Duration, mode) {
    Music_Title.textContent = Title.trim();
    Music_Artist.textContent = Artist.trim();
    Music_Album.textContent = Album.trim();
    Music_Art.src = Art;
    Music_Time.textContent = `00:00/${formatTime(Duration)}`;
}

function Song_Time_Update(seconds) {
    Music_Time.textContent = `${formatTime(seconds)}/${Music_Time.textContent.slice(Music_Time.textContent.indexOf("/") + 1)}`;
}

function formatTime(seconds) {
  const min = Math.floor(seconds / 60);
  const sec = Math.floor(seconds % 60);

  return `${min.toString().padStart(2, "0")}:${sec.toString().padStart(2, "0")}`;

}
function mmssToInt(tiempo) {
  const [min, seg] = tiempo.split(":").map(Number);
  return (min * 60) + seg;
}