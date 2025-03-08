use std::f32::consts::PI;

/// A simple audio processing module for adding noise and improving efficiency
mod audio_processing {
    /// Adds white noise to the audio signal
    pub fn add_white_noise(signal: &mut [f32], noise_level: f32) {
        for sample in signal.iter_mut() {
            let noise = (rand::random::<f32>() * 2.0 - 1.0) * noise_level;
            *sample += noise;
        }
    }

    /// Applies a low-pass filter to the audio signal
    pub fn low_pass_filter(signal: &mut [f32], cutoff_freq: f32, sample_rate: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / sample_rate;
        let alpha = dt / (rc + dt);

        let mut prev_sample = signal[0];
        for sample in signal.iter_mut().skip(1) {
            *sample = prev_sample + alpha * (*sample - prev_sample);
            prev_sample = *sample;
        }
    }

    /// Normalizes the audio signal to a specified peak amplitude
    pub fn normalize(signal: &mut [f32], peak_amplitude: f32) {
        let max_amplitude = signal.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            let scaling_factor = peak_amplitude / max_amplitude;
            for sample in signal.iter_mut() {
                *sample *= scaling_factor;
            }
        }
    }

    /// Efficiently processes the audio signal by applying noise and filtering
    pub fn process_audio(signal: &mut [f32], noise_level: f32, cutoff_freq: f32, sample_rate: f32) {
        add_white_noise(signal, noise_level);
        low_pass_filter(signal, cutoff_freq, sample_rate);
        normalize(signal, 1.0);
    }
}

fn main() {
    let mut audio_signal = vec![0.5, 0.3, 0.8, 0.2, 0.7];
    audio_processing::process_audio(&mut audio_signal, 0.1, 1000.0, 44100.0);

    println!("Processed Audio Signal: {:?}", audio_signal);
}