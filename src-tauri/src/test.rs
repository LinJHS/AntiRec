use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

pub fn analyze_audio(samples: &[f32], sample_rate: u32) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    let mut buffer: Vec<Complex<f32>> = samples.iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    fft.process(&mut buffer);

    let spectrum: Vec<f32> = buffer.iter()
        .map(|c| c.norm())
        .collect();

    spectrum
}