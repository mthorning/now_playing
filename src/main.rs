use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::Cell;
use ws::Message;
use ws::{connect, CloseCode};

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

struct Playing {
    is_playing: Cell<bool>,
}

impl Playing {
    fn new() -> Playing {
        Playing {
            is_playing: Cell::new(false),
        }
    }
    fn set_playing(&self, message: String) {
        let play_state: PlayState = serde_json::from_str(&message[..]).unwrap();
        self.is_playing.set(play_state.payload);
    }
}

fn get_now_playing(message: String, playing: bool) {
    if playing {
        let track_data: TrackData = serde_json::from_str(&message[..]).unwrap();
        let now_playing = NowPlaying::from(track_data);
        println!("\u{1F3B6}{} by {}", now_playing.title, now_playing.artist);
    } else {
        println!("\u{1F3B6}The Sound of Silence");
    }
}

fn main() {
    connect("ws://localhost:5672", |out| {
        let playing = Playing::new();
        move |msg: Message| {
            match msg {
                Message::Text(message) => {
                    let data: Value = serde_json::from_str(&message[..]).unwrap();
                    if data["channel"] == "playState" {
                        playing.set_playing(message);
                    } else if data["channel"] == "track" {
                        get_now_playing(message, playing.is_playing.get());
                    }
                }
                _ => (),
            };

            out.close(CloseCode::Normal)
        }
    })
    .unwrap();
}
