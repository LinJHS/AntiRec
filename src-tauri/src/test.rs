use std::f32::consts::PI;

/// A struct to represent audio processing parameters
struct AudioProcessor {
    sample_rate: u32,
    amplitude: f32,
    frequency: f32,
    disturbance_level: f32,
}

impl AudioProcessor {
    /// Creates a new AudioProcessor instance
    fn new(sample_rate: u32, amplitude: f32, frequency: f32, disturbance_level: f32) -> Self {
        AudioProcessor {
            sample_rate,
            amplitude,
            frequency,
            disturbance_level,
        }
    }

    /// Generates a sine wave with added disturbance
    fn generate_wave(&self, duration: f32) -> Vec<f32> {
        let num_samples = (duration * self.sample_rate as f32) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / self.sample_rate as f32;
            let sine_wave = self.amplitude * (2.0 * PI * self.frequency * t).sin();
            let disturbance = self.disturbance_level * (2.0 * PI * 2.0 * self.frequency * t).sin();
            samples.push(sine_wave + disturbance);
        }

        samples
    }

    /// Applies a low-pass filter to the samples
    fn apply_low_pass_filter(&self, samples: &mut [f32], cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_output = samples[0];

        for sample in samples.iter_mut() {
            *sample = prev_output + alpha * (*sample - prev_output);
            prev_output = *sample;
        }
    }
}

fn main() {
    let processor = AudioProcessor::new(44100, 1.0, 440.0, 0.1);
    let mut samples = processor.generate_wave(1.0);
    processor.apply_low_pass_filter(&mut samples, 1000.0);

    // Output the processed samples (for demonstration purposes)
    for sample in samples {
        println!("{}", sample);
    }
}