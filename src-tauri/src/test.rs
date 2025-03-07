use std::f32::consts::PI;
use std::sync::Arc;
use rayon::prelude::*;

#[derive(Clone)]
struct AudioBuffer {
    samples: Arc<Vec<f32>>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        Self {
            samples: Arc::new(samples),
            sample_rate,
        }
    }

    fn add_disturbance(&self, frequency: f32, amplitude: f32) -> Self {
        let mut disturbed_samples = Vec::with_capacity(self.samples.len());
        let phase_step = 2.0 * PI * frequency / self.sample_rate as f32;

        self.samples.par_iter().enumerate().for_each(|(i, &sample)| {
            let phase = phase_step * i as f32;
            let disturbance = amplitude * phase.sin();
            disturbed_samples.push(sample + disturbance);
        });

        AudioBuffer::new(disturbed_samples, self.sample_rate)
    }

    fn apply_low_pass_filter(&self, cutoff_frequency: f32) -> Self {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut filtered_samples = Vec::with_capacity(self.samples.len());
        let mut prev_sample = 0.0;

        self.samples.iter().for_each(|&sample| {
            let filtered_sample = prev_sample + alpha * (sample - prev_sample);
            filtered_samples.push(filtered_sample);
            prev_sample = filtered_sample;
        });

        AudioBuffer::new(filtered_samples, self.sample_rate)
    }

    fn rms(&self) -> f32 {
        let sum_squares: f32 = self.samples.par_iter().map(|&s| s * s).sum();
        (sum_squares / self.samples.len() as f32).sqrt()
    }
}

fn main() {
    let audio_buffer = AudioBuffer::new(vec![0.0; 44100], 44100);
    let disturbed = audio_buffer.add_disturbance(1000.0, 0.1);
    let filtered = disturbed.apply_low_pass_filter(500.0);
    let rms = filtered.rms();
    println!("RMS: {}", rms);
}