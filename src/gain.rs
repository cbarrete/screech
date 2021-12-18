use crate::types::AudioBuffer;

pub fn gain(mut audio: AudioBuffer, gain: f32) -> AudioBuffer {
    for s in &mut audio.data {
        *s *= gain;
    }
    audio
}

pub fn add_dc(mut audio: AudioBuffer, dc: f32) -> AudioBuffer {
    for s in &mut audio.data {
        *s += dc;
    }
    audio
}

pub fn remove_dc(mut audio: AudioBuffer) -> AudioBuffer {
    let dc = audio.data.iter().sum::<f32>() / audio.data.len() as f32;
    for s in &mut audio.data {
        *s -= dc;
    }
    audio
}

pub fn normalize(mut audio: AudioBuffer) -> AudioBuffer {
    let max_amplitude = audio
        .data
        .iter()
        .map(|s| s.abs())
        .max_by(|x, y| x.partial_cmp(y).expect("Invalid NaN sample"));

    if let Some(max) = max_amplitude {
        for s in &mut audio.data {
            *s /= max;
        }
    }

    audio
}
