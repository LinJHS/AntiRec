use std::f32::consts::PI;
use rayon::prelude::*;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer: Vec<f32>,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer: Vec<f32>) -> Self {
        AudioProcessor { sample_rate, buffer }
    }

    pub fn apply_disturbance(&mut self, frequency: f32, amplitude: f32) {
        self.buffer.par_iter_mut().enumerate().for_each(|(i, sample)| {
            let t = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * t).sin();
            *sample += disturbance;
        });
    }

    pub fn normalize(&mut self) {
        let max_amplitude = self.buffer.par_iter()
            .map(|&x| x.abs())
            .reduce(|| 0.0, f32::max);

        if max_amplitude > 0.0 {
            self.buffer.par_iter_mut().for_each(|sample| {
                *sample /= max_amplitude;
            });
        }
    }

    pub fn low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        self.buffer.iter_mut().fold(0.0, |prev, sample| {
            *sample = prev + alpha * (*sample - prev);
            *sample
        });
    }

    pub fn get_buffer(&self) -> &Vec<f32> {
        &self.buffer
    }
}