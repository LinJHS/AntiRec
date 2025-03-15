use std::f32::consts::PI;
use rand::Rng;

#[derive(Debug)]
struct AudioSample {
    data: Vec<f32>,
    sample_rate: u32,
}

impl AudioSample {
    fn new(data: Vec<f32>, sample_rate: u32) -> Self {
        AudioSample { data, sample_rate }
    }

    fn add_noise(&mut self, noise_level: f32) {
        let mut rng = rand::thread_rng();
        for sample in &mut self.data {
            let noise = rng.gen_range(-noise_level..noise_level);
            *sample += noise;
        }
    }

    fn apply_low_pass_filter(&mut self, cutoff_freq: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_output = 0.0;
        for sample in &mut self.data {
            *sample = alpha * *sample + (1.0 - alpha) * prev_output;
            prev_output = *sample;
        }
    }

    fn normalize(&mut self) {
        let max_amplitude = self.data.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 0.0 {
            for sample in &mut self.data {
                *sample /= max_amplitude;
            }
        }
    }
}

fn main() {
    let sample_data = vec![0.1, 0.5, 0.8, 0.3, 0.7];
    let mut audio_sample = AudioSample::new(sample_data, 44100);

    audio_sample.add_noise(0.05);
    audio_sample.apply_low_pass_filter(1000.0);
    audio_sample.normalize();

    println!("{:?}", audio_sample);
}