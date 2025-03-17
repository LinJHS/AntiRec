use std::f32::consts::PI;
use std::sync::Arc;
use rayon::prelude::*;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer_size: usize,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer_size: usize) -> Self {
        AudioProcessor { sample_rate, buffer_size }
    }

    pub fn process_buffer(&self, buffer: &mut [f32]) {
        buffer.par_iter_mut().for_each(|sample| {
            *sample = self.add_disturbance(*sample);
        });
    }

    fn add_disturbance(&self, sample: f32) -> f32 {
        let noise = (2.0 * PI * rand::random::<f32>()).sin() * 0.1;
        sample + noise
    }

    pub fn apply_low_pass_filter(&self, buffer: &mut [f32], cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_sample = 0.0;
        for sample in buffer.iter_mut() {
            *sample = prev_sample + alpha * (*sample - prev_sample);
            prev_sample = *sample;
        }
    }

    pub fn normalize_buffer(&self, buffer: &mut [f32]) {
        let max_amplitude = buffer.iter()
            .fold(0.0, |acc, &x| acc.max(x.abs()));

        if max_amplitude > 0.0 {
            buffer.par_iter_mut().for_each(|sample| {
                *sample /= max_amplitude;
            });
        }
    }
}

pub fn process_audio(buffer: &mut [f32], sample_rate: u32) -> Arc<AudioProcessor> {
    let processor = Arc::new(AudioProcessor::new(sample_rate, buffer.len()));
    processor.process_buffer(buffer);
    processor
}