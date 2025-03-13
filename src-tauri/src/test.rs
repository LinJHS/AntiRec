use std::f32::consts::PI;
use rand::Rng;

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

    pub fn process_audio(&self, input: &[f32], disturbance_level: f32) -> Vec<f32> {
        let mut output = Vec::with_capacity(self.buffer_size);
        let mut rng = rand::thread_rng();

        for &sample in input {
            let noise = rng.gen_range(-disturbance_level..disturbance_level);
            let processed_sample = sample + noise;
            output.push(processed_sample.clamp(-1.0, 1.0));
        }

        output
    }

    pub fn apply_low_pass_filter(&self, input: &[f32], cutoff_freq: f32) -> Vec<f32> {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut filtered_output = Vec::with_capacity(input.len());
        let mut prev_output = 0.0;

        for &sample in input {
            let output = alpha * sample + (1.0 - alpha) * prev_output;
            filtered_output.push(output);
            prev_output = output;
        }

        filtered_output
    }

    pub fn optimize_buffer(&self, input: &[f32]) -> Vec<f32> {
        input.chunks(self.buffer_size)
            .flat_map(|chunk| chunk.iter().map(|&sample| sample.round()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processing() {
        let processor = AudioProcessor::new(44100, 1024);
        let input = vec![0.5, -0.3, 0.8];
        let output = processor.process_audio(&input, 0.1);
        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_low_pass_filter() {
        let processor = AudioProcessor::new(44100, 1024);
        let input = vec![0.5, -0.3, 0.8];
        let output = processor.apply_low_pass_filter(&input, 1000.0);
        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_optimize_buffer() {
        let processor = AudioProcessor::new(44100, 1024);
        let input = vec![0.5, -0.3, 0.8];
        let output = processor.optimize_buffer(&input);
        assert_eq!(output.len(), input.len());
    }
}