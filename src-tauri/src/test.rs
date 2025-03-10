use std::f32::consts::PI;

#[derive(Debug)]
struct AudioSignal {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioSignal {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioSignal { samples, sample_rate }
    }

    fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut rng = rand::thread_rng();
        for i in 0..self.samples.len() {
            let t = i as f32 / self.sample_rate as f32;
            let disturbance = amplitude * (2.0 * PI * frequency * t).sin();
            self.samples[i] += disturbance + rng.gen_range(-0.05..0.05);
        }
    }

    fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);
        let mut prev_output = self.samples[0];

        for sample in self.samples.iter_mut() {
            *sample = prev_output + alpha * (*sample - prev_output);
            prev_output = *sample;
        }
    }

    fn normalize(&mut self) {
        let max_amplitude = normalize_max_amplitude(&self.samples);
        for sample in self.samples.iter_mut() {
            *sample /= max_amplitude;
        }
    }
}

fn normalize_max_amplitude(samples: &[f32]) -> f32 {
    samples.iter().fold(0.0, |max, &x| max.max(x.abs()))
}

fn main() {
    let mut audio_signal = AudioSignal::new(vec![0.0; 44100], 44100);
    audio_signal.add_disturbance(1000.0, 0.1);
    audio_signal.apply_low_pass_filter(5000.0);
    audio_signal.normalize();

    println!("{:?}", audio_signal);
}