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

    pub fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        for (i, sample) in self.samples.iter_mut().enumerate() {
            let t = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * t).sin();
            *sample += disturbance;
        }
    }

    pub fn normalize(&mut self) {
        let max_amplitude = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in self.samples.iter_mut() {
                *sample /= max_amplitude;
            }
        }
    }

    pub fn resample(&self, new_sample_rate: u32) -> AudioBuffer {
        let ratio = self.sample_rate as f32 / new_sample_rate as f32;
        let new_samples: Vec<f32> = (0..(self.samples.len() as f32 / ratio) as usize)
            .map(|i| self.samples[(i as f32 * ratio) as usize])
            .collect();
        AudioBuffer::new(new_samples, new_sample_rate)
    }

    pub fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_output = 0.0;
        for sample in self.samples.iter_mut() {
            *sample = prev_output + alpha * (*sample - prev_output);
            prev_output = *sample;
        }
    }
}