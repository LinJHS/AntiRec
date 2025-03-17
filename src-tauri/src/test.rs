use std::collections::VecDeque;
use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44100;
const BUFFER_SIZE: usize = 1024;

pub struct AudioProcessor {
    buffer: VecDeque<f32>,
    disturbance_level: f32,
}

impl AudioProcessor {
    pub fn new(disturbance_level: f32) -> Self {
        Self {
            buffer: VecDeque::with_capacity(BUFFER_SIZE),
            disturbance_level,
        }
    }

    pub fn process_audio(&mut self, input: &[f32]) -> Vec<f32> {
        let mut output = Vec::with_capacity(input.len());

        for &sample in input {
            let disturbed_sample = self.apply_disturbance(sample);
            self.buffer.push_back(disturbed_sample);
            if self.buffer.len() >= BUFFER_SIZE {
                let processed_sample = self.process_buffer();
                output.push(processed_sample);
                self.buffer.pop_front();
            }
        }

        output
    }

    fn apply_disturbance(&self, sample: f32) -> f32 {
        let noise = (2.0 * PI * self.disturbance_level * sample).sin();
        sample + noise
    }

    fn process_buffer(&self) -> f32 {
        self.buffer.iter().sum::<f32>() / self.buffer.len() as f32
    }
}

pub fn process_audio_stream(input: &[f32], disturbance_level: f32) -> Vec<f32> {
    let mut processor = AudioProcessor::new(disturbance_level);
    processor.process_audio(input)
}