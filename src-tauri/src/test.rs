use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::Mutex;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32) -> Self {
        AudioProcessor {
            sample_rate,
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn process_audio(&self, input: &[f32]) -> Vec<f32> {
        let mut output = Vec::with_capacity(input.len());
        let mut buffer = self.buffer.lock().unwrap();

        for &sample in input {
            let processed_sample = self.add_disturbance(sample);
            let filtered_sample = self.low_pass_filter(processed_sample, &mut buffer);
            output.push(filtered_sample);
        }

        output
    }

    fn add_disturbance(&self, sample: f32) -> f32 {
        let disturbance = ((2.0 * PI * 440.0 * sample) / self.sample_rate as f32).sin() * 0.1;
        sample + disturbance
    }

    fn low_pass_filter(&self, sample: f32, buffer: &mut Vec<f32>) -> f32 {
        const ALPHA: f32 = 0.1;
        let last_sample = buffer.last().copied().unwrap_or(0.0);
        let filtered_sample = ALPHA * sample + (1.0 - ALPHA) * last_sample;
        buffer.push(filtered_sample);
        filtered_sample
    }
}

pub fn optimize_buffer(buffer: &mut Vec<f32>) {
    buffer.shrink_to_fit();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processing() {
        let processor = AudioProcessor::new(44100);
        let input = vec![0.5, -0.3, 0.8, -0.1];
        let output = processor.process_audio(&input);
        assert_eq!(output.len(), input.len());
    }
}