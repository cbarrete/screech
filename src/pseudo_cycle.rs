use crate::types::AudioBuffer;

pub fn fractalize(buffer: &AudioBuffer, depth: u32) -> AudioBuffer {
    let mut new_data = vec![0.; buffer.data.len()];
    let chs = buffer.metadata.channels as usize;
    let spc = buffer.data.len() / chs;
    for ch in 0..chs {
        let mut cycle_end = 0;
        let mut cycle_beg = 0;
        while cycle_end < spc {
            // go over the next pseudo-cycle
            while cycle_end < spc && buffer.data[ch + chs * cycle_end] >= 0. {
                cycle_end += 1
            }
            while cycle_end < spc && buffer.data[ch + chs * cycle_end] <= 0. {
                cycle_end += 1
            }

            for current_depth in 1..=depth as usize {
                let fractal_len = (cycle_end - cycle_beg) / current_depth;
                for j in 0..fractal_len {
                    let value =
                        buffer.data[ch + chs * (cycle_beg + current_depth * j)] / depth as f32;
                    for cycle in 0..current_depth {
                        new_data[ch + chs * (cycle_beg + j + cycle * fractal_len)] += value
                    }
                }
            }
            cycle_beg = cycle_end;
        }
    }
    AudioBuffer {
        metadata: buffer.metadata.clone(),
        data: new_data,
    }
}

pub fn interpolate(audio: &AudioBuffer) -> AudioBuffer {
    let mut new_data = vec![0.; audio.data.len()];
    let chs = audio.metadata.channels as usize;
    let spc = audio.data.len() / chs;

    for ch in 0..chs {
        let mut i = 0;
        // skip the first pseudo cycle
        while i < spc && audio.data[ch + chs * i] >= 0. {
            new_data[ch + chs * i] = audio.data[ch + chs * i];
            i += 1;
        }
        while i < spc && audio.data[ch + chs * i] <= 0. {
            new_data[ch + chs * i] = audio.data[ch + chs * i];
            i += 1;
        }

        let mut first_cycle_beg = 0;
        let mut first_cycle_end = i - 1;
        let mut second_cycle_beg = i;

        let mut write = i;
        while i < spc {
            // go over the next pseudo-cycle
            while i < spc && audio.data[ch + chs * i] >= 0. {
                i += 1
            }
            while i < spc && audio.data[ch + chs * i] <= 0. {
                i += 1
            }

            let second_cycle_end = i - 1;

            let first_cycle_len = 1 + first_cycle_end - first_cycle_beg;
            let second_cycle_len = 1 + second_cycle_end - second_cycle_beg;
            let ratio = first_cycle_len as f32 / second_cycle_len as f32;
            for j in 0..second_cycle_len {
                let f = audio.data[ch + chs * (first_cycle_beg + (ratio * j as f32) as usize)];
                let s = audio.data[ch + chs * (second_cycle_beg + j)];
                new_data[ch + chs * write] = (f + s) / 2.;
                write += 1;
            }

            first_cycle_beg = second_cycle_beg;
            first_cycle_end = second_cycle_end;
            second_cycle_beg = first_cycle_end + 1;
        }
    }

    AudioBuffer {
        metadata: audio.metadata.clone(),
        data: new_data,
    }
}

pub fn expand(mut audio: AudioBuffer) -> AudioBuffer {
    let chs = audio.metadata.channels as usize;
    let spc = audio.data.len() / chs;
    for ch in 0..chs {
        let mut cycle_end = 0;
        let mut cycle_beg = 0;
        while cycle_end < spc {
            // go over the next pseudo-cycle
            while cycle_end < spc && audio.data[ch + chs * cycle_end] >= 0. {
                cycle_end += 1
            }
            while cycle_end < spc && audio.data[ch + chs * cycle_end] <= 0. {
                cycle_end += 1
            }

            let mut max = 0.;
            for i in cycle_beg..cycle_end {
                let current = audio.data[ch + i * chs].abs();
                if current > max {
                    max = current;
                }
            }

            for i in cycle_beg..cycle_end {
                audio.data[ch + i * chs] /= max;
            }

            cycle_beg = cycle_end;
        }
    }
    audio
}

pub fn reverse_pseudo_cycles(mut audio: AudioBuffer) -> AudioBuffer {
    let chs = audio.metadata.channels as usize;
    let spc = audio.data.len() / chs;

    let mut buffer = Vec::new();

    for ch in 0..chs {
        let mut cycle_end = 0;
        let mut cycle_beg = 0;

        while cycle_end < spc {
            // go over the next pseudo-cycle
            while cycle_end < spc && audio.data[ch + chs * cycle_end] >= 0. {
                cycle_end += 1
            }
            while cycle_end < spc && audio.data[ch + chs * cycle_end] <= 0. {
                cycle_end += 1
            }

            let buffer_len = cycle_end - cycle_beg;
            buffer.resize(buffer_len, 0.);

            for i in 0..buffer_len {
                buffer[i] = audio.data[ch + (cycle_beg + i) * chs];
            }

            for i in 0..buffer_len {
                audio.data[ch + (cycle_beg + i) * chs] = buffer[buffer_len - i - 1];
            }

            cycle_beg = cycle_end;
        }
    }
    audio
}

pub fn tense_pseudo_cycles(mut audio: AudioBuffer, tension: f32) -> AudioBuffer {
    let chs = audio.metadata.channels as usize;
    let spc = audio.data.len() / chs;

    for ch in 0..chs {
        let mut cycle_end = 0;
        let mut cycle_beg = 0;

        while cycle_end < spc {
            let mut max = 0.;
            // go over the next pseudo-cycle
            while cycle_end < spc {
                let sample = audio.data[ch + chs * cycle_end];
                if sample < 0. {
                    break;
                }
                let current_amp = sample.abs();
                if current_amp > max {
                    max = current_amp;
                }
                cycle_end += 1
            }
            while cycle_end < spc && audio.data[ch + chs * cycle_end] <= 0. {
                let sample = audio.data[ch + chs * cycle_end];
                if sample > 0. {
                    break;
                }
                let current_amp = sample.abs();
                if current_amp > max {
                    max = current_amp;
                }
                cycle_end += 1
            }

            for i in cycle_beg..cycle_end {
                let sample = audio.data[ch + chs * i];
                audio.data[ch + chs * i] =
                    sample.signum() * max * (1. - (1. - sample.abs() / max).powf(tension));
            }

            cycle_beg = cycle_end;
        }
    }
    audio
}
