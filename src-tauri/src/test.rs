use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;

struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { samples, sample_rate }
    }

    fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut rng = rand::thread_rng();
        for i in 0..self.samples.len() {
            let t = i as f32 / self.sample_rate as f32;
            let noise = amplitude * (2.0 * PI * frequency * t).sin();
            self.samples[i] += noise + rng.gen_range(-0.1..0.1);
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
                    *sample = sample.powf(2.0); // Example processing
                }
                local_samples
            }));
        }

        let mut result = Vec::with_capacity(self.samples.len());
        for handle in handles {
            result.extend(handle.join().unwrap());
        }

        self.samples = result;
    }
}

fn main() {
    let mut audio_buffer = AudioBuffer::new(vec![0.0; 44100], 44100);
    audio_buffer.add_disturbance(440.0, 0.5);
    audio_buffer.normalize();
    audio_buffer.process_in_parallel(4);
}