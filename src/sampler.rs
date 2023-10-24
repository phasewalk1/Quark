use rodio::cpal::Sample as CpalSample;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sample, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct QuickSampler {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    pub sources: HashMap<char, PathBuf>,
    waveform_data: Arc<Mutex<Vec<f32>>>,
}

impl QuickSampler {
    pub fn new() -> QuickSampler {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sources = HashMap::new();
        let waveform_data = Arc::new(Mutex::new(Vec::<f32>::new()));
        QuickSampler {
            stream,
            stream_handle,
            sources,
            waveform_data,
        }
    }

    pub fn add_source(&mut self, key: char, path: PathBuf) {
        self.sources.insert(key, path);
    }

    pub fn play_source(&mut self, key: char) {
        if let Some(path) = self.sources.get(&key) {
            let file = BufReader::new(File::open(path).unwrap());
            let source = Decoder::new(file).unwrap();

            let waveform_source = WaveformSource::new(source, self.waveform_data.clone());

            let _ = self
                .stream_handle
                .play_raw(waveform_source.convert_samples());
        }
    }

    pub fn get_waveform_data(&self) -> Vec<f32> {
        let waveform_data = self.waveform_data.lock().unwrap();
        waveform_data.clone()
    }
}

pub struct WaveformSource<S>
where
    S: Source,
    S::Item: Sample,
{
    input: S,
    waveform_data: Arc<Mutex<Vec<f32>>>,
}

impl<S> WaveformSource<S>
where
    S: Source,
    S::Item: Sample<Float = f32>,
{
    pub fn new(input: S, waveform_data: Arc<Mutex<Vec<f32>>>) -> Self {
        Self {
            input,
            waveform_data,
        }
    }
}

impl<S> Iterator for WaveformSource<S>
where
    S: Source,
    S::Item: Sample<Float = f32>,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.input.next()?;
        let float_sample: f32 = sample.to_float_sample();
        let mut waveform_data = self.waveform_data.lock().unwrap();
        waveform_data.push(float_sample);
        drop(waveform_data);
        Some(sample)
    }
}

impl<S> Source for WaveformSource<S>
where
    S: Source,
    S::Item: Sample<Float = f32>,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.input.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.input.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.input.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.input.total_duration()
    }
}
