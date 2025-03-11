use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// A struct representing an audio buffer with added noise
struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
    noise_level: f32,
}

impl AudioBuffer {
    /// Create a new AudioBuffer with the given samples and sample rate
    fn new(samples: Vec<f32>, sample_rate: u32, noise_level: f32) -> Self {
        AudioBuffer {
            samples,
            sample_rate,
            noise_level,
        }
    }

    /// Add white noise to the audio buffer
    fn add_noise(&mut self) {
        for sample in &mut self.samples {
            let noise = (rand::random::<f32>() - 0.5) * 2.0 * self.noise_level;
            *sample += noise;
        }
    }

    /// Apply a low-pass filter to the audio buffer
    fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_sample = self.samples[0];
        for sample in &mut self.samples {
            let filtered_sample = alpha * *sample + (1.0 - alpha) * prev_sample;
            prev_sample = filtered_sample;
            *sample = filtered_sample;
        }
    }

    /// Process the audio buffer in parallel
    fn process_parallel(&mut self) {
        let chunk_size = self.samples.len() / 4;
        let mut handles = vec![];

        for chunk in self.samples.chunks_mut(chunk_size) {
            let noise_level = self.noise_level;
            let handle = thread::spawn(move || {
                for sample in chunk {
                    let noise = (rand::random::<f32>() - 0.5) * 2.0 * noise_level;
                    *sample += noise;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn main() {
    let sample_rate = 44100;
    let noise_level = 0.1;
    let mut audio_buffer = AudioBuffer::new(vec![0.0; 44100], sample_rate, noise_level);

    audio_buffer.add_noise();
    audio_buffer.apply_low_pass_filter(5000.0);
    audio_buffer.process_parallel();

    println!("Audio processing complete.");
}