use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;

pub struct Audio {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    music_sink: Sink,
}

impl Audio {
    pub fn new() -> Self {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let music_sink = Sink::try_new(&handle).unwrap();
        Self { _stream, handle, music_sink }
    }

    pub fn play_music(&mut self, file_path: &str) {
        self.music_sink.stop();
        self.music_sink = Sink::try_new(&self.handle).unwrap();

        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file))
            .unwrap()
            .repeat_infinite();

        self.music_sink.append(source);
    }

    pub fn sfx(&self, file_path: &str) {
        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        self.handle.play_raw(source.convert_samples()).unwrap();
    }
}
