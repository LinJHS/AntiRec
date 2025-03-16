use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub struct AudioProcessor {
    sample_rate: u32,
    buffer_size: usize,
    is_processing: Arc<AtomicBool>,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32, buffer_size: usize) -> Self {
        Self {
            sample_rate,
            buffer_size,
            is_processing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn process_audio(&self, input: &[f32], output: &mut [f32]) {
        assert_eq!(input.len(), output.len());

        let mut phase = 0.0;
        let phase_increment = 440.0 * 2.0 * PI / self.sample_rate as f32;

        for i in 0..input.len() {
            let sample = input[i];
            let disturbed_sample = sample + 0.1 * (phase.sin() * 0.5 + 0.5);
            output[i] = disturbed_sample;

            phase += phase_increment;
            if phase >= 2.0 * PI {
                phase -= 2.0 * PI;
            }
        }
    }

    pub fn start_processing(&self) {
        self.is_processing.store(true, Ordering::SeqCst);
        let is_processing = Arc::clone(&self.is_processing);
        let buffer_size = self.buffer_size;
        let sample_rate = self.sample_rate;

        thread::spawn(move || {
            let mut input_buffer = vec![0.0; buffer_size];
            let mut output_buffer = vec![0.0; buffer_size];

            while is_processing.load(Ordering::SeqCst) {
                // Simulate audio input (e.g., from a microphone or file)
                for i in 0..buffer_size {
                    input_buffer[i] = (i as f32 / buffer_size as f32).sin();
                }

                // Process audio
                let mut processor = AudioProcessor::new(sample_rate, buffer_size);
                processor.process_audio(&input_buffer, &mut output_buffer);

                // Simulate audio output (e.g., to a speaker or file)
                for sample in &output_buffer {
                    let _ = *sample; // Placeholder for actual output handling
                }

                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    pub fn stop_processing(&self) {
        self.is_processing.store(false, Ordering::SeqCst);
    }
}