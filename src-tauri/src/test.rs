use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use hound::{WavReader, WavWriter, SampleFormat};

fn main() {
    let mut reader = WavReader::open("input.wav").unwrap();
    let spec = reader.spec();
    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(reader.samples::<i16>().map(|s| s.unwrap()).collect()));

    // Parallel processing of audio samples
    samples.par_iter_mut().for_each(|sample| {
        // Add random disturbance to each sample
        let disturbance = (rand::random::<i16>() >> 8) as i16;
        *sample = sample.saturating_add(disturbance);
    });

    let writer = WavWriter::create("output.wav", spec).unwrap();
    samples.lock().unwrap().iter().for_each(|&sample| writer.write_sample(sample).unwrap());
}