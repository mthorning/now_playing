use os_type;

use now_playing::playback_api::Data;
use now_playing::track::Track;

const LINUX_PATH: &str = ".config/Google Play Music Desktop Player/json_store/playback.json";

const MAC_PATH: &str =
    "Library/Application Support/Google Play Music Desktop Player/json_store/playback.json";

fn main() {
    let json_location = match os_type::current_platform().os_type {
        os_type::OSType::OSX => MAC_PATH,
        _ => LINUX_PATH,
    };
    let data = Data::new(json_location);
    let track = match Track::from(data.contents) {
        Ok(track) => track,
        Err(_) => return println!("Not connected"),
    };

    match track.playing {
        true => println!("{} by {}", track.title, track.artist),
        false => println!("Nothing playing"),
    };
}
