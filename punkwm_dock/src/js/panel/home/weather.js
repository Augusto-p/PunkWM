const weather_view = document.getElementById("weather-view");

function directionToDeg(dir) {
  const dirs = {
    N: 0, NNE: 22.5, NE: 45, ENE: 67.5,
    E: 90, ESE: 112.5, SE: 135, SSE: 157.5,
    S: 180, SSW: 202.5, SW: 225, WSW: 247.5,
    W: 270, WNW: 292.5, NW: 315, NNW: 337.5
  };

  return dirs[dir.toUpperCase()] ?? null;
}

function Load_weather(temp, phrase, icon, wind_direction, wind_speed) {
    weather_view.innerHTML = `<span id="weather-phrase">${phrase}</span>
                    <span id="weather-temp">${temp}</span>
                    <img id="weather-icon" src="assets/weathericons/${icon}.svg">
                    <span id="weather-wind">${wind_speed}</span>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -0.601 59.027 77.05" id="weather-wind-icon" style="transform: rotateZ(${directionToDeg(wind_direction)}deg);">
                        <g transform="matrix(1, 0, 0, 1, 213.758502, 193.4267)">
                            <path d="M -186.61 -191.32 L -212.38 -121.37 C -213.2 -119.13 -210.72 -117.12 -208.7 -118.39 L -184.25 -133.81 L -159.79 -118.39 C -157.77 -117.12 -155.29 -119.13 -156.11 -121.37 L -181.9 -191.37 C -182.37 -192.47 -183.18 -193 -184.28 -193.01 C -185.38 -193.01 -186.3 -192.15 -186.61 -191.32 Z M -186.75 -176.49 L -186.75 -138.15 L -205.15 -126.55 L -186.75 -176.49 Z"></path>
                        </g>
                    </svg>`;
    weather_view.removeAttribute("data-mode");
}