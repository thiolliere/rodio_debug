extern crate rodio;
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use rodio::Source;

fn main() {
    let endpoint = rodio::get_default_endpoint().unwrap();


    // this sound is correct
    let source = rodio::Decoder::new(File::open("audio.ogg").unwrap()).unwrap();
    rodio::play_raw(&endpoint, source.convert_samples());
    sleep(Duration::from_secs(1));

    // this sound is not, the beginning is lower than it should
    let source = rodio::Decoder::new(File::open("audio.ogg").unwrap()).unwrap();
    let sink = rodio::Sink::new(&endpoint);
    sink.append(source);
    sink.detach();

    sleep(Duration::from_secs(10));
}
