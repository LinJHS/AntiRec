use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer_size: usize,
    disturbance_level: f32,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer_size: usize, disturbance_level: f32) -> Self {
        AudioProcessor {
            sample_rate,
            buffer_size,
            disturbance_level,
        }
    }

    pub fn process_audio(&self, input: &[f32], output: &mut [f32]) {
        let disturbance = self.disturbance_level * (2.0 * PI * 440.0 / self.sample_rate as f32).sin();
        
        for i in 0..self.buffer_size {
            output[i] = input[i] + disturbance;
        }
    }

    pub fn parallel_process(&self, input: Arc<Vec<f32>>, output: Arc<Vec<f32>>) {
        let handles: Vec<_> = (0..4).map(|i| {
            let input = Arc::clone(&input);
            let output = Arc::clone(&output);
            thread::spawn(move || {
                let chunk_size = self.buffer_size / 4;
                let start = i * chunk_size;
                let end = start + chunk_size;
                self.process_audio(&input[start..end], &mut output[start..end]);
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

pub fn apply_reverb(samples: &mut [f32], delay: usize, decay: f32) {
    for i in delay..samples.len() {
        samples[i] += samples[i - delay] * decay;
    }
}

pub fn optimize_audio_processing(samples: &mut [f32]) {
    samples.iter_mut().for_each(|sample| *sample = sample.clamp(-1.0, 1.0));
}