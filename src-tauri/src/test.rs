use rustfft::num_complex::Complex;
use rustfft::FFTplanner;
use std::f32::consts::PI;

pub fn analyse_audio(samples: &[f32], sample_rate: u32) -> Vec<(f32, f32)> {
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(samples.len());

    let mut buffer: Vec<Complex<f32>> = samples.iter().map(|&x| Complex::new(x, 0.0)).collect();
    let mut spectrum = vec![Complex::new(0.0, 0.0); samples.len()];

    fft.process(&mut buffer, &mut spectrum);

    let bin_width = sample_rate as f32 / samples.len() as f32;
    let mut result = Vec::new();

    for (i, &bin) in spectrum.iter().enumerate() {
        let freq = i as f32 * bin_width;
        let magnitude = bin.norm();
        result.push((freq, magnitude));
    }

    result
}