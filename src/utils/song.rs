
use std::fmt;
use base64::{engine::general_purpose, Engine};
use audiotags::{Tag, MimeType};
use lofty::{probe::Probe, file::AudioFile};
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Song{
    pub id: String,
    pub cover: Option<String>,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub mode: String,
    pub duration: String,
}

impl Song{

    pub fn from_path(path:String)-> Option<Self>{
        let tag = Tag::new().read_from_path(&path).unwrap();
        let title = tag.title()?;
        let artist = tag.artist()?;
        let album = tag.album()?;
        let duration = Self::get_duration(path.clone())?;
        let mut cover: Option<String> = None;
        if let Some(picture) = tag.album_cover() {
            let base64_image = general_purpose::STANDARD.encode(picture.data);
            let mime = MyMime(picture.mime_type);
            cover = Some(format!("data:{};base64,{}", mime, base64_image));        
        }
        Some(Self{
            id: path,
            title: title.to_string(),
            album: album.title.to_string(),
            artist: artist.to_string(),
            cover: cover,
            duration: duration,
            mode: "Local".to_string()
        })  

    }

    fn get_duration(path:String) -> Option<String>{
        let tagged_file = Probe::open(&path).ok()?.read().ok()?;
        let properties = tagged_file.properties();
        let duration = properties.duration();
        let minutes = duration.as_secs() / 60;
        let seconds = duration.as_secs() % 60;
        Some(format!("{}:{:02}", minutes, seconds))
    }

   pub fn search_songs(songs: &[Self], query: &str) -> Vec<Self> {
    let q = query.trim().to_lowercase();

    if q.is_empty() {
        return Vec::new();
    }

    let mut results: Vec<(Self, u8)> = songs
        .iter()
        .filter_map(|song| {
            let title = song.title.to_lowercase();
            let artist = song.artist.to_lowercase();
            let album = song.album.to_lowercase();

            if title.contains(&q) {
                Some((song.clone(), 3))
            } else if artist.contains(&q) {
                Some((song.clone(), 2))
            } else if album.contains(&q) {
                Some((song.clone(), 1))
            } else {
                None
            }
        })
        .collect();

    results.sort_by(|a, b| b.1.cmp(&a.1));

    results.into_iter().map(|(song, _)| song).collect()
}
}

pub struct MyMime(pub MimeType);
impl fmt::Display for MyMime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mime_str = match self.0 {
            MimeType::Png => "image/png",
            MimeType::Jpeg => "image/jpeg",
            MimeType::Tiff => "image/tiff",
            MimeType::Bmp => "image/bmp",
            MimeType::Gif => "image/gif",
        };

        write!(f, "{}", mime_str)
    }
}

impl PartialEq for Song {
    fn eq(&self, other: &Self) -> bool {
        self.cover == other.cover
            && self.title == other.title
            && self.album == other.album
            && self.artist == other.artist
            && self.mode == other.mode
            && self.duration == other.duration
    }
}