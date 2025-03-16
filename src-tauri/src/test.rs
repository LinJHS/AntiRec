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
        AudioBuffer {
            samples,
            sample_rate,
        }
    }

    fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut rng = rand::thread_rng();
        let disturbance: Vec<f32> = (0..self.samples.len())
            .map(|i| {
                let t = i as f32 / self.sample_rate as f32;
                amplitude * (2.0 * PI * frequency * t).sin() + rng.gen_range(-0.1..0.1)
            })
            .collect();

        for (sample, dist) in self.samples.iter_mut().zip(disturbance.iter()) {
            *sample += dist;
        }
    }

    fn process_in_parallel(&mut self, num_threads: usize, process_fn: Arc<Mutex<dyn Fn(&mut AudioBuffer) + Send>>) {
        let chunk_size = self.samples.len() / num_threads;
        let mut handles = vec![];

        for i in 0..num_threads {
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                self.samples.len()
            } else {
                start + chunk_size
            };

            let mut buffer = AudioBuffer {
                samples: self.samples[start..end].to_vec(),
                sample_rate: self.sample_rate,
            };

            let process_fn = Arc::clone(&process_fn);
            let handle = thread::spawn(move || {
                let process_fn = process_fn.lock().unwrap();
                process_fn(&mut buffer);
                buffer
            });

            handles.push(handle);
        }

        for (i, handle) in handles.into_iter().enumerate() {
            let processed_buffer = handle.join().unwrap();
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                self.samples.len()
            } else {
                start + chunk_size
            };
            self.samples[start..end].copy_from_slice(&processed_buffer.samples);
        }
    }
}

fn main() {
    let sample_rate = 44100;
    let samples = vec![0.0; sample_rate * 5]; // 5 seconds of silence
    let mut audio_buffer = AudioBuffer::new(samples, sample_rate);

    let process_fn = Arc::new(Mutex::new(|buffer: &mut AudioBuffer| {
        buffer.add_disturbance(440.0, 0.1); // Add a 440Hz disturbance
    }));

    audio_buffer.process_in_parallel(4, process_fn);
}