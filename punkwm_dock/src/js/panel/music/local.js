Local_Music_Player.addEventListener("timeupdate", (e)=>{
    Song_Time_Update(Math.floor(Local_Music_Player.currentTime));
    
})