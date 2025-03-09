use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct AudioBuffer {
    samples: Arc<Vec<f32>>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer {
            samples: Arc::new(samples),
            sample_rate,
        }
    }

    fn add_disturbance(&self, frequency: f32, amplitude: f32) -> AudioBuffer {
        let mut disturbed_samples = Vec::with_capacity(self.samples.len());
        let phase_increment = 2.0 * PI * frequency / self.sample_rate as f32;

        for (i, &sample) in self.samples.iter().enumerate() {
            let phase = phase_increment * i as f32;
            let disturbance = amplitude * phase.sin();
            disturbed_samples.push(sample + disturbance);
        }

        AudioBuffer::new(disturbed_samples, self.sample_rate)
    }

    fn process_parallel<F>(&self, mut f: F) -> AudioBuffer
    where
        F: FnMut(f32) -> f32 + Send + Clone + 'static,
    {
        let mut handles = vec![];
        let chunk_size = self.samples.len() / 4;
        let samples = self.samples.clone();

        for chunk in samples.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let mut f = f.clone();
            handles.push(thread::spawn(move || {
                chunk.into_iter().map(|s| f(s)).collect::<Vec<f32>>()
            }));
        }

        let processed_samples: Vec<f32> = handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap())
            .collect();

        AudioBuffer::new(processed_samples, self.sample_rate)
    }
}

fn main() {
    let original_samples = vec![0.0; 44100];
    let audio_buffer = AudioBuffer::new(original_samples, 44100);

    let disturbed_audio = audio_buffer.add_disturbance(440.0, 0.1);

    let processed_audio = disturbed_audio.process_parallel(|sample| sample * 0.5);

    thread::sleep(Duration::from_secs(1));
}