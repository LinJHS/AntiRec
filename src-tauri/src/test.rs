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

    pub fn process_audio(&self, input: &[f32], output: &mut [f32], gain: f32, noise_level: f32) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
        buffer.extend(input.iter());

        for (i, sample) in buffer.iter().enumerate() {
            let modulated = *sample * gain;
            let noise = (2.0 * PI * i as f32 / self.sample_rate as f32).sin() * noise_level;
            output[i] = modulated + noise;
        }
    }

    pub fn apply_disturbance(&self, input: &mut [f32], frequency: f32, amplitude: f32) {
        for (i, sample) in input.iter_mut().enumerate() {
            let disturbance = amplitude * (2.0 * PI * frequency * i as f32 / self.sample_rate as f32).sin();
            *sample += disturbance;
        }
    }

    pub fn normalize_audio(&self, input: &mut [f32]) {
        let max_amplitude = input.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in input.iter_mut() {
                *sample /= max_amplitude;
            }
        }
    }
}