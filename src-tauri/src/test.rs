use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct AudioBuffer {
    samples: Arc<Vec<f32>>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer {
            samples: Arc::new(samples),
            sample_rate,
        }
    }

    fn add_disturbance(&self, frequency: f32, amplitude: f32) -> Self {
        let mut disturbed_samples = Vec::with_capacity(self.samples.len());
        let step = 2.0 * PI * frequency / self.sample_rate as f32;

        for (i, &sample) in self.samples.iter().enumerate() {
            let disturbance = (step * i as f32).sin() * amplitude;
            disturbed_samples.push(sample + disturbance);
        }

        AudioBuffer::new(disturbed_samples, self.sample_rate)
    }

    fn normalize(&self) -> Self {
        let max_amplitude = self.samples.iter()
            .fold(0.0, |max, &sample| f32::max(max, sample.abs()));

        if max_amplitude == 0.0 {
            return self.clone();
        }

        let normalized_samples = self.samples.iter()
            .map(|&sample| sample / max_amplitude)
            .collect();

        AudioBuffer::new(normalized_samples, self.sample_rate)
    }

    fn process_in_parallel(&self, num_threads: usize, process_fn: fn(f32) -> f32) -> Self {
        let chunk_size = self.samples.len() / num_threads;
        let mut handles = vec![];

        for i in 0..num_threads {
            let samples = Arc::clone(&self.samples);
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                samples.len()
            } else {
                start + chunk_size
            };

            handles.push(thread::spawn(move || {
                samples[start..end].iter().map(|&sample| process_fn(sample)).collect::<Vec<f32>>()
            }));
        }

        let mut processed_samples = Vec::with_capacity(self.samples.len());
        for handle in handles {
            processed_samples.extend(handle.join().unwrap());
        }

        AudioBuffer::new(processed_samples, self.sample_rate)
    }
}

fn apply_compression(sample: f32) -> f32 {
    let threshold = 0.5;
    let ratio = 2.0;
    if sample.abs() > threshold {
        threshold + (sample.abs() - threshold) / ratio
    } else {
        sample
    }
}

fn main() {
    let samples = vec![0.1, 0.5, 0.9, -0.8, -0.4, 0.3];
    let audio_buffer = AudioBuffer::new(samples, 44100);

    let disturbed_buffer = audio_buffer.add_disturbance(1000.0, 0.1);
    let normalized_buffer = disturbed_buffer.normalize();
    let processed_buffer = normalized_buffer.process_in_parallel(4, apply_compression);

    println!("Processed samples: {:?}", processed_buffer.samples);
}