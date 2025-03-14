use std::f32::consts::PI;

#[derive(Debug)]
pub struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    pub fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { samples, sample_rate }
    }

    pub fn add_noise(&mut self, noise_level: f32) {
        for sample in &mut self.samples {
            let noise = (rand::random::<f32>() - 0.5) * noise_level;
            *sample += noise;
        }
    }

    pub fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_output = self.samples[0];
        for sample in &mut self.samples {
            let output = alpha * *sample + (1.0 - alpha) * prev_output;
            prev_output = output;
            *sample = output;
        }
    }

    pub fn normalize(&mut self) {
        let max_sample = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_sample > 0.0 {
            for sample in &mut self.samples {
                *sample /= max_sample;
            }
        }
    }

    pub fn to_mono(&mut self) {
        if self.samples.len() % 2 != 0 {
            return; // Assuming stereo samples
        }

        let mut mono_samples = Vec::with_capacity(self.samples.len() / 2);
        for i in (0..self.samples.len()).step_by(2) {
            let mono_sample = (self.samples[i] + self.samples[i + 1]) / 2.0;
            mono_samples.push(mono_sample);
        }
        self.samples = mono_samples;
    }
}