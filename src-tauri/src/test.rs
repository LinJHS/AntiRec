use std::f32::consts::PI;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer: Vec<f32>,
    noise_level: f32,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer: Vec<f32>, noise_level: f32) -> Self {
        AudioProcessor {
            sample_rate,
            buffer,
            noise_level,
        }
    }

    pub fn add_disturbance(&mut self) {
        let noise_amplitude = self.noise_level * 0.01;
        let phase_shift = 0.5 * PI;

        for sample in &mut self.buffer {
            let noise = noise_amplitude * (phase_shift * *sample).sin();
            *sample += noise;
        }
    }

    pub fn apply_low_pass_filter(&mut self, cutoff_freq: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_output = 0.0;
        for sample in &mut self.buffer {
            *sample = alpha * *sample + (1.0 - alpha) * prev_output;
            prev_output = *sample;
        }
    }

    pub fn normalize(&mut self) {
        let max_amplitude = self.buffer.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in &mut self.buffer {
                *sample /= max_amplitude;
            }
        }
    }

    pub fn get_buffer(&self) -> &Vec<f32> {
        &self.buffer
    }
}