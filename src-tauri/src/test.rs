use std::f32::consts::PI;
use rand::Rng;

struct AudioProcessor {
    sample_rate: u32,
    buffer: Vec<f32>,
}

impl AudioProcessor {
    fn new(sample_rate: u32, buffer_size: usize) -> Self {
        Self {
            sample_rate,
            buffer: vec![0.0; buffer_size],
        }
    }

    fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut rng = rand::thread_rng();
        for i in 0..self.buffer.len() {
            let t = i as f32 / self.sample_rate as f32;
            let noise = rng.gen_range(-amplitude..amplitude);
            let sine_wave = (2.0 * PI * frequency * t).sin();
            self.buffer[i] += sine_wave * amplitude + noise;
        }
    }

    fn normalize(&mut self) {
        let max_value = self.buffer.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_value > 0.0 {
            for sample in &mut self.buffer {
                *sample /= max_value;
            }
        }
    }

    fn process(&mut self, frequency: f32, amplitude: f32) -> &[f32] {
        self.add_disturbance(frequency, amplitude);
        self.normalize();
        &self.buffer
    }
}

fn main() {
    let sample_rate = 44100;
    let buffer_size = 1024;
    let mut processor = AudioProcessor::new(sample_rate, buffer_size);
    let processed_audio = processor.process(440.0, 0.5);
    println!("Processed audio: {:?}", processed_audio);
}