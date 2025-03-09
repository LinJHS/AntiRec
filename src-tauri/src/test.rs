use std::f32::consts::PI;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer: Vec<f32>,
    noise_level: f32,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer_size: usize, noise_level: f32) -> Self {
        Self {
            sample_rate,
            buffer: vec![0.0; buffer_size],
            noise_level,
        }
    }

    pub fn process(&mut self) -> &[f32] {
        let noise_amplitude = self.noise_level / 100.0;
        let phase_increment = 2.0 * PI * 440.0 / self.sample_rate as f32;

        for i in 0..self.buffer.len() {
            let sample = (i as f32 * phase_increment).sin();
            let noise = noise_amplitude * (rand::random::<f32>() - 0.5);
            self.buffer[i] = sample + noise;
        }

        &self.buffer
    }

    pub fn apply_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let phase_increment = 2.0 * PI * frequency / self.sample_rate as f32;

        for i in 0..self.buffer.len() {
            let disturbance = amplitude * (i as f32 * phase_increment).sin();
            self.buffer[i] += disturbance;
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

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|x| *x = 0.0);
    }
}