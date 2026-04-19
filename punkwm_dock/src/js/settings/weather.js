// https://www.accuweather.com/web-api/three-day-redirect?lat=-34.905824075638236&lon=-56.17794159905825&target=

const map = L.map('WeatherMap').setView([52.075783, -1.010332], 13);

  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; OpenStreetMap'
  }).addTo(map);

  let marker;

  map.on('click', function(e) {
    const { lat, lng } = e.latlng;

    console.log(`Lat: ${lat}, Lng: ${lng}`);
    
    

    if (marker) {
      marker.setLatLng([lat, lng]);
    } else {
      marker = L.marker([lat, lng]).addTo(map);
    }
  });