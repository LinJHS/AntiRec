use std::f32::consts::PI;

/// A struct representing an audio signal
#[derive(Debug)]
struct AudioSignal {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioSignal {
    /// Create a new AudioSignal with given samples and sample rate
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioSignal { samples, sample_rate }
    }

    /// Add white noise to the audio signal
    fn add_white_noise(&mut self, noise_level: f32) {
        self.samples.iter_mut().for_each(|sample| {
            let noise = (rand::random::<f32>() - 0.5) * noise_level;
            *sample += noise;
        });
    }

    /// Apply a low-pass filter to the audio signal
    fn apply_low_pass_filter(&mut self, cutoff_freq: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_freq);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_sample = self.samples[0];
        for sample in self.samples.iter_mut() {
            *sample = prev_sample + alpha * (*sample - prev_sample);
            prev_sample = *sample;
        }
    }

    /// Normalize the audio signal to the range [-1.0, 1.0]
    fn normalize(&mut self) {
        let max_abs = self.samples.iter()
                                           .map(|&x| x.abs())
                                           .fold(0.0, |a, b| a.max(b));
        if max_abs > 0.0 {
            self.samples.iter_mut().for_each(|x| *x /= max_abs);
        }
    }
}

/// Generate a sine wave audio signal
fn generate_sine_wave(freq: f32, duration: f32, sample_rate: u32) -> AudioSignal {
    let num_samples = (duration * sample_rate as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let phase_inc = 2.0 * PI * freq / sample_rate as f32;

    for i in 0..num_samples {
        samples.push((phase_inc * i as f32).sin());
    }

    AudioSignal::new(samples, sample_rate)
}

fn main() {
    let mut sine_wave = generate_sine_wave(440.0, 1.0, 44100);
    sine_wave.add_white_noise(0.1);
    sine_wave.apply_low_pass_filter(1000.0);
    sine_wave.normalize();

    println!("Processed audio signal: {:?}", sine_wave);
}