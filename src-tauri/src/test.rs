use std::f32::consts::PI;

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

    pub fn compute_rms(&self) -> f32 {
        let sum_squares: f32 = self.samples.iter().map(|&s| s * s).sum();
        (sum_squares / self.samples.len() as f32).sqrt()
    }

    pub fn normalize(&mut self) {
        let rms = self.compute_rms();
        if rms > 0.0 {
            let scale_factor = 1.0 / rms;
            for sample in self.samples.iter_mut() {
                *sample *= scale_factor;
            }
        }
    }

    pub fn process_with_gain(&mut self, gain: f32) {
        for sample in self.samples.iter_mut() {
            *sample *= gain;
        }
    }
}

pub fn apply_low_pass_filter(buffer: &mut AudioBuffer, cutoff_frequency: f32) {
    let rc = 1.0 / (2.0 * PI * cutoff_frequency);
    let dt = 1.0 / buffer.sample_rate as f32;
    let alpha = dt / (rc + dt);

    let mut prev_output = 0.0;

    for sample in buffer.samples.iter_mut() {
        let output = prev_output + alpha * (*sample - prev_output);
        prev_output = output;
        *sample = output;
    }
}