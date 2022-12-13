use hound;
use std::f32::consts::PI;

const SAMPLING_RATE: usize = 48000;

#[derive(Debug)]
struct Score {
    track_no: usize,
    start_time: usize,
    freq: f32,
    gain: f32,
    length: usize,
}

macro_rules! create_scores {
    ($([$n:expr, $t:expr, $f:expr, $g:expr, $l:expr]),+) => {
        {
            let mut scores = Vec::new();
            $(

                let score = Score{track_no: $n, start_time: $t, freq: $f, gain: $g, length: $l};
                scores.push(score);
            )*
            scores
        }
    };
}

fn create_sine_wave(freq: f32, gain: f32, length: usize) -> Vec<f32> {
    let size = length * SAMPLING_RATE;
    let mut data = Vec::new();

    for t in 0..size {
        data.push(gain * f32::sin(t as f32 * freq * 2.0 * PI / 48000.0));
    }

    for t in 0..SAMPLING_RATE as usize / 100 {
        data[t] *= t as f32 / (SAMPLING_RATE as f32 * 0.01);
        data[size - t - 1] *= t as f32 / (SAMPLING_RATE as f32 * 0.01);
    }

    data
}

fn main() {
    let duration = 10;
    let scores = create_scores![
        [1, 2, 659.26, 0.5, 1],
        [1, 3, 587.33, 0.5, 1],
        [1, 4, 523.25, 0.5, 1],
        [1, 5, 493.88, 0.5, 1],
        [1, 6, 440.00, 0.5, 1],
        [1, 7, 392.00, 0.5, 1],
        [1, 8, 440.00, 0.5, 1],
        [1, 9, 493.88, 0.5, 1],
        [2, 2, 261.63, 0.5, 1],
        [2, 3, 196.00, 0.5, 1],
        [2, 4, 220.00, 0.5, 1],
        [2, 5, 164.81, 0.5, 1],
        [2, 6, 174.61, 0.5, 1],
        [2, 7, 130.81, 0.5, 1],
        [2, 8, 174.61, 0.5, 1],
        [2, 9, 196.00, 0.5, 1]
    ];
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLING_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let mut data = vec![0f32; duration * SAMPLING_RATE * 2];

    for score in &scores {
        dbg!(score);
        let wave = create_sine_wave(score.freq, score.gain, score.length);
        let offset = score.start_time * SAMPLING_RATE * 2;
        let ch = score.track_no - 1;

        for (i, s) in wave.iter().enumerate() {
            data[offset + (i * 2) + ch] = *s;
        }
    }

    for d in data {
        writer.write_sample(d).unwrap();
    }
}
