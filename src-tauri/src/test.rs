use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;

struct AudioBuffer {
    data: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(data: Vec<f32>, sample_rate: u32) -> Self {
        Self { data, sample_rate }
    }

    fn apply_disturbance(&mut self, frequency: f32, amplitude: f32) {
        for i in 0..self.data.len() {
            let t = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * t).sin();
            self.data[i] += disturbance;
        }
    }

    fn normalize(&mut self) {
        let max_amplitude = self.data.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            let scale = 1.0 / max_amplitude;
            for sample in &mut self.data {
                *sample *= scale;
            }
        }
    }

    fn process_in_parallel(&mut self, num_threads: usize) {
        let chunk_size = self.data.len() / num_threads;
        let mut handles = vec![];
        let arc_data = Arc::new(self.data.clone());

        for i in 0..num_threads {
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                self.data.len()
            } else {
                start + chunk_size
            };

            let arc_data = Arc::clone(&arc_data);
            let handle = thread::spawn(move || {
                let mut local_data = arc_data[start..end].to_vec();
                for sample in &mut local_data {
                    *sample = sample.tanh(); 
                }
                local_data
            });

            handles.push((start, end, handle));
        }

        for (start, end, handle) in handles {
            self.data[start..end].copy_from_slice(&handle.join().unwrap());
        }
    }
}

fn main() {
    let sample_rate = 44100;
    let mut audio_buffer = AudioBuffer::new(vec![0.0; sample_rate * 2], sample_rate);

    audio_buffer.apply_disturbance(440.0, 0.1);
    audio_buffer.normalize();
    audio_buffer.process_in_parallel(4);

    println!("Audio Buffer: {:?}", audio_buffer.data);
}