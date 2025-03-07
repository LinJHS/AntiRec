use std::f32::consts::PI;

pub fn analyze_audio(samples: &[f32], sample_rate: u32) -> (f32, f32) {
    let mut rms = 0.0;
    let mut spectral_centroid = 0.0;
    let mut total_magnitude = 0.0;

    // Calculate RMS (Root Mean Square) for amplitude analysis
    for &sample in samples {
        rms += sample * sample;
    }
    rms = (rms / samples.len() as f32).sqrt();

    // Calculate Spectral Centroid for frequency analysis
    let fft_size = samples.len();
    let mut fft_input = vec![];
    for &sample in samples {
        fft_input.push(sample);
    }

    let fft_output = fft(&fft_input);

    for (i, &bin) in fft_output.iter().enumerate() {
        let frequency = i as f32 * sample_rate as f32 / fft_size as f32;
        let magnitude = bin.norm();
        spectral_centroid += frequency * magnitude;
        total_magnitude += magnitude;
    }

    if total_magnitude > 0.0 {
        spectral_centroid /= total_magnitude;
    }

    (rms, spectral_centroid)
}

fn fft(input: &[f32]) -> Vec<num_complex::Complex<f32>> {
    let mut output = vec![];
    for &sample in input {
        output.push(num_complex::Complex::new(sample, 0.0));
    }
    output
}