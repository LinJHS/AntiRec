pub fn analyze_audio(audio_data: &[f32]) -> (f32, f32) {
    let sum: f32 = audio_data.iter().sum();
    let mean = sum / audio_data.len() as f32;
    let variance: f32 = audio_data.iter().map(|&x| (x - mean).powi(2)).sum() / audio_data.len() as f32;
    (mean, variance.sqrt())
}