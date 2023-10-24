use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct DrumMachine {
    stream_handle: OutputStreamHandle,
    sources: HashMap<char, PathBuf>,
}

impl DrumMachine {
    pub fn new() -> DrumMachine {
        let (_, stream_handle) = OutputStream::try_default().unwrap();
        let sources = HashMap::new();
        DrumMachine {
            stream_handle,
            sources,
        }
    }

    pub fn add_source(&mut self, key: char, path: PathBuf) {
        self.sources.insert(key, path);
    }

    pub fn play_source(&self, key: char) {
        if let Some(path) = self.sources.get(&key) {
            let file = BufReader::new(File::open(path).unwrap());
            let source = Decoder::new(file).unwrap();
            let _ = self.stream_handle.play_raw(source.convert_samples());
        }
    }
}
