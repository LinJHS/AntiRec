mod audio;

use audio::{AudioBuffer, DisturbanceType, process_audio};

fn main() {
    let mut buffer = AudioBuffer::new(44100, 2); // 44100 Hz, stereo
    buffer.load_from_file("input.wav").expect("Failed to load audio file");

    // Add white noise disturbance
    process_audio(&mut buffer, DisturbanceType::WhiteNoise(0.05)); // 5% white noise

    // Apply a low-pass filter to reduce high-frequency noise
    process_audio(&mut buffer, DisturbanceType::LowPassFilter(5000.0)); // 5 kHz cutoff

    // Normalize the audio to maximize volume without clipping
    process_audio(&mut buffer, DisturbanceType::Normalize);

    buffer.save_to_file("output.wav").expect("Failed to save audio file");
}