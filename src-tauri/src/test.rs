use std::f32::consts::PI;

pub struct AudioProcessor {
    sample_rate: f32,
    buffer: Vec<f32>,
    noise_level: f32,
}

impl AudioProcessor {
    pub fn new(sample_rate: f32, buffer_size: usize, noise_level: f32) -> Self {
        Self {
            sample_rate,
            buffer: vec![0.0; buffer_size],
            noise_level,
        }
    }

    pub fn process(&mut self, input: &[f32]) -> Vec<f32> {
        self.buffer.copy_from_slice(input);

        // Apply a simple low-pass filter
        let alpha = 0.1;
        for i in 1..self.buffer.len() {
            self.buffer[i] = alpha * self.buffer[i] + (1.0 - alpha) * self.buffer[i - 1];
        }

        // Add noise disturbance
        for sample in &mut self.buffer {
            *sample += self.noise_level * (rand::random::<f32>() * 2.0 - 1.0);
        }

        // Normalize audio to prevent clipping
        let max_amplitude = self.buffer.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in &mut self.buffer {
                *sample /= max_amplitude;
            }
        }

        self.buffer.clone()
    }

    pub fn generate_sine_wave(&self, frequency: f32, duration: f32) -> Vec<f32> {
        let num_samples = (duration * self.sample_rate) as usize;
        let mut sine_wave = vec![0.0; num_samples];

        for i in 0..num_samples {
            sine_wave[i] = (2.0 * PI * frequency * i as f32 / self.sample_rate).sin();
        }

        sine_wave
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processing() {
        let mut processor = AudioProcessor::new(44100.0, 1024, 0.01);
        let input = vec![0.0; 1024];
        let output = processor.process(&input);
        assert_eq!(output.len(), 1024);
    }

    #[test]
    fn test_sine_wave_generation() {
        let processor = AudioProcessor::new(44100.0, 1024, 0.01);
        let sine_wave = processor.generate_sine_wave(440.0, 0.1);
        assert_eq!(sine_wave.len(), 4410);
    }
}