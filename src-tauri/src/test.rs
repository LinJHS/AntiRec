use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;

/// A simple audio buffer structure to hold audio samples.
struct AudioBuffer {
    samples: Vec<f32>,
}

impl AudioBuffer {
    /// Creates a new AudioBuffer with the specified capacity.
    fn new(capacity: usize) -> Self {
        AudioBuffer {
            samples: Vec::with_capacity(capacity),
        }
    }

    /// Adds a sample to the buffer.
    fn push(&mut self, sample: f32) {
        self.samples.push(sample);
    }

    /// Processes the audio buffer by applying a disturbance (noise) to the samples.
    fn add_disturbance(&mut self, disturbance_level: f32) {
        for sample in &mut self.samples {
            let noise = (rand::random::<f32>() - 0.5) * disturbance_level;
            *sample += noise;
        }
    }

    /// Applies a low-pass filter to smoothen the audio samples.
    fn apply_low_pass_filter(&mut self, cutoff_frequency: f32, sample_rate: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / sample_rate;
        let alpha = dt / (rc + dt);

        let mut prev_output = 0.0;
        for sample in &mut self.samples {
            let output = prev_output + alpha * (*sample - prev_output);
            prev_output = output;
            *sample = output;
        }
    }
}

/// A multi-threaded audio processor that processes audio buffers in parallel.
struct AudioProcessor {
    buffers: Arc<Mutex<Vec<AudioBuffer>>>,
}

impl AudioProcessor {
    /// Creates a new AudioProcessor with the specified number of buffers.
    fn new(num_buffers: usize, buffer_capacity: usize) -> Self {
        let buffers = (0..num_buffers)
            .map(|_| AudioBuffer::new(buffer_capacity))
            .collect::<Vec<_>>();
        AudioProcessor {
            buffers: Arc::new(Mutex::new(buffers)),
        }
    }

    /// Processes all audio buffers in parallel using multiple threads.
    fn process_buffers(&self, disturbance_level: f32, cutoff_frequency: f32, sample_rate: f32) {
        let handles = (0..self.buffers.lock().unwrap().len())
            .map(|i| {
                let buffers = Arc::clone(&self.buffers);
                thread::spawn(move || {
                    let mut buffers = buffers.lock().unwrap();
                    buffers[i].add_disturbance(disturbance_level);
                    buffers[i].apply_low_pass_filter(cutoff_frequency, sample_rate);
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn main() {
    let processor = AudioProcessor::new(4, 1024);
    processor.process_buffers(0.1, 1000.0, 44100.0);
}