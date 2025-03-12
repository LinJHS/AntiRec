use std::f32::consts::PI;
use rayon::prelude::*;

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
        self.samples.par_iter_mut().enumerate().for_each(|(i, sample)| {
            let time = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * time).sin();
            *sample += disturbance;
        });
    }

    fn normalize(&mut self) {
        let max_amplitude = self.samples.par_iter().fold(|| 0.0, |acc, &x| acc.max(x.abs())).max();
        if max_amplitude > 0.0 {
            self.samples.par_iter_mut().for_each(|sample| {
                *sample /= max_amplitude;
            });
        }
    }

    fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_output = 0.0;

        for sample in &mut self.samples {
            let output = prev_output + alpha * (*sample - prev_output);
            prev_output = output;
            *sample = output;
        }
    }
}

fn main() {
    let sample_rate = 44100;
    let mut audio_buffer = AudioBuffer::new(vec![0.0; 44100], sample_rate);

    audio_buffer.add_disturbance(440.0, 0.5);
    audio_buffer.apply_low_pass_filter(1000.0);
    audio_buffer.normalize();

    println!("{:?}", audio_buffer);
}