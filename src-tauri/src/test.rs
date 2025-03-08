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
        for (i, sample) in self.samples.iter_mut().enumerate() {
            let time = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * time).sin();
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

    fn process_in_parallel(&self, num_threads: usize, process_fn: Arc<dyn Fn(&mut AudioBuffer) + Send + Sync>) {
        let chunk_size = self.samples.len() / num_threads;
        let mut handles = vec![];

        for i in 0..num_threads {
            let mut buffer = AudioBuffer::new(self.samples.clone(), self.sample_rate);
            let process_fn = Arc::clone(&process_fn);
            let handle = thread::spawn(move || {
                let start = i * chunk_size;
                let end = if i == num_threads - 1 {
                    buffer.samples.len()
                } else {
                    start + chunk_size
                };
                process_fn(&mut buffer);
                buffer.samples[start..end].to_vec()
            });
            handles.push(handle);
        }

        let mut result = Vec::with_capacity(self.samples.len());
        for handle in handles {
            result.extend(handle.join().unwrap());
        }

        self.samples.clone_from(&result);
    }
}

fn main() {
    let sample_rate = 44100;
    let mut buffer = AudioBuffer::new(vec![0.0; 44100], sample_rate);

    buffer.add_disturbance(440.0, 0.1);
    buffer.normalize();

    let process_fn = Arc::new(|buf: &mut AudioBuffer| {
        buf.add_disturbance(880.0, 0.05);
    });

    buffer.process_in_parallel(4, process_fn);

    // Further processing or output of the buffer can be done here
}