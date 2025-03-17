use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

/// AudioBuffer represents a buffer of audio samples.
#[derive(Clone)]
pub struct AudioBuffer {
    samples: Arc<Vec<f32>>,
}

impl AudioBuffer {
    /// Create a new AudioBuffer with the given samples.
    pub fn new(samples: Vec<f32>) -> Self {
        Self {
            samples: Arc::new(samples),
        }
    }

    /// Process the audio buffer with a given processing function.
    pub fn process<F>(&self, processor: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        let processed_samples: Vec<f32> = self.samples.iter().map(|&sample| processor(sample)).collect();
        AudioBuffer::new(processed_samples)
    }

    /// Adds white noise to the audio buffer.
    pub fn add_white_noise(&self, amplitude: f32) -> Self {
        self.process(|sample| sample + amplitude * (rand::random::<f32>() - 0.5))
    }

    /// Applies a low-pass filter to the audio buffer.
    pub fn low_pass_filter(&self, cutoff_frequency: f32, sample_rate: f32) -> Self {
        let rc = 1.0 / (2.0 * PI * cutoff_frequency);
        let dt = 1.0 / sample_rate;
        let alpha = dt / (rc + dt);

        let mut filtered_samples = Vec::with_capacity(self.samples.len());
        let mut previous_sample = 0.0;

        for &sample in self.samples.iter() {
            let filtered_sample = alpha * sample + (1.0 - alpha) * previous_sample;
            filtered_samples.push(filtered_sample);
            previous_sample = filtered_sample;
        }

        AudioBuffer::new(filtered_samples)
    }
}

/// AudioProcessor handles concurrent audio processing.
pub struct AudioProcessor {
    buffer: AudioBuffer,
    stop_signal: Arc<AtomicBool>,
}

impl AudioProcessor {
    /// Create a new AudioProcessor with the given audio buffer.
    pub fn new(buffer: AudioBuffer) -> Self {
        Self {
            buffer,
            stop_signal: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start processing the audio buffer in a separate thread.
    pub fn start_processing(&self) {
        let buffer = self.buffer.clone();
        let stop_signal = Arc::clone(&self.stop_signal);

        thread::spawn(move || {
            let mut processed_buffer = buffer.add_white_noise(0.01);
            processed_buffer = processed_buffer.low_pass_filter(1000.0, 44100.0);

            while !stop_signal.load(Ordering::Relaxed) {
                // Continuously process audio in real-time
                processed_buffer = processed_buffer.low_pass_filter(1000.0, 44100.0);
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    /// Stop the audio processing thread.
    pub fn stop_processing(&self) {
        self.stop_signal.store(true, Ordering::Relaxed);
    }
}

fn main() {
    let samples = vec![0.0; 44100]; // 1 second of silence at 44.1kHz
    let buffer = AudioBuffer::new(samples);

    let processor = AudioProcessor::new(buffer.clone());
    processor.start_processing();

    // Simulate some processing time
    thread::sleep(Duration::from_secs(2));

    processor.stop_processing();
}