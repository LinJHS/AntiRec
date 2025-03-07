use std::f32::consts::PI;
use rand::Rng;

#[derive(Debug)]
pub struct AudioProcessor {
    sample_rate: u32,
    buffer: Vec<f32>,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer: Vec<f32>) -> Self {
        Self { sample_rate, buffer }
    }

    pub fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut rng = rand::thread_rng();
        for i in 0..self.buffer.len() {
            let t = i as f32 / self.sample_rate as f32;
            let noise = amplitude * rng.gen_range(-1.0..1.0);
            let disturbance = amplitude * (2.0 * PI * frequency * t).sin();
            self.buffer[i] += noise + disturbance;
        }
    }

    pub fn normalize(&mut self) {
        let max_amplitude = self.buffer.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in self.buffer.iter_mut() {
                *sample /= max_amplitude;
            }
        }
    }

    pub fn process(&mut self) {
        self.add_disturbance(440.0, 0.1);
        self.normalize();
    }

    pub fn get_buffer(&self) -> &Vec<f32> {
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_disturbance() {
        let mut processor = AudioProcessor::new(44100, vec![0.0; 44100]);
        processor.add_disturbance(440.0, 0.1);
        assert!(processor.get_buffer().iter().any(|&x| x != 0.0));
    }

    #[test]
    fn test_normalize() {
        let mut processor = AudioProcessor::new(44100, vec![1.0, 2.0, 3.0]);
        processor.normalize();
        assert_eq!(processor.get_buffer(), &vec![1.0 / 3.0, 2.0 / 3.0, 1.0]);
    }
}