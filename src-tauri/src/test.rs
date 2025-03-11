use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// A simple audio buffer structure
struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    /// Create a new audio buffer with the given sample rate
    fn new(sample_rate: u32) -> Self {
        AudioBuffer {
            samples: Vec::new(),
            sample_rate,
        }
    }

    /// Add a sample to the buffer
    fn push_sample(&mut self, sample: f32) {
        self.samples.push(sample);
    }

    /// Apply a disturbance (noise) to the audio buffer
    fn add_disturbance(&mut self, amplitude: f32) {
        for sample in &mut self.samples {
            let noise = (rand::random::<f32>() * 2.0 - 1.0) * amplitude;
            *sample += noise;
        }
    }

    /// Apply a low-pass filter to the audio buffer
    fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_sample = 0.0;
        for sample in &mut self.samples {
            let filtered_sample = prev_sample + alpha * (*sample - prev_sample);
            prev_sample = filtered_sample;
            *sample = filtered_sample;
        }
    }

    /// Process the audio buffer in parallel using multiple threads
    fn process_in_parallel(&mut self, num_threads: usize) {
        let chunk_size = self.samples.len() / num_threads;
        let mut handles = vec![];

        let samples_arc = Arc::new(self.samples.clone());

        for i in 0..num_threads {
            let samples_arc = Arc::clone(&samples_arc);
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                self.samples.len()
            } else {
                start + chunk_size
            };

            handles.push(thread::spawn(move || {
                let mut local_samples = samples_arc[start..end].to_vec();
                for sample in &mut local_samples {
                    *sample = sample.sin(); // Example processing: apply sine wave
                }
                local_samples
            }));
        }

        let mut processed_samples = Vec::new();
        for handle in handles {
            processed_samples.extend(handle.join().unwrap());
        }

        self.samples = processed_samples;
    }
}

fn main() {
    let mut audio_buffer = AudioBuffer::new(44100);

    // Generate a simple sine wave
    for i in 0..44100 {
        let sample = (2.0 * PI * 440.0 * i as f32 / 44100.0).sin();
        audio_buffer.push_sample(sample);
    }

    // Add some disturbance
    audio_buffer.add_disturbance(0.1);

    // Apply a low-pass filter
    audio_buffer.apply_low_pass_filter(1000.0);

    // Process the audio in parallel
    audio_buffer.process_in_parallel(4);

    // Simulate playback
    for sample in &audio_buffer.samples {
        println!("{}", sample);
        thread::sleep(Duration::from_millis(10));
    }
}