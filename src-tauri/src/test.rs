pub fn analyze_audio(data: &[f32]) -> (f32, f32) {
    let max_amplitude = data.iter().fold(0.0, |acc, &x| acc.max(x));
    let rms = (data.iter().map(|&x| x * x).sum::<f32>() / data.len() as f32).sqrt();
    (max_amplitude, rms)
}