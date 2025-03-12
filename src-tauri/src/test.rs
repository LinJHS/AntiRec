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

    pub fn process_audio(&mut self, input: &[f32]) -> Vec<f32> {
        self.buffer.clear();
        self.buffer.extend(input.iter().map(|&sample| {
            let disturbed_sample = self.add_disturbance(sample);
            self.apply_low_pass_filter(disturbed_sample)
        }));
        self.buffer.clone()
    }

    fn add_disturbance(&self, sample: f32) -> f32 {
        let noise = (2.0 * PI * 440.0 * sample / self.sample_rate as f32).sin() * 0.1;
        sample + noise
    }

    fn apply_low_pass_filter(&self, sample: f32) -> f32 {
        let alpha = 0.1;
        let mut filtered_sample = 0.0;
        filtered_sample = alpha * sample + (1.0 - alpha) * filtered_sample;
        filtered_sample
    }
}

pub fn optimize_audio_processing(input: &[f32], sample_rate: u32) -> Vec<f32> {
    let mut processor = AudioProcessor::new(sample_rate);
    processor.process_audio(input)
}