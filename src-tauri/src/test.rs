use std::f32::consts::PI;
use rand::Rng;

// Generate a sine wave with a given frequency and sample rate
fn generate_sine_wave(frequency: f32, sample_rate: f32, duration: f32) -> Vec<f32> {
    let num_samples = (sample_rate * duration) as usize;
    let mut wave = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate;
        wave.push((2.0 * PI * frequency * t).sin());
    }
    wave
}

// Add white noise to the audio signal
fn add_white_noise(signal: &mut Vec<f32>, noise_level: f32) {
    let mut rng = rand::thread_rng();
    for sample in signal.iter_mut() {
        let noise: f32 = rng.gen_range(-noise_level..noise_level);
        *sample += noise;
    }
}

// Apply a low-pass filter to the audio signal
fn apply_low_pass_filter(signal: &mut Vec<f32>, cutoff_frequency: f32, sample_rate: f32) {
    let rc = 1.0 / (2.0 * PI * cutoff_frequency);
    let dt = 1.0 / sample_rate;
    let alpha = dt / (rc + dt);

    let mut prev_output = 0.0;
    for sample in signal.iter_mut() {
        *sample = prev_output + alpha * (*sample - prev_output);
        prev_output = *sample;
    }
}

// Main function to process audio
fn process_audio() {
    let sample_rate = 44100.0;
    let duration = 2.0;
    let frequency = 440.0;
    let noise_level = 0.1;
    let cutoff_frequency = 5000.0;

    let mut audio_signal = generate_sine_wave(frequency, sample_rate, duration);
    add_white_noise(&mut audio_signal, noise_level);
    apply_low_pass_filter(&mut audio_signal, cutoff_frequency, sample_rate);

    // Output the processed audio signal (simulated)
    for sample in audio_signal.iter() {
        println!("{}", sample);
    }
}

fn main() {
    process_audio();
}