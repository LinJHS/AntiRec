use rustfft::{Fft, FftDirection};
use rustfft::num_complex::Complex;

pub fn analyze_audio(samples: &[f32], sample_rate: usize) -> Vec<f32> {
    let mut spectrum = vec![Complex::new(0.0, 0.0); samples.len()];
    for (i, &sample) in samples.iter().enumerate() {
        spectrum[i] = Complex::new(sample, 0.0);
    }

    let fft = Fft::new(samples.len(), FftDirection::Forward);
    fft.process(&mut spectrum);

    let mut magnitudes = Vec::with_capacity(spectrum.len() / 2);
    for i in 0..spectrum.len() / 2 {
        let mag = spectrum[i].norm();
        magnitudes.push(mag);
    }

    magnitudes
}