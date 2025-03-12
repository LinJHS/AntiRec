use std::f32::consts::PI;

#[derive(Debug)]
pub struct AudioProcessor {
    sample_rate: u32,
    buffer_size: usize,
    disturbance_level: f32,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer_size: usize, disturbance_level: f32) -> Self {
        AudioProcessor {
            sample_rate,
            buffer_size,
            disturbance_level,
        }
    }

    pub fn process_audio(&self, input: &[f32]) -> Vec<f32> {
        let mut output = Vec::with_capacity(input.len());
        let disturbance = self.generate_disturbance(input.len());

        for (i, &sample) in input.iter().enumerate() {
            let processed_sample = sample + disturbance[i];
            output.push(processed_sample);
        }

        output
    }

    fn generate_disturbance(&self, length: usize) -> Vec<f32> {
        let mut disturbance = Vec::with_capacity(length);
        let frequency = 440.0; // A4 note frequency
        let amplitude = self.disturbance_level;

        for i in 0..length {
            let t = i as f32 / self.sample_rate as f32;
            let value = amplitude * (2.0 * PI * frequency * t).sin();
            disturbance.push(value);
        }

        disturbance
    }

    pub fn apply_low_pass_filter(&self, input: &[f32], cutoff_frequency: f32) -> Vec<f32> {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut filtered = Vec::with_capacity(input.len());
        let mut prev_output = 0.0;

        for &sample in input {
            let output = prev_output + alpha * (sample - prev_output);
            filtered.push(output);
            prev_output = output;
        }

        filtered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processing() {
        let processor = AudioProcessor::new(44100, 1024, 0.1);
        let input = vec![0.0; 1024];
        let output = processor.process_audio(&input);

        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_low_pass_filter() {
        let processor = AudioProcessor::new(44100, 1024, 0.1);
        let input = vec![1.0; 1024];
        let filtered = processor.apply_low_pass_filter(&input, 1000.0);

        assert_eq!(filtered.len(), input.len());
    }
}