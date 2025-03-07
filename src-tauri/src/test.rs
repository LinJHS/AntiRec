use rustfft::{Fft, FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

pub fn analyze_audio(samples: &[f32], sample_rate: usize) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());
    
    let mut spectrum: Vec<Complex<f32>> = samples.iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    
    fft.process(&mut spectrum);

    let magnitudes: Vec<f32> = spectrum.iter()
        .map(|c| c.norm())
        .collect();

    let freq_bins: Vec<f32> = (0..magnitudes.len()).map(|i| {
        i as f32 * sample_rate as f32 / samples.len() as f32
    }).collect();

    magnitudes
}