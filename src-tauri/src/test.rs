use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// A struct representing an audio buffer with sample rate and data
#[derive(Clone)]
struct AudioBuffer {
    sample_rate: u32,
    data: Vec<f32>,
}

impl AudioBuffer {
    /// Create a new AudioBuffer with a given sample rate and data
    fn new(sample_rate: u32, data: Vec<f32>) -> Self {
        AudioBuffer { sample_rate, data }
    }

    /// Add white noise to the audio buffer
    fn add_white_noise(&mut self, noise_level: f32) {
        self.data.iter_mut().for_each(|sample| {
            let noise = (rand::random::<f32>() - 0.5) * 2.0 * noise_level;
            *sample += noise;
        });
    }

    /// Apply a low-pass filter to the audio buffer
    fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_sample = 0.0;
        for sample in &mut self.data {
            let filtered_sample = prev_sample + alpha * (*sample - prev_sample);
            prev_sample = filtered_sample;
            *sample = filtered_sample;
        }
    }

    /// Process the audio buffer in parallel using multiple threads
    fn process_in_parallel(&mut self, num_threads: usize, process_fn: fn(&mut f32)) {
        let chunk_size = self.data.len() / num_threads;
        let mut handles = vec![];

        let data_arc = Arc::new(self.data.split_off(0));

        for i in 0..num_threads {
            let data_arc_clone = Arc::clone(&data_arc);
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                data_arc_clone.len()
            } else {
                start + chunk_size
            };

            handles.push(thread::spawn(move || {
                let mut chunk = data_arc_clone[start..end].to_vec();
                chunk.iter_mut().for_each(process_fn);
                chunk
            }));
        }

        let mut processed_data = Vec::with_capacity(self.data.len());
        for handle in handles {
            processed_data.extend(handle.join().unwrap());
        }

        self.data = processed_data;
    }
}

/// Example usage of the AudioBuffer struct
fn main() {
    let sample_rate = 44100;
    let mut audio_buffer = AudioBuffer::new(sample_rate, vec![0.0; 44100]);

    // Add white noise to the audio buffer
    audio_buffer.add_white_noise(0.05);

    // Apply a low-pass filter to the audio buffer
    audio_buffer.apply_low_pass_filter(1000.0);

    // Process the audio buffer in parallel
    audio_buffer.process_in_parallel(4, |sample| {
        *sample = sample.sin(); // Example processing function
    });

    // Simulate some delay to observe the processing
    thread::sleep(Duration::from_secs(1));
}