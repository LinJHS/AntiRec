use std::f32::consts::PI;

pub fn analyze_audio(data: &[f32]) -> (f32, f32, f32) {
    let sum: f32 = data.iter().sum();
    let mean = sum / data.len() as f32;

    let variance: f32 = data.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / data.len() as f32;
    let std_dev = variance.sqrt();

    let max_amplitude = data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

    (mean, std_dev, max_amplitude)
}

pub fn dominant_frequency(data: &[f32], sample_rate: f32) -> f32 {
    let n = data.len();
    let mut magnitudes: Vec<f32> = vec![0.0; n];

    for k in 0..n {
        let mut sum_real = 0.0;
        let mut sum_imag = 0.0;

        for t in 0..n {
            let angle = 2.0 * PI * (k as f32) * (t as f32) / (n as f32);
            sum_real += data[t] * angle.cos();
            sum_imag -= data[t] * angle.sin();
        }

        magnitudes[k] = (sum_real.powi(2) + sum_imag.powi(2)).sqrt();
    }

    let max_index = magnitudes
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap_or(0);

    (max_index as f32 * sample_rate) / (n as f32)
}