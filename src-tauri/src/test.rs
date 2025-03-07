fn analyze_audio(buffer: &[f32]) -> (f32, f32) {
    let mut sum = 0.0;
    let mut max_amplitude = 0.0;

    for &sample in buffer {
        sum += sample.abs();
        if sample.abs() > max_amplitude {
            max_amplitude = sample.abs();
        }
    }

    let average_amplitude = sum / buffer.len() as f32;
    (average_amplitude, max_amplitude)
}