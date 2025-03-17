use std::f32::consts::PI;

// Audio processing module
mod audio_processing {
    pub struct AudioBuffer {
        samples: Vec<f32>,
        sample_rate: u32,
    }

    impl AudioBuffer {
        pub fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
            AudioBuffer { samples, sample_rate }
        }

        pub fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
            for (i, sample) in self.samples.iter_mut().enumerate() {
                let t = i as f32 / self.sample_rate as f32;
                *sample += amplitude * (2.0 * PI * frequency * t).sin();
            }
        }

        pub fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
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

        pub fn normalize(&mut self) {
            let max_amplitude = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
            if max_amplitude > 0.0 {
                for sample in &mut self.samples {
                    *sample /= max_amplitude;
                }
            }
        }
    }
}

// Efficient audio processing pipeline
fn process_audio(samples: Vec<f32>, sample_rate: u32) -> Vec<f32> {
    let mut buffer = audio_processing::AudioBuffer::new(samples, sample_rate);
    buffer.add_disturbance(50.0, 0.1); // Add 50Hz disturbance
    buffer.apply_low_pass_filter(1000.0); // Apply low-pass filter
    buffer.normalize(); // Normalize audio
    buffer.samples
}

fn main() {
    let samples = vec![0.0, 0.5, 1.0, -1.0, 0.0];
    let sample_rate = 44100;
    let processed_samples = process_audio(samples, sample_rate);
    println!("Processed Samples: {:?}", processed_samples);
}