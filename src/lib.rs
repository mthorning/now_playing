pub mod track {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct TrackData {
        playing: bool,
        song: Song,
    }

    #[derive(Deserialize, Serialize)]
    struct Song {
        title: String,
        artist: String,
    }

    pub struct Track {
        pub title: String,
        pub artist: String,
        pub playing: bool,
    }

    impl Track {
        pub fn from(track_json: String) -> Result<Track, ()> {
            let track_data: TrackData = match serde_json::from_str(&track_json[..]) {
                Ok(json) => json,
                Err(_) => return Err(()),
            };
            Ok(Track {
                title: track_data.song.title,
                artist: track_data.song.artist,
                playing: track_data.playing,
            })
        }
    }
}

pub mod playback_api {
    use dirs;
    use std::fs::File;
    use std::io::Read;

    pub struct Data {
        pub contents: String,
    }

    impl Data {
        pub fn new(json_location: &str) -> Data {
            let home = dirs::home_dir().unwrap();
            let path = home.join(json_location);
            let mut file = File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Data { contents }
        }
    }
}
