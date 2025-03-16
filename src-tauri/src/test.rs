use std::f32::consts::PI;

/// Applies a low-pass filter to the audio signal to reduce high-frequency noise.
pub fn low_pass_filter(signal: &mut [f32], cutoff_freq: f32, sample_rate: f32) {
    let rc = 1.0 / (2.0 * PI * cutoff_freq);
    let dt = 1.0 / sample_rate;
    let alpha = dt / (rc + dt);

    for i in 1..signal.len() {
        signal[i] = signal[i - 1] + alpha * (signal[i] - signal[i - 1]);
    }
}

/// Adds white noise to the audio signal with a specified amplitude.
pub fn add_white_noise(signal: &mut [f32], noise_amplitude: f32) {
    for sample in signal.iter_mut() {
        let noise: f32 = (rand::random::<f32>() * 2.0 - 1.0) * noise_amplitude;
        *sample += noise;
    }
}

/// Efficiently normalizes the audio signal to the range [-1.0, 1.0].
pub fn normalize_signal(signal: &mut [f32]) {
    let max_amplitude = signal.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
    if max_amplitude > 0.0 {
        let scale = 1.0 / max_amplitude;
        for sample in signal.iter_mut() {
            *sample *= scale;
        }
    }
}

/// Processes the audio signal with a combination of filtering, noise addition, and normalization.
pub fn process_audio_signal(signal: &mut [f32], cutoff_freq: f32, sample_rate: f32, noise_amplitude: f32) {
    low_pass_filter(signal, cutoff_freq, sample_rate);
    add_white_noise(signal, noise_amplitude);
    normalize_signal(signal);
}