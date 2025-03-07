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

    fn process_in_parallel(&mut self, num_threads: usize, process_fn: fn(f32) -> f32) {
        let chunk_size = self.samples.len() / num_threads;
        let samples_arc = Arc::new(Mutex::new(&mut self.samples));

        let handles: Vec<_> = (0..num_threads).map(|i| {
            let samples_arc = Arc::clone(&samples_arc);
            thread::spawn(move || {
                let mut samples = samples_arc.lock().unwrap();
                let start = i * chunk_size;
                let end = if i == num_threads - 1 {
                    samples.len()
                } else {
                    start + chunk_size
                };
                for j in start..end {
                    samples[j] = process_fn(samples[j]);
                }
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn low_pass_filter(sample: f32) -> f32 {
    static mut PREV_SAMPLE: f32 = 0.0;
    let alpha = 0.5;
    unsafe {
        let filtered_sample = alpha * sample + (1.0 - alpha) * PREV_SAMPLE;
        PREV_SAMPLE = filtered_sample;
        filtered_sample
    }
}

fn main() {
    let samples = vec![0.0; 44100];
    let mut audio_buffer = AudioBuffer::new(samples, 44100);
    audio_buffer.add_disturbance(50.0, 0.1);
    audio_buffer.process_in_parallel(4, low_pass_filter);
    audio_buffer.normalize();
}