use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { samples, sample_rate }
    }

    fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let duration = self.samples.len() as f32 / self.sample_rate as f32;
        for (i, sample) in self.samples.iter_mut().enumerate() {
            let time = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * time).sin();
            *sample += disturbance;
        }
    }

    fn process_in_parallel(&mut self, num_threads: usize) {
        let chunk_size = self.samples.len() / num_threads;
        let mut handles = vec![];

        let samples_arc = Arc::new(self.samples.split_off(0));

        for i in 0..num_threads {
            let samples_clone = Arc::clone(&samples_arc);
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                self.samples.len()
            } else {
                start + chunk_size
            };

            handles.push(thread::spawn(move || {
                let mut local_samples = samples_clone[start..end].to_vec();
                for sample in &mut local_samples {
                    *sample = sample.abs().sqrt();
                }
                local_samples
            }));
        }

        let mut processed_samples = Vec::with_capacity(self.samples.len());
        for handle in handles {
            processed_samples.extend(handle.join().unwrap());
        }

        self.samples = processed_samples;
    }
}

fn main() {
    let sample_rate = 44100;
    let mut audio_buffer = AudioBuffer::new(vec![0.0; sample_rate * 2], sample_rate);

    audio_buffer.add_disturbance(440.0, 0.1);
    audio_buffer.process_in_parallel(4);

    println!("Audio processing complete.");
}