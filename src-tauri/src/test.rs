use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use dasp::signal::{self, Signal};

struct AudioProcessor {
    sample_rate: u32,
    buffer_size: usize,
}

impl AudioProcessor {
    fn new(sample_rate: u32, buffer_size: usize) -> Self {
        AudioProcessor {
            sample_rate,
            buffer_size,
        }
    }

    fn process(&self, input: &[f32]) -> Vec<f32> {
        let mut output = Vec::with_capacity(self.buffer_size);
        let mut phase = 0.0;

        for &sample in input {
            let processed_sample = self.add_disturbance(sample, phase);
            output.push(processed_sample);
            phase = (phase + 2.0 * PI * 440.0 / self.sample_rate as f32) % (2.0 * PI);
        }

        output
    }

    fn add_disturbance(&self, sample: f32, phase: f32) -> f32 {
        let disturbance = 0.1 * (2.0 * PI * 440.0 * phase).sin();
        sample + disturbance
    }
}

fn main() {
    let processor = Arc::new(AudioProcessor::new(44100, 1024));
    let input_signal = signal::rate(44100).const_hz(440.0).sine();

    let handles: Vec<_> = (0..4).map(|_| {
        let processor = Arc::clone(&processor);
        let input_signal = input_signal.clone();
        thread::spawn(move || {
            let mut buffer = Vec::with_capacity(1024);
            for sample in input_signal.take(1024) {
                buffer.push(sample);
            }
            processor.process(&buffer)
        })
    }).collect();

    for handle in handles {
        let _ = handle.join().unwrap();
    }
}