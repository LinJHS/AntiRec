use std::f32::consts::PI;

/// Generates a sine wave with specified frequency and duration.
pub fn generate_sine_wave(frequency: f32, sample_rate: f32, duration: f32) -> Vec<f32> {
    let num_samples = (sample_rate * duration) as usize;
    (0..num_samples)
        .map(|i| (2.0 * PI * frequency * i as f32 / sample_rate).sin())
        .collect()
}

/// Adds white noise to the audio signal.
pub fn add_white_noise(signal: &mut [f32], noise_level: f32) {
    for sample in signal.iter_mut() {
        let noise = (rand::random::<f32>() - 0.5) * 2.0 * noise_level;
        *sample += noise;
    }
}

/// Applies a low-pass filter to the audio signal.
pub fn apply_low_pass_filter(signal: &mut [f32], cutoff_frequency: f32, sample_rate: f32) {
    let rc = 1.0 / (2.0 * PI * cutoff_frequency);
    let dt = 1.0 / sample_rate;
    let alpha = dt / (rc + dt);

    let mut prev_output = 0.0;
    for sample in signal.iter_mut() {
        *sample = prev_output + alpha * (*sample - prev_output);
        prev_output = *sample;
    }
}

/// Normalizes the audio signal to the range [-1.0, 1.0].
pub fn normalize_signal(signal: &mut [f32]) {
    let max_amplitude = signal
        .iter()
        .fold(0.0, |max, &x| if x.abs() > max { x.abs() } else { max });

    if max_amplitude > 0.0 {
        for sample in signal.iter_mut() {
            *sample /= max_amplitude;
        }
    }
}

/// Processes the audio signal by applying noise and filtering.
pub fn process_audio_signal(signal: &mut [f32], noise_level: f32, cutoff_frequency: f32, sample_rate: f32) {
    add_white_noise(signal, noise_level);
    apply_low_pass_filter(signal, cutoff_frequency, sample_rate);
    normalize_signal(signal);
}