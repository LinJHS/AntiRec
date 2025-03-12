use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        Self { samples, sample_rate }
    }

    fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut rng = rand::thread_rng();
        for i in 0..self.samples.len() {
            let t = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * t).sin();
            self.samples[i] += disturbance + rng.gen_range(-0.01..0.01);
        }
    }

    fn normalize(&mut self) {
        let max_amplitude = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in &mut self.samples {
                *sample /= max_amplitude;
            }
        }
    }

    fn process_in_parallel(&self, num_threads: usize) -> AudioBuffer {
        let chunk_size = self.samples.len() / num_threads;
        let mut handles = vec![];
        let arc_samples = Arc::new(self.samples.clone());

        for i in 0..num_threads {
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                self.samples.len()
            } else {
                start + chunk_size
            };
            let samples_ref = Arc::clone(&arc_samples);
            handles.push(thread::spawn(move || {
                let mut processed_chunk = samples_ref[start..end].to_vec();
                for sample in &mut processed_chunk {
                    *sample = (*sample * 0.5).tanh(); // Apply a simple non-linear transformation
                }
                processed_chunk
            }));
        }

        let mut processed_samples = vec![0.0; self.samples.len()];
        for (i, handle) in handles.into_iter().enumerate() {
            let start = i * chunk_size;
            let processed_chunk = handle.join().unwrap();
            processed_samples[start..start + processed_chunk.len()].copy_from_slice(&processed_chunk);
        }

        AudioBuffer::new(processed_samples, self.sample_rate)
    }
}

fn main() {
    let samples = vec![0.0; 44100]; // 1 second of silence at 44.1kHz
    let mut audio_buffer = AudioBuffer::new(samples, 44100);

    audio_buffer.add_disturbance(1000.0, 0.1); // Add 1kHz disturbance
    audio_buffer.normalize();

    let processed_buffer = audio_buffer.process_in_parallel(4); // Process in parallel with 4 threads

    println!("Audio processing complete!");
}