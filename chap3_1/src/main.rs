use hound;
use std::f32::consts::PI;

const SAMPLING_RATE: u32 = 48000;

struct Score {
    track_no: usize,
    start_time: usize,
    freq: f32,
    gain: f32,
    length: usize,
}

fn create_sine_wave(freq: f32, gain: f32, length:usize) -> Vec<f32> {
    let size = length * (SAMPLING_RATE as usize);
    let mut data = Vec::new();

    for t in 0 .. size { 
        data.push(gain * f32::sin(t as f32 * freq * 2.0 * PI / 48000.0));
    }

    for t in 0 .. SAMPLING_RATE as usize / 100 {
        data[t] *= t as f32 / (SAMPLING_RATE as f32* 0.01);
        data[size - t - 1] *= t as f32 / (SAMPLING_RATE as f32* 0.01);
    }

    data
}

fn main() {
    let duration = 10;
    let scores = vec![Score {
        track_no: 1,
        start_time: 2,
        freq: 659.26,
        gain: 0.5,
        length: 1,
    }];
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLING_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };


    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    for score in scores {
        let wave = create_sine_wave(score.freq, score.gain, score.length);
        for s in wave {
            writer.write_sample(s).unwrap();
            writer.write_sample(s).unwrap();
        }
    }
}
