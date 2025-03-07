pub fn analyze_audio(audio_data: &[f32], sample_rate: u32) -> Vec<f32> {
    let window_size = 1024;
    let step_size = window_size / 2;
    let mut spectrogram = Vec::new();

    for i in (0..audio_data.len() - window_size).step_by(step_size) {
        let window = &audio_data[i..i + window_size];
        let mut fft = vec![0.0; window_size];

        for j in 0..window_size {
            fft[j] = window[j] * ((j as f32 / window_size as f32).sin());
        }

        let mut magnitudes = Vec::new();
        for j in 0..window_size / 2 {
            let re = fft[j];
            let im = fft[j + window_size / 2];
            let magnitude = (re * re + im * im).sqrt();
            magnitudes.push(magnitude);
        }

        spectrogram.extend(magnitudes);
    }

    spectrogram
}