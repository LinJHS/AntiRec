// src/audio_processor.rs

use std::f32::consts::PI;
use rayon::prelude::*;

pub struct AudioProcessor {
    sample_rate: u32,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32) -> Self {
        AudioProcessor { sample_rate }
    }

    pub fn apply_disturbance(&self, audio_buffer: &mut [f32], frequency: f32, amplitude: f32) {
        let angular_frequency = 2.0 * PI * frequency / self.sample_rate as f32;

        audio_buffer.par_iter_mut().enumerate().for_each(|(i, sample)| {
            let phase = angular_frequency * i as f32;
            *sample += amplitude * phase.sin();
        });
    }

    pub fn normalize_audio(&self, audio_buffer: &mut [f32]) {
        let max_amplitude = audio_buffer.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        
        if max_amplitude > 0.0 {
            audio_buffer.par_iter_mut().for_each(|sample| {
                *sample /= max_amplitude;
            });
        }
    }

    pub fn apply_low_pass_filter(&self, audio_buffer: &mut [f32], cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_sample = audio_buffer[0];

        for sample in audio_buffer.iter_mut().skip(1) {
            *sample = prev_sample + alpha * (*sample - prev_sample);
            prev_sample = *sample;
        }
    }
}