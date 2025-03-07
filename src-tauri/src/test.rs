use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

pub fn analyze_audio(samples: &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    let mut buffer: Vec<Complex<f32>> = samples.iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    fft.process(&mut buffer);

    buffer.iter()
        .map(|c| c.norm())
        .collect()
}