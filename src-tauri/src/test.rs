use std::f32::consts::PI;

pub struct AudioProcessor {
    sample_rate: u32,
    disturbance_level: f32,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, disturbance_level: f32) -> Self {
        AudioProcessor {
            sample_rate,
            disturbance_level,
        }
    }

    pub fn process_audio(&self, input: &[f32], output: &mut [f32]) {
        for (i, sample) in input.iter().enumerate() {
            let disturbance = self.disturbance_level * (2.0 * PI * i as f32 / self.sample_rate as f32).sin();
            output[i] = sample + disturbance;
        }
    }

    pub fn apply_low_pass_filter(&self, input: &[f32], output: &mut [f32], cutoff_freq: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        output[0] = input[0];
        for i in 1..input.len() {
            output[i] = alpha * input[i] + (1.0 - alpha) * output[i - 1];
        }
    }

    pub fn amplify(&self, input: &[f32], output: &mut [f32], gain: f32) {
        for (i, sample) in input.iter().enumerate() {
            output[i] = sample * gain;
        }
    }
}