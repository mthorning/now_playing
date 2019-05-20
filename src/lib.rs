use serde::{Deserialize, Serialize};
use std::cell::Cell;

#[derive(Deserialize, Serialize)]
struct TrackData {
    channel: String,
    payload: Payload,
}

#[derive(Deserialize, Serialize)]
struct Payload {
    title: String,
    artist: String,
    album: String,
}

struct NowPlaying {
    title: String,
    artist: String,
}

impl NowPlaying {
    fn from(track_data: TrackData) -> NowPlaying {
        NowPlaying {
            title: track_data.payload.title,
            artist: track_data.payload.artist,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PlayState {
    payload: bool,
}

pub struct Playing {
    pub is_playing: Cell<bool>,
}

impl Playing {
    pub fn new() -> Playing {
        Playing {
            is_playing: Cell::new(false),
        }
    }
    pub fn set_playing(&self, message: String) {
        let play_state: PlayState = serde_json::from_str(&message[..]).unwrap();
        self.is_playing.set(play_state.payload);
    }
    pub fn get_now_playing(message: String, playing: bool) {
        if playing {
            let track_data: TrackData = serde_json::from_str(&message[..]).unwrap();
            let now_playing = NowPlaying::from(track_data);
            println!("{} by {}", now_playing.title, now_playing.artist);
        } else {
            println!("Nothing playing at the moment");
        }
    }
}
