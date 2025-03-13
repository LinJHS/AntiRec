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

    pub fn process_buffer(&self, buffer: &mut [f32]) {
        let frequency = 440.0; // A4 note frequency
        let amplitude = 0.5;
        let noise_level = 0.02;

        for i in 0..self.buffer_size {
            let sample = amplitude * (2.0 * PI * frequency * i as f32 / self.sample_rate as f32).sin();
            let noise = noise_level * (rand::random::<f32>() - 0.5);
            buffer[i] = sample + noise;
        }
    }

    pub fn apply_disturbance(&self, buffer: &mut [f32], disturbance: f32) {
        for sample in buffer.iter_mut() {
            *sample += disturbance * (rand::random::<f32>() - 0.5);
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

pub fn efficient_processing(buffer: &mut [f32], sample_rate: u32) {
    let processor = AudioProcessor::new(sample_rate, buffer.len());
    processor.process_buffer(buffer);
    processor.apply_disturbance(buffer, 0.05);
    processor.normalize_buffer(buffer);
}