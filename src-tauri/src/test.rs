use std::f32::consts::PI;
use rand::Rng;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer: Vec<f32>,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32) -> Self {
        AudioProcessor {
            sample_rate,
            buffer: Vec::new(),
        }
    }

    pub fn process(&mut self, input: &[f32]) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        let mut output = Vec::with_capacity(input.len());

        for &sample in input {
            let noise: f32 = rng.gen_range(-0.1..0.1);
            let processed_sample = sample + noise;
            output.push(processed_sample.clamp(-1.0, 1.0));
        }

        output
    }

    pub fn apply_filter(&mut self, input: &[f32], cutoff_freq: f32) -> Vec<f32> {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut filtered = Vec::with_capacity(input.len());
        let mut prev_output = 0.0;

        for &sample in input {
            let output = alpha * sample + (1.0 - alpha) * prev_output;
            filtered.push(output);
            prev_output = output;
        }

        filtered
    }

    pub fn normalize(&mut self, input: &[f32]) -> Vec<f32> {
        let max_amplitude = input.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude == 0.0 {
            return input.to_vec();
        }

        input.iter().map(|&x| x / max_amplitude).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let mut processor = AudioProcessor::new(44100);
        let input = vec![0.5, -0.5, 0.0];
        let output = processor.process(&input);

        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_apply_filter() {
        let mut processor = AudioProcessor::new(44100);
        let input = vec![0.5, -0.5, 0.0];
        let output = processor.apply_filter(&input, 1000.0);

        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_normalize() {
        let mut processor = AudioProcessor::new(44100);
        let input = vec![0.5, -0.5, 0.0];
        let output = processor.normalize(&input);

        assert_eq!(output.len(), input.len());
        assert!(output.iter().all(|&x| x.abs() <= 1.0));
    }
}