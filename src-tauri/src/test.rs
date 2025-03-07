use rayon::prelude::*;
use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Duration;

pub fn process_audio(file_path: &str, disturbance_level: f32) -> Result<(), Box<dyn std::error::Error>> {
    // Load the audio file
    let file = File::open(file_path)?;
    let source = Decoder::new(BufReader::new(file))?;

    // Convert the source to a vector of samples for parallel processing
    let samples: Vec<f32> = source.convert_samples().collect();
    let samples = Arc::new(samples);

    // Apply disturbance to the audio samples in parallel
    let disturbed_samples: Vec<f32> = samples
        .par_iter()
        .map(|&sample| {
            let disturbance = (rand::random::<f32>() - 0.5) * 2.0 * disturbance_level;
            sample + disturbance
        })
        .collect();

    // Create a new audio source from the processed samples
    let processed_source = rodio::buffer::SamplesBuffer::new(1, 44100, disturbed_samples);

    // Play the processed audio
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(processed_source);
    sink.sleep_until_end();

    Ok(())
}

pub fn optimize_audio_processing(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load the audio file
    let file = File::open(file_path)?;
    let source = Decoder::new(BufReader::new(file))?;

    // Efficiently downsample the audio by a factor of 2
    let downsampled_samples: Vec<f32> = source
        .convert_samples()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, sample)| sample)
        .collect();

    // Create a new audio source from the downsampled samples
    let downsampled_source = rodio::buffer::SamplesBuffer::new(1, 22050, downsampled_samples);

    // Play the downsampled audio
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(downsampled_source);
    sink.sleep_until_end();

    Ok(())
}