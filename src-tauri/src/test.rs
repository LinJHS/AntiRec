use std::f32::consts::PI;
use ndarray::{Array1, Array2};
use rayon::prelude::*;

fn apply_disturbance(audio: &Array1<f32>, noise_level: f32) -> Array1<f32> {
    let noise: Array1<f32> = Array1::random(audio.len(), rand::distributions::Uniform::new(-1.0, 1.0)) * noise_level;
    audio + noise
}

fn normalize_audio(audio: &Array1<f32>) -> Array1<f32> {
    let max_amplitude = audio.fold(f32::NEG_INFINITY, |acc, &x| acc.max(x.abs()));
    audio / max_amplitude
}

fn fft_transform(audio: &Array1<f32>) -> Array1<f32> {
    let n = audio.len();
    let mut spectrum = Array1::zeros(n);
    for k in 0..n {
        let mut sum_real = 0.0;
        let mut sum_imag = 0.0;
        for t in 0..n {
            let angle = 2.0 * PI * (k as f32) * (t as f32) / (n as f32);
            sum_real += audio[t] * angle.cos();
            sum_imag -= audio[t] * angle.sin();
        }
        spectrum[k] = (sum_real.powi(2) + sum_imag.powi(2)).sqrt();
    }
    spectrum
}

fn parallel_fft(audio: &Array1<f32>) -> Array1<f32> {
    let n = audio.len();
    let spectrum: Vec<f32> = (0..n).into_par_iter().map(|k| {
        let mut sum_real = 0.0;
        let mut sum_imag = 0.0;
        for t in 0..n {
            let angle = 2.0 * PI * (k as f32) * (t as f32) / (n as f32);
            sum_real += audio[t] * angle.cos();
            sum_imag -= audio[t] * angle.sin();
        }
        (sum_real.powi(2) + sum_imag.powi(2)).sqrt()
    }).collect();
    Array1::from(spectrum)
}

fn main() {
    let audio = Array1::linspace(0.0, 1.0, 1024);
    let disturbed_audio = apply_disturbance(&audio, 0.1);
    let normalized_audio = normalize_audio(&disturbed_audio);
    let spectrum = parallel_fft(&normalized_audio);
    println!("{:?}", spectrum);
}