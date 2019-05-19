use serde::{Deserialize, Serialize};
use serde_json::Value;
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

fn get_now_playing(message: String) {
    let track_data: TrackData = serde_json::from_str(&message[..]).unwrap();
    let now_playing = NowPlaying::from(track_data);
    println!("\u{1F3B6}{} by {}", now_playing.title, now_playing.artist);
}

fn main() {
    connect("ws://localhost:5672", |out| {
        out.send("Hello WebSocket").unwrap();

        move |msg: Message| {
            match msg {
                Message::Text(message) => {
                    let data: Value = serde_json::from_str(&message[..]).unwrap();
                    if data["channel"] == "track" {
                        get_now_playing(message);
                    }
                }
                _ => (),
            };

            out.close(CloseCode::Normal)
        }
    })
    .unwrap();
}
