use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Generates a sine wave with a given frequency and sample rate.
fn generate_sine_wave(frequency: f32, sample_rate: f32, duration: f32) -> Vec<f32> {
    let num_samples = (sample_rate * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let angular_frequency = 2.0 * PI * frequency;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate;
        samples.push((angular_frequency * t).sin());
    }

    samples
}

/// Adds white noise to the audio signal.
fn add_white_noise(signal: &mut Vec<f32>, noise_level: f32) {
    for sample in signal.iter_mut() {
        let noise = (rand::random::<f32>() - 0.5) * 2.0 * noise_level;
        *sample += noise;
    }
}

/// Applies a low-pass filter to the audio signal.
fn apply_low_pass_filter(signal: &mut Vec<f32>, cutoff_frequency: f32, sample_rate: f32) {
    let rc = 1.0 / (2.0 * PI * cutoff_frequency);
    let dt = 1.0 / sample_rate;
    let alpha = dt / (rc + dt);

    let mut prev_output = 0.0;

    for sample in signal.iter_mut() {
        *sample = alpha * *sample + (1.0 - alpha) * prev_output;
        prev_output = *sample;
    }
}

/// Processes audio in parallel using multiple threads.
fn parallel_audio_processing(signal: Arc<Vec<f32>>, num_threads: usize) -> Vec<f32> {
    let chunk_size = signal.len() / num_threads;
    let mut handles = vec![];

    for i in 0..num_threads {
        let signal_clone = Arc::clone(&signal);
        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                signal_clone.len()
            } else {
                start + chunk_size
            };

            signal_clone[start..end].to_vec()
        });

        handles.push(handle);
    }

    let mut processed_signal = Vec::new();
    for handle in handles {
        let mut chunk = handle.join().unwrap();
        processed_signal.append(&mut chunk);
    }

    processed_signal
}

fn main() {
    let sample_rate = 44100.0;
    let duration = 1.0;
    let frequency = 440.0;
    let noise_level = 0.1;
    let cutoff_frequency = 5000.0;

    let mut signal = generate_sine_wave(frequency, sample_rate, duration);
    add_white_noise(&mut signal, noise_level);
    apply_low_pass_filter(&mut signal, cutoff_frequency, sample_rate);

    let signal_arc = Arc::new(signal);
    let processed_signal = parallel_audio_processing(signal_arc, 4);

    println!("Audio processing complete.");
}