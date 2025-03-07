use std::fs::File;
use std::io::Read;
use std::path::Path;
use hound::{WavReader, SampleFormat};
use rustfft::{FftPlanner, num_complex::Complex};

pub fn analyze_audio(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let reader = WavReader::new(&buffer[..])?;
    let spec = reader.spec();

    let samples: Vec<f32> = match spec.sample_format {
        SampleFormat::Int => reader.into_samples::<i32>()
            .map(|s| s.unwrap() as f32 / i32::MAX as f32)
            .collect(),
        SampleFormat::Float => reader.into_samples::<f32>()
            .map(|s| s.unwrap())
            .collect(),
    };

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    let mut complex_samples: Vec<Complex<f32>> = samples
        .into_iter()
        .map(|s| Complex::new(s, 0.0))
        .collect();

    fft.process(&mut complex_samples);

    // Further analysis can be performed on the complex_samples here

    Ok(())
}