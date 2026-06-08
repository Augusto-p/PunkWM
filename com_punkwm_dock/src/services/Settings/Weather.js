class Weather extends Sender {
    static async SaveUnits(value){super.Emit("Weather", "Save Units", {"Mode": value ? "metric": "imperial"});}
    static async SaveLocation(City, State, Country){super.Emit("Weather", "Save Location", {"City": City, "State": State, "Country": Country});}
    static Load(Units, City, State, Country){
        Storage.setItem("Weather-Units", Units == "metric");
        LoadWeatherUnits();
        fetch(`https://nominatim.openstreetmap.org/search?format=json&q=${encodeURIComponent(`${City}, ${State}, ${Country}`)}`, {headers: {'User-Agent': 'PunkWM'}}).then(res=>{
            res.json().then(data=>{
                if (data.length > 0) {
                    const { lat, lon } = data[0];
                    Storage.setItem("Weather-Lat", lat);
                    Storage.setItem("Weather-Lon", lon);
                    LoadMap();
                }
            })            
        })
        
    }
    
}

