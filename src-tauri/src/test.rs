use std::f32::consts::PI;

fn apply_disturbance(input_signal: &[f32], noise_level: f32) -> Vec<f32> {
    input_signal.iter()
        .map(|&sample| sample + (noise_level * (rand::random::<f32>() - 0.5)))
        .collect()
}

fn low_pass_filter(signal: &mut [f32], cutoff_freq: f32, sample_rate: f32) {
    let rc = 1.0 / (2.0 * PI * cutoff_freq);
    let dt = 1.0 / sample_rate;
    let alpha = dt / (rc + dt);

    signal.iter_mut().fold(0.0, |prev, sample| {
        let filtered = prev + alpha * (*sample - prev);
        *sample = filtered;
        filtered
    });
}

fn process_audio(signal: &mut [f32], sample_rate: f32) -> Vec<f32> {
    let noise_level = 0.05;
    let cutoff_freq = 5000.0;

    let disturbed_signal = apply_disturbance(signal, noise_level);
    let mut filtered_signal = disturbed_signal.clone();
    low_pass_filter(&mut filtered_signal, cutoff_freq, sample_rate);

    filtered_signal
}

fn main() {
    let sample_rate = 44100.0;
    let mut signal = vec![0.0; 44100]; // Example audio signal

    let processed_signal = process_audio(&mut signal, sample_rate);

    println!("Processed audio signal: {:?}", processed_signal);
}