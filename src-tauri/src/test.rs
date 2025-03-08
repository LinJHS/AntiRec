use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::Mutex;
use rayon::prelude::*;

fn apply_disturbance(buffer: &mut [f32], frequency: f32, amplitude: f32) {
    buffer.par_iter_mut().enumerate().for_each(|(i, sample)| {
        let t = i as f32 / 44100.0;
        *sample += amplitude * (2.0 * PI * frequency * t).sin();
    });
}

fn process_audio(buffer: &mut [f32], gain: f32) {
    buffer.par_iter_mut().for_each(|sample| {
        *sample *= gain;
    });
}

fn main() {
    let mut audio_buffer = vec![0.0; 44100];
    let shared_buffer = Arc::new(Mutex::new(audio_buffer));

    let buffer_clone = Arc::clone(&shared_buffer);
    let handle = std::thread::spawn(move || {
        let mut buffer = buffer_clone.lock().unwrap();
        apply_disturbance(&mut buffer, 440.0, 0.1);
    });

    handle.join().unwrap();

    let mut buffer = shared_buffer.lock().unwrap();
    process_audio(&mut buffer, 1.5);

    println!("Audio processing completed.");
}