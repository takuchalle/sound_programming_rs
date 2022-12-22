use core::f32::consts::PI;
pub const SAMPLING_RATE: usize = 48000;

pub fn create_sine_wave(freq: f32, gain: f32, length: usize) -> Vec<f32> {
    let size = length * SAMPLING_RATE;

    let mut data: Vec<_> = (0..size)
        .map(|t| gain * f32::sin(t as f32 * freq * 2.0 * PI / 48000.0))
        .collect();

    for t in 0..SAMPLING_RATE as usize / 100 {
        data[t] *= t as f32 / (SAMPLING_RATE as f32 * 0.01);
        data[size - t - 1] *= t as f32 / (SAMPLING_RATE as f32 * 0.01);
    }

    data
}
