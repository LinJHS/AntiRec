use std::f32::consts::PI;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct AudioSample {
    pub left: f32,
    pub right: f32,
}

pub struct AudioBuffer {
    pub samples: Vec<AudioSample>,
    pub sample_rate: u32,
}

impl AudioBuffer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            samples: Vec::new(),
            sample_rate,
        }
    }

    pub fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut rng = rand::thread_rng();
        for sample in &mut self.samples {
            let noise = rng.gen_range(-amplitude..amplitude);
            let phase = 2.0 * PI * frequency / self.sample_rate as f32;
            sample.left += noise * phase.sin();
            sample.right += noise * phase.cos();
        }
    }

    pub fn apply_equalizer(&mut self, frequencies: &[f32], gains: &[f32]) {
        assert!(frequencies.len() == gains.len(), "Frequencies and gains must have the same length");

        for (i, sample) in self.samples.iter_mut().enumerate() {
            let mut eq_left = 0.0;
            let mut eq_right = 0.0;

            for (j, &freq) in frequencies.iter().enumerate() {
                let gain = gains[j];
                let phase = 2.0 * PI * freq * i as f32 / self.sample_rate as f32;
                eq_left += gain * phase.sin();
                eq_right += gain * phase.cos();
            }

            sample.left *= eq_left;
            sample.right *= eq_right;
        }
    }

    pub fn normalize(&mut self) {
        let max_amplitude = self.samples.iter()
            .map(|s| s.left.abs().max(s.right.abs()))
            .fold(0.0, |a, b| a.max(b));

        if max_amplitude > 0.0 {
            for sample in &mut self.samples {
                sample.left /= max_amplitude;
                sample.right /= max_amplitude;
            }
        }
    }
}