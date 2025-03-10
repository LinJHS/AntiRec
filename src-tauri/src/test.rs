use std::f32::consts::PI;

fn apply_disturbance(signal: &[f32], noise_level: f32) -> Vec<f32> {
    signal.iter()
        .map(|&x| x + noise_level * (2.0 * rand::random::<f32>() - 1.0))
        .collect()
}

fn fast_fourier_transform(signal: &[f32]) -> Vec<f32> {
    let n = signal.len();
    let mut output = signal.to_vec();

    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..n {
            let theta = -2.0 * PI * (i as f32) * (j as f32) / (n as f32);
            sum += signal[j] * (theta.cos() + theta.sin());
        }
        output[i] = sum;
    }
    output
}

fn compress_audio(signal: &[f32], threshold: f32) -> Vec<f32> {
    signal.iter()
        .map(|&x| if x.abs() < threshold { 0.0 } else { x })
        .collect()
}

fn main() {
    let signal = vec![0.5, -0.2, 0.7, -0.1, 0.3];
    let noisy_signal = apply_disturbance(&signal, 0.1);
    let compressed_signal = compress_audio(&noisy_signal, 0.2);
    let freq_domain = fast_fourier_transform(&compressed_signal);

    println!("{:?}", freq_domain);
}