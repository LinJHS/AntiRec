use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { samples, sample_rate }
    }

    fn apply_disturbance(&mut self, frequency: f32, amplitude: f32) {
        for (i, sample) in self.samples.iter_mut().enumerate() {
            let time = i as f32 / self.sample_rate as f32;
            let disturbance = (2.0 * PI * frequency * time).sin() * amplitude;
            *sample += disturbance;
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

    fn process_in_parallel(&mut self, num_threads: usize, task: fn(&mut [f32])) {
        let chunk_size = self.samples.len() / num_threads;
        let mut handles = vec![];

        let samples = Arc::new(self.samples.clone());

        for i in 0..num_threads {
            let samples = Arc::clone(&samples);
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                self.samples.len()
            } else {
                start + chunk_size
            };

            let handle = thread::spawn(move || {
                let mut chunk = samples[start..end].to_vec();
                task(&mut chunk);
                chunk
            });

            handles.push(handle);
        }

        self.samples.clear();
        for handle in handles {
            self.samples.extend(handle.join().unwrap());
        }
    }
}

fn main() {
    let sample_rate = 44100;
    let mut audio_buffer = AudioBuffer::new(vec![0.0; sample_rate], sample_rate);

    audio_buffer.apply_disturbance(440.0, 0.5);
    audio_buffer.normalize();

    audio_buffer.process_in_parallel(4, |chunk| {
        for sample in chunk.iter_mut() {
            *sample = sample.powf(2.0);
        }
    });

    println!("{:?}", audio_buffer);
}