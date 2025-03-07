use std::f32::consts::PI;

pub fn analyze_audio(samples: &[f32], sample_rate: u32) -> (f32, f32) {
    let mut rms = 0.0;
    let mut spectral_centroid = 0.0;
    let mut total_magnitude = 0.0;

    for &sample in samples {
        rms += sample * sample;
    }
    rms = (rms / samples.len() as f32).sqrt();

    let fft_size = samples.len();
    let mut fft_output = vec![0.0; fft_size];
    let mut fft_input: Vec<_> = samples.iter().map(|&x| x as f64).collect();

    // Perform FFT
    let mut planner = rustfft::FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);
    fft.process(&mut fft_input);

    for (i, &bin) in fft_input.iter().enumerate() {
        let magnitude = bin.abs() as f32;
        let frequency = i as f32 * sample_rate as f32 / fft_size as f32;
        spectral_centroid += frequency * magnitude;
        total_magnitude += magnitude;
    }

    if total_magnitude > 0.0 {
        spectral_centroid /= total_magnitude;
    }

    (rms, spectral_centroid)
}