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
        let sample_count = self.samples.len();
        for i in 0..sample_count {
            let t = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * t).sin();
            self.samples[i] += disturbance;
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

    fn process_in_parallel(&mut self, num_threads: usize, process_fn: Arc<dyn Fn(&mut [f32]) + Send + Sync>) {
        let chunk_size = self.samples.len() / num_threads;
        let mut handles = vec![];

        for chunk in self.samples.chunks_mut(chunk_size) {
            let process_fn = Arc::clone(&process_fn);
            handles.push(thread::spawn(move || {
                process_fn(chunk);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn main() {
    let sample_rate = 44100;
    let mut audio_buffer = AudioBuffer::new(vec![0.0; sample_rate * 2], sample_rate);

    let process_fn = Arc::new(|chunk: &mut [f32]| {
        for sample in chunk.iter_mut() {
            *sample = sample.tanh(); // Apply a non-linear transformation
        }
    });

    audio_buffer.add_disturbance(1000.0, 0.1);
    audio_buffer.process_in_parallel(4, process_fn);
    audio_buffer.normalize();

    println!("Audio processing complete.");
}