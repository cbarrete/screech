use crate::types::AudioBuffer;

pub fn decimate(mut audio: AudioBuffer, depth: f32) -> AudioBuffer {
    for s in &mut audio.data {
        *s = (*s * depth).round() / depth;
    }
    audio
}

pub fn fold(mut audio: AudioBuffer) -> AudioBuffer {
    for s in &mut audio.data {
        *s = s.sin();
    }
    audio
}

pub fn hard_clip(mut audio: AudioBuffer, thresh: f32) -> AudioBuffer {
    for s in &mut audio.data {
        *s = if s.abs() > thresh {
            thresh * s.signum()
        } else {
            *s
        }
    }
    audio
}

pub fn soft_clip(mut audio: AudioBuffer, amount: f32) -> AudioBuffer {
    for s in &mut audio.data {
        *s -= amount * s.powi(3) / 3.;
    }
    audio
}

pub fn tense(mut audio: AudioBuffer, tension: f32) -> AudioBuffer {
    for s in &mut audio.data {
        *s = s.signum() * (1. - (1. - s.abs()).powf(tension));
    }
    audio
}
