use std::f32::consts::PI;

struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { samples, sample_rate }
    }

    fn add_noise(&mut self, noise_level: f32) {
        for sample in &mut self.samples {
            let noise = (fastrand::f32() - 0.5) * 2.0 * noise_level;
            *sample += noise;
        }
    }

    fn apply_low_pass_filter(&mut self, cutoff_freq: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_output = 0.0;
        for sample in &mut self.samples {
            *sample = alpha * *sample + (1.0 - alpha) * prev_output;
            prev_output = *sample;
        }
    }

    fn rms(&self) -> f32 {
        let sum_squares: f32 = self.samples.iter().map(|&s| s * s).sum();
        (sum_squares / self.samples.len() as f32).sqrt()
    }

    fn normalize(&mut self) {
        let max_amplitude = self.samples.iter().fold(0.0, |acc, &s| acc.max(s.abs()));
        if max_amplitude > 0.0 {
            for sample in &mut self.samples {
                *sample /= max_amplitude;
            }
        }
    }
}

fn main() {
    let samples = vec![0.1, 0.3, 0.5, 0.7, 0.9];
    let mut audio_buffer = AudioBuffer::new(samples, 44100);

    audio_buffer.add_noise(0.05);
    audio_buffer.apply_low_pass_filter(1000.0);
    audio_buffer.normalize();

    println!("RMS: {}", audio_buffer.rms());
}