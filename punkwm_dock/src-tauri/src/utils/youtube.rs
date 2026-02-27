pub struct YTMusic;
use crate::IpcFrontMessage;
use crate::apphandle::get_api_ipc;
use crate::utils::cookies::{serialize_cookies, parse_cookies, CookieSearch,};
use reqwest::header::HeaderValue;
use reqwest::header::HeaderMap;
use sha1::Sha1;
use sha1::Digest;
use serde_json::Value;
use std::time::UNIX_EPOCH;
use serde::Serialize;
use serde::Deserialize;
use tauri::Manager;
use serde_json::json;
use std::time::SystemTime;
use tauri::WebviewUrl;
use crate::WebviewWindowBuilder;
use std::fs;

impl YTMusic{
    pub fn quick_picks(msg: serde_json::Value){
        tokio::spawn({
                let msg_data = msg.clone();
                async move {
                    let headers = YTMusic::get_headers(msg_data.clone());
                    let body = json!({"context": {"client": {"clientName": "WEB_REMIX","clientVersion": "1.20260209.03.00"}},"browseId": "FEmusic_home"});
                    let client = reqwest::Client::new();
                    let response = match client.post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false").headers(headers).json(&body).send().await{
                        Ok(resp) => resp,
                        Err(e) => {eprintln!("Error haciendo request: {}", e);return;}
                    };

                    if response.status().is_success() {
                        let json: serde_json::Value = match response.json().await {
                            Ok(t) => t,
                            Err(e) => {
                                eprintln!("Error leyendo respuesta: {}", e);
                                return;
                            }
                        };

                        let mut tracks: Vec<YTSong> = Vec::new();
                        if let Some(songs) = json["contents"]["singleColumnBrowseResultsRenderer"]["tabs"][0]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["musicCarouselShelfRenderer"]["contents"].as_array(){
                            for item in songs {
                                let renderer = &item["musicResponsiveListItemRenderer"];
                                let cover = renderer["thumbnail"]["musicThumbnailRenderer"]["thumbnail"]["thumbnails"].as_array().and_then(|arr| arr.last()).and_then(|thumb| thumb["url"].as_str()).unwrap_or("").to_string();
                                let id = renderer["playlistItemData"]["videoId"].as_str().unwrap_or("").to_string();
                                let title = renderer["flexColumns"][0]["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"][0]["text"].as_str().unwrap_or("").to_string();
                                let artist = renderer["flexColumns"][1]["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"][0]["text"].as_str().unwrap_or("").to_string();
                                let album = renderer["flexColumns"][2]["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"][0]["text"].as_str().unwrap_or("").to_string();
                                let song = YTSong{id,cover,title,album,artist,mode: "YT-Music".to_string(),duration: None,};
                                tracks.push(song);
                            }

                        }
                        let api = get_api_ipc();
                        let msg: IpcFrontMessage = IpcFrontMessage{category: "Panel:Music".to_string(),name: "YT:Load Quik Picks".to_string(),data: json!({"songs": tracks})};
                        let _ = api.emit(msg.clone());
                    }
                }
            });
    }

    pub fn next_songs(msg: serde_json::Value){
        tokio::spawn({
            let msg_data = msg.clone(); // clonar para mover al async
            async move {
                let headers = YTMusic::get_headers(msg_data.clone());
                let song_id = msg_data["songid"].to_string().trim_matches('"').to_string();
                let body = json!({"videoId": song_id,"playlistId": format!("RDAMVM{}", song_id),"isAudioOnly": true,"context": {"client": {"clientName": "WEB_REMIX","clientVersion": "1.20260209.03.00"}}                });
                let client = reqwest::Client::new();
                let response = match client.post("https://music.youtube.com/youtubei/v1/next?prettyPrint=false").headers(headers).json(&body).send().await{
                    Ok(resp) => resp,
                    Err(e) => {
                        eprintln!("Error haciendo request: {}", e);
                        return;
                    }
                };
                if response.status().is_success() {
                    let json: serde_json::Value = match response.json().await {
                        Ok(t) => t,
                        Err(e) => {
                            eprintln!("Error leyendo respuesta: {}", e);
                            return;
                        }
                    };
                    let mut tracks: Vec<YTSong> = Vec::new();
                    if let Some(songs) = json["contents"]["singleColumnMusicWatchNextResultsRenderer"]["tabbedRenderer"]["watchNextTabbedResultsRenderer"]["tabs"][0]["tabRenderer"]["content"]["musicQueueRenderer"]["content"]["playlistPanelRenderer"]["contents"].as_array(){
                        for item in songs {
                            let renderer = &item["playlistPanelVideoRenderer"];
                            let cover = renderer["thumbnail"]["thumbnails"].as_array().and_then(|arr| arr.last()).and_then(|thumb| thumb["url"].as_str()).unwrap_or("").to_string();                              
                            let id = renderer["videoId"].as_str().unwrap_or("").to_string();
                            let title = renderer["title"]["runs"][0]["text"].as_str().unwrap_or("").to_string();
                            let duration = renderer["lengthText"]["runs"][0]["text"].as_str().unwrap_or("").to_string();
                            let unit_text = renderer["longBylineText"]["runs"].as_array().unwrap_or(&vec![]).iter().filter_map(|item| item["text"].as_str()).collect::<String>();
                            let split_text: Vec<&str> = unit_text.split(" • ").map(|s| s.trim()).collect();
                            let artist = split_text[0].to_string();
                            let album = split_text[1].to_string();
                            let song = YTSong{id,cover,title,album,artist,mode: "YT-Music".to_string(),duration: Some(duration),};
                            tracks.push(song);
                        }
                    }
                    let api = get_api_ipc();
                    let msg: IpcFrontMessage = IpcFrontMessage{
                        category: "Panel:Music".to_string(),
                        name: "YT:Load Next Songs".to_string(),
                        data: json!({"songs": tracks})
                    };
                    let _ = api.emit(msg.clone());
        

                }

            }
        });
    }

    pub fn search(msg: serde_json::Value){
        tokio::spawn({
            let msg_data = msg.clone(); // clonar para mover al async
            async move {
                let headers = YTMusic::get_headers(msg_data.clone());
                let q = msg_data["q"].to_string().trim_matches('"').to_string();
                let body = json!({"query": q,"inlineSettingStatus": "INLINE_SETTING_STATUS_ON","params": "EgWKAQIIAWoSEAkQBBADEBAQBRAVEAoQDhAR","context": {"client": {"clientName": "WEB_REMIX","clientVersion": "1.20260209.03.00"}}});
                let client = reqwest::Client::new();
                let response = match client.post("https://music.youtube.com/youtubei/v1/search?prettyPrint=false").headers(headers).json(&body).send().await{
                    Ok(resp) => resp,
                    Err(e) => {
                        eprintln!("Error haciendo request: {}", e);
                        return;
                    }
                };
                if response.status().is_success() {
                    let json: serde_json::Value = match response.json().await {
                        Ok(t) => t,
                        Err(e) => {
                            eprintln!("Error leyendo respuesta: {}", e);
                            return;
                        }
                    };
                    let mut tracks: Vec<YTSong> = Vec::new();
                    if let Some(songs) = json["contents"]["tabbedSearchResultsRenderer"]["tabs"][0]["tabRenderer"]["content"]["sectionListRenderer"]["contents"][0]["musicShelfRenderer"]["contents"].as_array(){
                        for item in songs {
                            let renderer = &item["musicResponsiveListItemRenderer"];
                            let id = renderer["playlistItemData"]["videoId"].as_str().unwrap_or("").to_string();
                            let cover = renderer["thumbnail"]["musicThumbnailRenderer"]["thumbnail"]["thumbnails"].as_array().and_then(|arr| arr.last()).and_then(|thumb| thumb["url"].as_str()).unwrap_or("").to_string();                              
                            let title = renderer["flexColumns"][0]["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"][0]["text"].as_str().unwrap_or("").to_string();
                            let unit_text = renderer["flexColumns"][1]["musicResponsiveListItemFlexColumnRenderer"]["text"]["runs"].as_array().unwrap_or(&vec![]).iter().filter_map(|item| item["text"].as_str()).collect::<String>();
                            let split_text: Vec<&str> = unit_text.split(" • ").map(|s| s.trim()).collect();
                            let artist = split_text[0].to_string();
                            let album = split_text[1].to_string();
                            let duration = split_text[2].to_string();
                            let song = YTSong{id,cover,title,album,artist,mode: "YT-Music".to_string(),duration: Some(duration),};
                            tracks.push(song);
                        }
                    }
                    let api = get_api_ipc();
                    let msg: IpcFrontMessage = IpcFrontMessage{
                        category: "Panel:Music".to_string(),
                        name: "YT:Load Search".to_string(),
                        data: json!({"songs": tracks})
                    };
                    let _ = api.emit(msg.clone());   
                }
            }
        });
    }

    pub fn start_song(msg: serde_json::Value){
        let api = get_api_ipc();
        let song_id = msg["songid"].to_string().trim_matches('"').to_string();
        let url = format!("https://music.youtube.com/watch?v={}", song_id);
        if let Some(window) = api.app_handle.get_webview_window("yt_music_player") {
            window.navigate(url.parse().unwrap()).unwrap();
        }else{   
            let _ = WebviewWindowBuilder::new(&api.app_handle,"yt_music_player",
            WebviewUrl::External(url.parse().unwrap()),)
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .initialization_script("
                    const { event: TAURI_EVENT } = window.__TAURI__;
                    window.addEventListener('load', () => {
                        console.log('hola');
                        let video = document.querySelector('.video-stream');
                        console.log(video);
                        let lastSecond = -1;
                        video.addEventListener('timeupdate', async () => {
                            const currentSecond = Math.floor(video.currentTime);
                            if (currentSecond !== lastSecond) {
                                console.log(currentSecond);
                                lastSecond = currentSecond;
                                await TAURI_EVENT.emit('ipc', { 'category': 'Panel:Music', 'name': 'YT:Load Current Time', 'data': { 'time': currentSecond } });
                            }
                        });
                    });
                ").title("____").visible(true).build();
        }
    }
        
    pub fn play(){
        let api = get_api_ipc();
        if let Some(window) = api.app_handle.get_webview_window("yt_music_player") {
            let _ = window.eval("document.querySelector('video').play()").unwrap();
        }
    }

    pub fn pause(){
        let api = get_api_ipc();
        if let Some(window) = api.app_handle.get_webview_window("yt_music_player") {
            let _ = window.eval("document.querySelector('video').pause()").unwrap();
        }
    }
    
    pub fn status(){
        let api = get_api_ipc();
        let mut status = "OFF";
        if let Some(_window) = api.app_handle.get_webview_window("yt_music_player") {
            status = "ON";
        }
        let msg: IpcFrontMessage = IpcFrontMessage{
            category: "Panel:Music".to_string(),
            name: "YT:ViewStatus".to_string(),
            data: json!({"status": status})
        };
        let _ = api.emit(msg.clone());
    }

    pub fn stop(){
        let api = get_api_ipc();
        if let Some(window) = api.app_handle.get_webview_window("yt_music_player") {
            let _ = window.close();
        }
    }

    pub fn start(){
        let api = get_api_ipc();
        let path = api.app_handle.path().app_data_dir().unwrap().join("cookies");
        let cookies_data = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => "".to_string(),
        };
        let cookies = parse_cookies(&cookies_data);
        let cookies_vec = cookies.search_many(".youtube.com",
            vec!["HSID", "APISID", "SAPISID", "SID", "SSID", "__Secure-1PSIDTS", "__Secure-ROLLOUT_TOKEN", "_gcl_au"]
        );
        if cookies_vec.len() == 8{
            let combined = serialize_cookies(&cookies_vec.iter().map(|c| (*c).clone()).collect::<Vec<_>>());
            let msg: IpcFrontMessage = IpcFrontMessage{
                category: "Panel:Music".to_string(),
                name: "YT:Set Cookies".to_string(),
                data: serde_json::to_value(&combined).unwrap()
            };
            let _ = api.emit(msg.clone());           
        }else{
            let _ = WebviewWindowBuilder::new(&api.app_handle,"yt_music_load_data",WebviewUrl::App("ytm.html".into()),)
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .title("__")
                .visible(false)
                .on_navigation(move |url : &tauri::Url| {
                    let url_clon = url.clone();
                    if url_clon.to_string().contains("about:blank"){
                        if let Some(window) = api.app_handle.get_webview_window("yt_music_load_data") {
                            window.show().unwrap();
                        }                     
                    }
                    if url_clon.to_string().contains("https://music.youtube.com"){
                        if let Some(window) = api.app_handle.get_webview_window("yt_music_load_data") {
                            let path = api.app_handle.path().app_data_dir().unwrap().join("cookies");
                            let cookies_data = match fs::read_to_string(path) {
                                Ok(c) => c,
                                Err(_) => return false,
                            };
                            let cookies = parse_cookies(&cookies_data);
                            let cookies_vec = cookies.search_many(".youtube.com",
                                vec!["HSID", "APISID", "SAPISID", "SID", "SSID", "__Secure-1PSIDTS", "__Secure-ROLLOUT_TOKEN", "_gcl_au"]
                            );
                            let combined = serialize_cookies(&cookies_vec.iter().map(|c| (*c).clone()).collect::<Vec<_>>());
                            let msg: IpcFrontMessage = IpcFrontMessage{
                                category: "Panel:Music".to_string(),
                                name: "YT:Set Cookies".to_string(),
                                data: serde_json::to_value(&combined).unwrap()
                            };
                            let _ = api.emit(msg.clone());
                            let _ = window.close();
                            return true;
                        }                   
                    }
                    true 
                }).build();
            }
    }

    fn get_headers(msg_data: serde_json::Value)->  HeaderMap{
        let sapisid = msg_data["cookies"]["SAPISID"].as_str().unwrap_or("");                  
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(t) => t.as_secs().to_string(),
            Err(_) => {
                eprintln!("Error obteniendo timestamp");
                return Default::default();
                }
            };

        let input_string = format!("{} {} https://music.youtube.com", timestamp, sapisid);
        let mut hasher = Sha1::new();
        hasher.update(input_string.as_bytes());
        let hash_hex = format!("{:x}", hasher.finalize());
        let authorization = format!("SAPISIDHASH {}_{}", timestamp, hash_hex);
        let cookies_str = msg_data.get("cookies").map(|v| serde_json::to_string(v).unwrap()).unwrap_or_else(|| "".to_string());
        let cookie_header = YTMusic::json_to_cookie_header(&cookies_str);
        let mut headers = HeaderMap::new();
        if let Ok(h) = HeaderValue::from_str(&authorization) {headers.insert("authorization", h);}
        headers.insert("x-origin", HeaderValue::from_static("https://music.youtube.com"));
        if let Ok(h) = HeaderValue::from_str(&cookie_header) {headers.insert("cookie", h);}
        return headers;

    }

    fn json_to_cookie_header(json_str: &str) -> String {
        let parsed: Value = match serde_json::from_str(json_str) {
            Ok(v) => v,
            Err(_) => return String::new(),
        };
        let mut cookies = Vec::new();
        if let Some(obj) = parsed.as_object() {
            for (key, value) in obj {
                if let Some(val_str) = value.as_str() {cookies.push(format!("{}={}", key, val_str));}
            }
        }
        cookies.join("; ")
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YTSong{
    pub id: String,
    pub cover: String,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub mode: String,
    pub duration: Option<String>,
}