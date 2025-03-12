use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// A simple audio buffer structure
struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    /// Create a new audio buffer with a given sample rate
    fn new(sample_rate: u32) -> Self {
        AudioBuffer {
            samples: Vec::new(),
            sample_rate,
        }
    }

    /// Add a sine wave to the buffer
    fn add_sine_wave(&mut self, frequency: f32, duration: f32) {
        let num_samples = (duration * self.sample_rate as f32) as usize;
        for i in 0..num_samples {
            let t = i as f32 / self.sample_rate as f32;
            let sample = (2.0 * PI * frequency * t).sin();
            self.samples.push(sample);
        }
    }

    /// Add white noise to the buffer
    fn add_white_noise(&mut self, amplitude: f32, duration: f32) {
        let num_samples = (duration * self.sample_rate as f32) as usize;
        for _ in 0..num_samples {
            let sample = (rand::random::<f32>() - 0.5) * 2.0 * amplitude;
            self.samples.push(sample);
        }
    }

    /// Apply a low-pass filter to the buffer
    fn apply_low_pass_filter(&mut self, cutoff_frequency: f32) {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let mut prev_sample = 0.0;
        for sample in &mut self.samples {
            let filtered_sample = prev_sample + alpha * (*sample - prev_sample);
            prev_sample = filtered_sample;
            *sample = filtered_sample;
        }
    }

    /// Normalize the audio buffer to a given peak amplitude
    fn normalize(&mut self, peak_amplitude: f32) {
        let max_sample = self
            .samples
            .iter()
            .fold(0.0, |max, &sample| sample.abs().max(max));
        if max_sample > 0.0 {
            let scale_factor = peak_amplitude / max_sample;
            for sample in &mut self.samples {
                *sample *= scale_factor;
            }
        }
    }

    /// Play the audio buffer using a separate thread
    fn play(&self) {
        let samples = Arc::new(self.samples.clone());
        let sample_rate = self.sample_rate;
        thread::spawn(move || {
            let mut audio_stream = cpal::default_host()
                .default_output_device()
                .unwrap()
                .default_output_format()
                .unwrap()
                .build_output_stream()
                .unwrap();
            let mut sample_index = 0;
            while sample_index < samples.len() {
                let sample = samples[sample_index];
                audio_stream.write(&[sample]).unwrap();
                sample_index += 1;
                thread::sleep(Duration::from_secs_f32(1.0 / sample_rate as f32));
            }
        });
    }
}

fn main() {
    let mut audio_buffer = AudioBuffer::new(44100);
    audio_buffer.add_sine_wave(440.0, 1.0);
    audio_buffer.add_white_noise(0.1, 1.0);
    audio_buffer.apply_low_pass_filter(1000.0);
    audio_buffer.normalize(0.5);
    audio_buffer.play();
}