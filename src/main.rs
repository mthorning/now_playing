use serde_json::Value;
use ws::{connect, CloseCode, Error, Handler, Message, Result, Sender};

use now_playing::Playing;

struct Client {
    out: Sender,
    playing: Playing,
}

impl Handler for Client {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(message) => {
                let data: Value = serde_json::from_str(&message[..]).unwrap();
                if data["channel"] == "playState" {
                    self.playing.set_playing(message);
                } else if data["channel"] == "track" {
                    Playing::get_now_playing(message, self.playing.is_playing.get());
                }
            }
            _ => println!(),
        };

        self.out.close(CloseCode::Normal)
    }
    fn on_error(&mut self, _err: Error) {
        println!("\u{1F3B6}GPMDP isn't running");
    }
}

fn main() {
    connect("ws://localhost:5672", |out| {
        let playing = Playing::new();
        Client { out, playing }
    })
    .unwrap();
}
