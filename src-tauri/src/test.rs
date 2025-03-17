use std::f32::consts::PI;
use rayon::prelude::*;

#[derive(Debug)]
struct AudioBuffer {
    sample_rate: u32,
    data: Vec<f32>,
}

impl AudioBuffer {
    fn new(sample_rate: u32, data: Vec<f32>) -> Self {
        Self { sample_rate, data }
    }

    fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        self.data.par_iter_mut().enumerate().for_each(|(i, sample)| {
            let time = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * time).sin();
            *sample += disturbance;
        });
    }

    fn normalize(&mut self) {
        let max_amplitude = self.data.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            self.data.par_iter_mut().for_each(|sample| {
                *sample /= max_amplitude;
            });
        }
    }

    fn apply_high_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = rc / (rc + dt);

        let mut prev_sample = self.data[0];
        self.data[0] = 0.0;

        for i in 1..self.data.len() {
            let current_sample = self.data[i];
            self.data[i] = alpha * (prev_sample + self.data[i] - current_sample);
            prev_sample = current_sample;
        }
    }

    fn rms(&self) -> f32 {
        let sum_squares: f32 = self.data.par_iter().map(|x| x * x).sum();
        (sum_squares / self.data.len() as f32).sqrt()
    }
}

fn main() {
    let sample_rate = 44100;
    let mut audio_buffer = AudioBuffer::new(sample_rate, vec![0.0; sample_rate]);

    audio_buffer.add_disturbance(1000.0, 0.1);
    audio_buffer.apply_high_pass_filter(500.0);
    audio_buffer.normalize();

    println!("RMS: {}", audio_buffer.rms());
}