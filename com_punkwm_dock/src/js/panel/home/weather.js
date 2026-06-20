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

async function Weather_Load(Lang, City, State, Country, Units) {
    try {
        const lang = "es"    
        const headers = {"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36","Accept": "*/*","Accept-Language": lang,"Referer": "https://www.accuweather.com/"};
        const params = new URLSearchParams({query: `${City}, ${State}, ${Country}`,lang: lang});
        const response = await window.__TAURI_PLUGIN_HTTP__.fetch(`https://www.accuweather.com/web-api/autocomplete?${params}`, {method: 'GET',headers: headers});
        if (!response.ok) {return null;}
        const autocomplete = await response.json();
        const firstItem = autocomplete[0];
        if (!firstItem || !firstItem.key) {return null;}
        const key = firstItem.key;
        const keyDecoded = decodeURIComponent(key);
        const paramsRedirect = new URLSearchParams({key: keyDecoded,lang: lang});
        // console.log(`https://www.accuweather.com/web-api/three-day-redirect?${paramsRedirect}`);
        
        const responseRedirect = await window.__TAURI_PLUGIN_HTTP__.fetch(
          `https://www.accuweather.com/web-api/three-day-redirect?${paramsRedirect}`, {method: 'GET',headers: headers,
            redirect: 'follow'
          });
          if (responseRedirect.url == `https://www.accuweather.com/web-api/three-day-redirect?${paramsRedirect}`) {
              return null;
          }
          const url = responseRedirect.url.replace("weather-forecast", "current-weather");
          const main_params = new URLSearchParams({unit: Units === "imperial" ? "f" : "c",lang: lang});
          const Mainresponse = await window.__TAURI_PLUGIN_HTTP__.fetch(`${url}?${main_params}`, {method: 'GET',headers: headers});
          // if (!Mainresponse.ok) {return null;}
          const body = await Mainresponse.text();
          const parser = new DOMParser();
          const document = parser.parseFromString(body, 'text/html');
          // ---------- EXTRAER TEMPERATURA ----------
          const tempElement = document.querySelector('div.display-temp');
          if (!tempElement) return null;
          const temp = tempElement.textContent.trim();
          
          
          // ---------- EXTRAER FRASE ----------
          const phraseElement = document.querySelector('div.phrase');
          if (!phraseElement) return null;
          const phrase = phraseElement.textContent.trim();
          
          
          // ---------- EXTRAER ICONO ----------
          const iconElement = document.querySelector('img.icon');
          
          
          if (!iconElement) return null;
        
          const src = iconElement.getAttribute('src');
          if (!src) return null;

          // Equivalente a: src.split('/').last()...replace(".svg", "")
          const srcParts = src.split('/');
          const lastPart = srcParts[srcParts.length - 1] || "";
          const icon = lastPart.replace(".svg", "");
                   
          // ---------- EXTRAER VIENTO ----------
          // Buscamos todos los elementos 'p.panel-item'
          const panelItems = Array.from(document.querySelectorAll('p.panel-item'));
          
          // Equivalente al .find(|e| e.text().contains("/h"))
          const windPanel = panelItems.find(e => e.textContent.includes('/h'));
          if (!windPanel) return null;

          // Buscamos el 'span.value' dentro de ese panel
          const valueElement = windPanel.querySelector('span.value');
          if (!valueElement) return null;
          const windText = valueElement.textContent.trim();
                    
          // Dividimos por espacios en blanco para separar dirección de velocidad
          const parts = windText.split(/\s+/); // Divide por cualquier cantidad de espacios
          const windDirection = parts[0] || "";
          const windSpeed = parts.slice(1).join(" ");
          Weather_View(temp, phrase, icon, windDirection, windSpeed);
    } catch (error) {return null;}
}





function Weather_View(temp, phrase, icon, wind_direction, wind_speed) {
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