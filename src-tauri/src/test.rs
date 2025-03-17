use std::f32::consts::PI;
use std::sync::Arc;
use std::thread;

struct AudioBuffer {
    data: Vec<f32>,
    sample_rate: u32,
}

impl AudioBuffer {
    fn new(data: Vec<f32>, sample_rate: u32) -> Self {
        AudioBuffer { data, sample_rate }
    }

    fn add_disturbance(&mut self, frequency: f32, amplitude: f32) {
        let phase_step = 2.0 * PI * frequency / self.sample_rate as f32;
        for (i, sample) in self.data.iter_mut().enumerate() {
            let phase = phase_step * i as f32;
            *sample += amplitude * phase.sin();
        }
    }

    fn process_in_parallel(&mut self, chunk_size: usize) {
        let mut handles = vec![];
        let data_arc = Arc::new(self.data.clone());

        for chunk in data_arc.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let handle = thread::spawn(move || {
                chunk.into_iter().map(|sample| sample * 0.5).collect::<Vec<f32>>()
            });
            handles.push(handle);
        }

        let mut processed_data = vec![];
        for handle in handles {
            processed_data.extend(handle.join().unwrap());
        }

        self.data = processed_data;
    }
}

fn main() {
    let mut audio_buffer = AudioBuffer::new(vec![0.0; 44100], 44100);
    audio_buffer.add_disturbance(440.0, 0.1);
    audio_buffer.process_in_parallel(4410);

    println!("Audio processing complete!");
}