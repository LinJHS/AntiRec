use std::f32::consts::PI;
use rand::Rng;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer_size: usize,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer_size: usize) -> Self {
        AudioProcessor {
            sample_rate,
            buffer_size,
        }
    }

    pub fn process_audio(&self, input: &[f32], disturbance_level: f32) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        input.iter()
            .map(|&sample| {
                let noise = rng.gen_range(-disturbance_level..disturbance_level);
                sample + noise
            })
            .collect()
    }

    pub fn apply_low_pass_filter(&self, input: &[f32], cutoff_frequency: f32) -> Vec<f32> {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut filtered = Vec::with_capacity(input.len());
        let mut prev_output = 0.0;

        for &sample in input {
            let output = prev_output + alpha * (sample - prev_output);
            filtered.push(output);
            prev_output = output;
        }

        filtered
    }

    pub fn apply_high_pass_filter(&self, input: &[f32], cutoff_frequency: f32) -> Vec<f32> {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = rc / (rc + dt);

        let mut filtered = Vec::with_capacity(input.len());
        let mut prev_input = 0.0;
        let mut prev_output = 0.0;

        for &sample in input {
            let output = alpha * (prev_output + sample - prev_input);
            filtered.push(output);
            prev_input = sample;
            prev_output = output;
        }

        filtered
    }

    pub fn apply_distortion(&self, input: &[f32], gain: f32) -> Vec<f32> {
        input.iter()
            .map(|&sample| {
                let distorted = sample * gain;
                distorted.tanh()
            })
            .collect()
    }
}