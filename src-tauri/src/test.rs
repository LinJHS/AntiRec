use std::f32::consts::PI;

pub fn analyze_audio(buffer: &[f32], sample_rate: u32) -> (f32, f32) {
    let rms = calculate_rms(buffer);
    let frequency = estimate_frequency(buffer, sample_rate);
    (rms, frequency)
}

fn calculate_rms(buffer: &[f32]) -> f32 {
    let sum_of_squares: f32 = buffer.iter().map(|&x| x * x).sum();
    (sum_of_squares / buffer.len() as f32).sqrt()
}

fn estimate_frequency(buffer: &[f32], sample_rate: u32) -> f32 {
    let mut max_value = 0.0;
    let mut max_index = 0;

    for (i, &value) in buffer.iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_index = i;
        }
    }

    let frequency = (max_index as f32 * sample_rate as f32) / (2.0 * PI * buffer.len() as f32);
    frequency
}