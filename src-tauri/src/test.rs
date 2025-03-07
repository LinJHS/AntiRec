pub fn analyze_audio(buffer: &[f32]) -> (f32, f32) {
    let sum: f32 = buffer.iter().sum();
    let mean = sum / buffer.len() as f32;

    let variance: f32 = buffer.iter()
        .map(|sample| {
            let diff = sample - mean;
            diff * diff
        })
        .sum::<f32>() / buffer.len() as f32;

    (mean, variance.sqrt())
}