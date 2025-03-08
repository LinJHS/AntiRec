use std::f32::consts::PI;

/// A struct representing an audio buffer with disturbance and efficiency improvements.
pub struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
    disturbance_level: f32,
}

impl AudioBuffer {
    /// Creates a new AudioBuffer with the given samples and sample rate.
    pub fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        Self {
            samples,
            sample_rate,
            disturbance_level: 0.0,
        }
    }

    /// Adds a sinusoidal disturbance to the audio buffer.
    pub fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let disturbance_phase = 2.0 * PI * frequency / self.sample_rate as f32;
        for (i, sample) in self.samples.iter_mut().enumerate() {
            let disturbance = amplitude * (disturbance_phase * i as f32).sin();
            *sample += disturbance;
        }
    }

    /// Normalizes the audio buffer to prevent clipping.
    pub fn normalize(&mut self) {
        let max_amplitude = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 1.0 {
            for sample in self.samples.iter_mut() {
                *sample /= max_amplitude;
            }
        }
    }

    /// Applies a low-pass filter to the audio buffer for efficiency improvements.
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

    /// Returns a reference to the processed samples.
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }
}