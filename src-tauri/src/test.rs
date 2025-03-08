use std::f32::consts::PI;

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

    pub fn process_audio(&mut self, input: &[f32], gain: f32, noise_level: f32) -> Vec<f32> {
        let mut output = Vec::with_capacity(input.len());
        for &sample in input {
            let processed_sample = sample * gain;
            let noise = noise_level * (2.0 * fastrand::f32() - 1.0);
            output.push(processed_sample + noise);
        }
        self.buffer = output.clone();
        output
    }

    pub fn apply_high_pass_filter(&mut self, cutoff_frequency: f32) -> Vec<f32> {
        let alpha = 1.0 / (1.0 + (2.0 * PI * cutoff_frequency / self.sample_rate as f32));
        let mut filtered_buffer = Vec::with_capacity(self.buffer.len());
        let mut prev_sample = 0.0;
        for &sample in &self.buffer {
            let filtered_sample = alpha * (prev_sample + sample - filtered_buffer.last().unwrap_or(&0.0));
            filtered_buffer.push(filtered_sample);
            prev_sample = sample;
        }
        self.buffer = filtered_buffer.clone();
        filtered_buffer
    }

    pub fn normalize_audio(&mut self, target_level: f32) -> Vec<f32> {
        let max_sample = self.buffer.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        let normalization_factor = target_level / max_sample;
        let normalized_buffer: Vec<f32> = self.buffer.iter().map(|&x| x * normalization_factor).collect();
        self.buffer = normalized_buffer.clone();
        normalized_buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processing() {
        let mut processor = AudioProcessor::new(44100);
        let input = vec![0.5, -0.3, 0.8];
        let output = processor.process_audio(&input, 1.5, 0.1);
        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn test_high_pass_filter() {
        let mut processor = AudioProcessor::new(44100);
        processor.buffer = vec![0.5, -0.3, 0.8];
        let filtered = processor.apply_high_pass_filter(1000.0);
        assert_eq!(filtered.len(), processor.buffer.len());
    }

    #[test]
    fn test_normalize_audio() {
        let mut processor = AudioProcessor::new(44100);
        processor.buffer = vec![0.5, -0.3, 0.8];
        let normalized = processor.normalize_audio(1.0);
        assert_eq!(normalized.len(), processor.buffer.len());
    }
}