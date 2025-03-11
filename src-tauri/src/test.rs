use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
struct AudioBuffer {
    samples: Arc<Mutex<Vec<f32>>>,
}

impl AudioBuffer {
    fn new(size: usize) -> Self {
        AudioBuffer {
            samples: Arc::new(Mutex::new(vec![0.0; size])),
        }
    }

    fn add_disturbance(&self, frequency: f32, amplitude: f32) {
        let mut samples = self.samples.lock().unwrap();
        for (i, sample) in samples.iter_mut().enumerate() {
            let t = i as f32 / 44100.0;
            *sample += amplitude * (2.0 * PI * frequency * t).sin();
        }
    }

    fn apply_compression(&self, threshold: f32, ratio: f32) {
        let mut samples = self.samples.lock().unwrap();
        for sample in samples.iter_mut() {
            if *sample.abs() > threshold {
                *sample = threshold + (*sample.abs() - threshold) / ratio * (*sample).signum();
            }
        }
    }

    fn normalize(&self) {
        let mut samples = self.samples.lock().unwrap();
        let max_amplitude = samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in samples.iter_mut() {
                *sample /= max_amplitude;
            }
        }
    }
}

fn process_audio(buffer: AudioBuffer) {
    buffer.add_disturbance(440.0, 0.1);
    buffer.apply_compression(0.5, 4.0);
    buffer.normalize();
}

fn main() {
    let buffer = AudioBuffer::new(44100);
    process_audio(buffer);
}