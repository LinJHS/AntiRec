use std::f32::consts::PI;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer_size: usize,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer_size: usize) -> Self {
        AudioProcessor {
            sample_rate,
            buffer_size,
        }
    }

    pub fn process_buffer(&self, buffer: &mut [f32], frequency: f32, noise_level: f32) {
        let angular_frequency = 2.0 * PI * frequency / self.sample_rate as f32;
        let noise_amplitude = noise_level * 0.1;

        for (i, sample) in buffer.iter_mut().enumerate() {
            let phase = angular_frequency * i as f32;
            let signal = phase.sin();
            let noise = (rand::random::<f32>() * 2.0 - 1.0) * noise_amplitude;
            *sample = signal + noise;
        }
    }

    pub fn normalize_buffer(&self, buffer: &mut [f32]) {
        let max_amplitude = buffer.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in buffer.iter_mut() {
                *sample /= max_amplitude;
            }
        }
    }
}

pub fn add_reverb(buffer: &mut [f32], delay_samples: usize, decay: f32) {
    let mut delay_buffer = vec![0.0; buffer.len() + delay_samples];

    for i in 0..buffer.len() {
        delay_buffer[i] += buffer[i];
        if i + delay_samples < delay_buffer.len() {
            delay_buffer[i + delay_samples] += buffer[i] * decay;
        }
    }

    for i in 0..buffer.len() {
        buffer[i] = delay_buffer[i];
    }
}