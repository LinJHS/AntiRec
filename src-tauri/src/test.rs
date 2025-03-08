use std::f32::consts::PI;
use rand::Rng;

#[derive(Debug)]
struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { samples, sample_rate }
    }

    fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut rng = rand::thread_rng();
        for i in 0..self.samples.len() {
            let time = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * time).sin();
            self.samples[i] += disturbance + rng.gen_range(-0.01..0.01); // Adding random noise
        }
    }

    fn normalize(&mut self) {
        let max_amplitude = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in &mut self.samples {
                *sample /= max_amplitude;
            }
        }
    }

    fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_filtered = self.samples[0];
        for sample in &mut self.samples {
            let filtered = alpha * *sample + (1.0 - alpha) * prev_filtered;
            prev_filtered = filtered;
            *sample = filtered;
        }
    }
}

fn main() {
    let mut audio_buffer = AudioBuffer::new(vec![0.0; 44100], 44100);
    audio_buffer.add_disturbance(440.0, 0.5);
    audio_buffer.normalize();
    audio_buffer.apply_low_pass_filter(1000.0);

    println!("Processed Audio Buffer: {:?}", audio_buffer);
}