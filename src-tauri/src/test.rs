// src/audio_processor.rs

use std::f32::consts::PI;
use std::sync::Arc;

/// Audio processor that applies disturbances and optimizes audio signals.
pub struct AudioProcessor {
    sample_rate: u32,
    disturbance_factor: f32,
}

impl AudioProcessor {
    /// Create a new AudioProcessor with the given sample rate and disturbance factor.
    pub fn new(sample_rate: u32, disturbance_factor: f32) -> Self {
        Self {
            sample_rate,
            disturbance_factor,
        }
    }

    /// Process audio buffer by applying a disturbance and optimizing the signal.
    pub fn process(&self, buffer: &mut [f32]) {
        let frame_count = buffer.len();
        let frequency = 440.0; // A4 note frequency

        for i in 0..frame_count {
            let time = i as f32 / self.sample_rate as f32;
            let sine_wave = (2.0 * PI * frequency * time).sin();
            
            // Apply disturbance
            let disturbance = (self.disturbance_factor * sine_wave).tanh(); // Using tanh for smooth clipping
            
            // Optimize signal by mixing original and disturbed signal
            buffer[i] = (buffer[i] + disturbance) * 0.5;
        }

        // Apply a simple low-pass filter to smooth out the signal
        self.apply_low_pass_filter(buffer);
    }

    /// Apply a low-pass filter to the audio buffer.
    fn apply_low_pass_filter(&self, buffer: &mut [f32]) {
        let alpha = 0.5; // Filter coefficient
        let mut prev_sample = 0.0;

        for sample in buffer.iter_mut() {
            *sample = alpha * *sample + (1.0 - alpha) * prev_sample;
            prev_sample = *sample;
        }
    }

    /// Compute the RMS (Root Mean Square) of the audio buffer for volume normalization.
    pub fn compute_rms(&self, buffer: &[f32]) -> f32 {
        let sum_squares: f32 = buffer.iter().map(|&x| x * x).sum();
        (sum_squares / buffer.len() as f32).sqrt()
    }

    /// Normalize the audio buffer to a target RMS level.
    pub fn normalize(&self, buffer: &mut [f32], target_rms: f32) {
        let rms = self.compute_rms(buffer);
        if rms > 0.0 {
            let gain = target_rms / rms;
            for sample in buffer.iter_mut() {
                *sample *= gain;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processor() {
        let mut buffer = vec![0.0; 44100];
        let processor = AudioProcessor::new(44100, 0.1);
        processor.process(&mut buffer);

        let rms = processor.compute_rms(&buffer);
        assert!(rms > 0.0, "RMS should be greater than 0 after processing");
    }
}