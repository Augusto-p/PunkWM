let WeatherUnits = document.getElementById("WeatherUnits");
const WeatherMap = L.map("WeatherMap");
let WeatherMapMarker;
L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {attribution: '&copy; OpenStreetMap'}).addTo(WeatherMap);

WeatherUnits.addEventListener("change", ()=>{
  Storage.setItem("Weather-Units", WeatherUnits.checked);
  Weather.SaveUnits(this.checked);
})

function LoadWeatherUnits() {
  WeatherUnits.checked = Storage.getItem("Weather-Units") ?? false;
}

function getLat() {
  return Storage.getItem("Weather-Lat") ?? 52.075783;
}
function getLon() {
  return Storage.getItem("Weather-Lon") ?? -1.010332;
}


function LoadMap() {
  let lat = getLat();
  let lng = getLon();
  WeatherMap.setView([lat, lng], 13);
  if (WeatherMapMarker) {
    WeatherMapMarker.setLatLng([lat, lng]);
  } else {
    WeatherMapMarker = L.marker([lat, lng]).addTo(WeatherMap);
  }
}

WeatherMap.on('click', function (e) {
  const { lat, lng } = e.latlng;
  Storage.setItem("Weather-Lat", lat);
  Storage.setItem("Weather-Lon", lng);

  console.log(`LAT: ${lat}, LNG: ${lng}`);

  fetch(`https://www.accuweather.com/web-api/three-day-redirect?lat=${lat}&lon=${lng}&lang=${getLang()}`).then(res=>{
    res.text().then(data=>{
      let html = document.createElement("div")
      html.innerHTML = data.split("<body")[0] + "</html>";
      let jsondata = JSON.parse(html.getElementsByTagName("script")[1].textContent);
      let City = new DOMParser().parseFromString(jsondata["address"]["addressLocality"].split(",")[0], "text/html").documentElement.textContent;
      let State = new DOMParser().parseFromString(jsondata["address"]["addressRegion"], "text/html").documentElement.textContent;
      let Country = new DOMParser().parseFromString(jsondata["geo"]["addressCountry"], "text/html").documentElement.textContent;
      Weather.SaveLocation(City, State, Country);      
    })
  })
  if (WeatherMapMarker) {
    WeatherMapMarker.setLatLng([lat, lng]);
  } else {
    WeatherMapMarker = L.marker([lat, lng]).addTo(WeatherMap);
  }
});

LoadMap();