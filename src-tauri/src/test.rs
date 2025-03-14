use std::f32::consts::PI;
use std::sync::Arc;

/// Represents an audio buffer with a fixed sample rate.
pub struct AudioBuffer {
    sample_rate: u32,
    samples: Vec<f32>,
}

impl AudioBuffer {
    /// Creates a new AudioBuffer with the given sample rate and samples.
    pub fn new(sample_rate: u32, samples: Vec<f32>) -> Self {
        Self { sample_rate, samples }
    }

    /// Applies a disturbance to the audio buffer by adding white noise.
    pub fn add_disturbance(&mut self, noise_level: f32) {
        let mut rng = rand::thread_rng();
        self.samples.iter_mut().for_each(|sample| {
            let noise: f32 = rng.gen_range(-noise_level..noise_level);
            *sample += noise;
        });
    }

    /// Applies a low-pass filter to the audio buffer to smooth out high-frequency noise.
    pub fn apply_low_pass_filter(&mut self, cutoff_freq: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_sample = 0.0;
        for sample in &mut self.samples {
            let filtered = prev_sample + alpha * (*sample - prev_sample);
            prev_sample = filtered;
            *sample = filtered;
        }
    }

    /// Resamples the audio buffer to a new sample rate.
    pub fn resample(&self, new_sample_rate: u32) -> AudioBuffer {
        let ratio = new_sample_rate as f32 / self.sample_rate as f32;
        let new_len = (self.samples.len() as f32 * ratio) as usize;
        let mut resampled = Vec::with_capacity(new_len);

        for i in 0..new_len {
            let pos = i as f32 / ratio;
            let prev_index = pos.floor() as usize;
            let next_index = (prev_index + 1).min(self.samples.len() - 1);
            let weight = pos - prev_index as f32;

            let prev_sample = self.samples[prev_index];
            let next_sample = self.samples[next_index];
            let interpolated = prev_sample + weight * (next_sample - prev_sample);
            resampled.push(interpolated);
        }

        AudioBuffer::new(new_sample_rate, resampled)
    }
}

/// Represents a multi-threaded audio processor.
pub struct AudioProcessor {
    buffer: Arc<AudioBuffer>,
}

impl AudioProcessor {
    pub fn new(buffer: AudioBuffer) -> Self {
        Self {
            buffer: Arc::new(buffer),
        }
    }

    /// Processes the audio buffer in parallel to apply effects.
    pub fn process_parallel(&self) -> AudioBuffer {
        let buffer = Arc::clone(&self.buffer);
        let mut processed = buffer.samples.clone();

        // Parallelize the processing of the audio buffer
        processed.par_iter_mut().for_each(|sample| {
            // Example effect: Apply a simple gain reduction
            *sample *= 0.8;
        });

        AudioBuffer::new(buffer.sample_rate, processed)
    }
}

use rand::Rng;
use rayon::prelude::*;