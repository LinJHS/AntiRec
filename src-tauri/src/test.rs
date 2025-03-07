fn analyze_audio(audio_data: &[f32]) -> f32 {
    let sum: f32 = audio_data.iter().map(|&x| x * x).sum();
    let rms = (sum / audio_data.len() as f32).sqrt();
    rms
}