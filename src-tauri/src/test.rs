use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { samples, sample_rate }
    }

    fn apply_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut rng = rand::thread_rng();
        let disturbance: Vec<f32> = (0..self.samples.len())
            .map(|i| {
                let t = i as f32 / self.sample_rate as f32;
                amplitude * (2.0 * PI * frequency * t).sin() * rng.gen::<f32>()
            })
            .collect();

        for (sample, dist) in self.samples.iter_mut().zip(disturbance) {
            *sample += dist;
        }
    }

    fn normalize(&mut self) {
        let max_amplitude = self
            .samples
            .iter()
            .fold(0.0, |max, &sample| sample.abs().max(max));

        if max_amplitude > 0.0 {
            for sample in &mut self.samples {
                *sample /= max_amplitude;
            }
        }
    }

    fn process_in_parallel(&mut self, num_threads: usize) {
        let chunk_size = self.samples.len() / num_threads;
        let buffer_mutex = Arc::new(Mutex::new(&mut self.samples));

        let handles: Vec<_> = (0..num_threads)
            .map(|i| {
                let buffer_mutex = Arc::clone(&buffer_mutex);
                thread::spawn(move || {
                    let start = i * chunk_size;
                    let end = if i == num_threads - 1 {
                        self.samples.len()
                    } else {
                        start + chunk_size
                    };

                    let mut chunk = buffer_mutex.lock().unwrap()[start..end].to_vec();
                    chunk.iter_mut().for_each(|sample| *sample *= 0.5);
                    buffer_mutex.lock().unwrap()[start..end].copy_from_slice(&chunk);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn main() {
    let sample_rate = 44100;
    let samples = vec![0.0; sample_rate * 2]; // 2 seconds of silence
    let mut audio_buffer = AudioBuffer::new(samples, sample_rate);

    audio_buffer.apply_disturbance(440.0, 0.1);
    audio_buffer.normalize();
    audio_buffer.process_in_parallel(4);

    println!("Audio processing complete.");
}