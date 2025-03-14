use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Represents an audio buffer with methods for processing
#[derive(Clone)]
struct AudioBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { samples, sample_rate }
    }

    /// Applies a sine wave disturbance to the audio buffer
    fn apply_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let mut phase = 0.0;
        let phase_increment = 2.0 * PI * frequency / self.sample_rate as f32;

        for sample in &mut self.samples {
            let disturbance = amplitude * phase.sin();
            *sample += disturbance;
            phase += phase_increment;
            if phase >= 2.0 * PI {
                phase -= 2.0 * PI;
            }
        }
    }

    /// Normalizes the audio buffer to prevent clipping
    fn normalize(&mut self) {
        let max_amplitude = self.samples.iter().fold(0.0, |acc, &x| acc.max(x.abs()));
        if max_amplitude > 1.0 {
            for sample in &mut self.samples {
                *sample /= max_amplitude;
            }
        }
    }

    /// Processes the audio buffer in parallel for improved efficiency
    fn process_parallel(&mut self, num_threads: usize, frequency: f32, amplitude: f32) {
        let chunk_size = self.samples.len() / num_threads;
        let mut handles = vec![];

        let arc_samples = Arc::new(self.samples.clone());
        let mutex_samples = Arc::new(std::sync::Mutex::new(vec![0.0; self.samples.len()]));

        for i in 0..num_threads {
            let arc_samples = Arc::clone(&arc_samples);
            let mutex_samples = Arc::clone(&mutex_samples);
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                self.samples.len()
            } else {
                start + chunk_size
            };

            handles.push(thread::spawn(move || {
                let mut phase = 0.0;
                let phase_increment = 2.0 * PI * frequency / 44100.0;

                for j in start..end {
                    let disturbance = amplitude * phase.sin();
                    let mut guard = mutex_samples.lock().unwrap();
                    guard[j] = arc_samples[j] + disturbance;
                    phase += phase_increment;
                    if phase >= 2.0 * PI {
                        phase -= 2.0 * PI;
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        self.samples = Arc::try_unwrap(mutex_samples).unwrap().into_inner().unwrap();
    }
}

fn main() {
    let mut audio_buffer = AudioBuffer::new(vec![0.0; 44100], 44100);
    audio_buffer.process_parallel(4, 440.0, 0.1);
    audio_buffer.normalize();

    println!("Audio processing complete!");
}