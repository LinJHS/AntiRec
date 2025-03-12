use std::f32::consts::PI;

/// Applies a low-pass filter to the audio signal
pub fn low_pass_filter(signal: &[f32], cutoff_freq: f32, sample_rate: f32) -> Vec<f32> {
    let rc = 1.0 / (2.0 * PI * cutoff_freq);
    let dt = 1.0 / sample_rate;
    let alpha = dt / (rc + dt);

    let mut filtered_signal = Vec::with_capacity(signal.len());
    let mut prev_value = 0.0;

    for &value in signal {
        prev_value = alpha * value + (1.0 - alpha) * prev_value;
        filtered_signal.push(prev_value);
    }

    filtered_signal
}

/// Adds white noise to the audio signal
pub fn add_white_noise(signal: &[f32], noise_level: f32) -> Vec<f32> {
    signal.iter()
        .map(|&sample| sample + (rand::random::<f32>() * 2.0 - 1.0) * noise_level)
        .collect()
}

/// Efficiently mixes two audio signals
pub fn mix_signals(signal1: &[f32], signal2: &[f32]) -> Vec<f32> {
    signal1.iter()
        .zip(signal2.iter())
        .map(|(&s1, &s2)| (s1 + s2).clamp(-1.0, 1.0))
        .collect()
}

/// Normalizes the audio signal to prevent clipping
pub fn normalize_signal(signal: &mut [f32]) {
    let max_amplitude = signal.iter()
        .fold(0.0, |max, &sample| max.max(sample.abs()));

    if max_amplitude > 0.0 {
        let scale_factor = 1.0 / max_amplitude;
        for sample in signal {
            *sample *= scale_factor;
        }
    }
}