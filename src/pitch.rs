use crate::types::AudioBuffer;

pub fn delay_pitch(mut audio: AudioBuffer, factor: f32, log_size: u8) -> AudioBuffer {
    if factor == 1. {
        return audio;
    }

    let channels = audio.metadata.channels as usize;
    let samples_per_channel = audio.data.len() / channels;

    let data_len = audio.data.len();
    let data_len_bounded_size = if data_len.is_power_of_two() {
        data_len
    } else {
        1 + (usize::MAX >> data_len.leading_zeros() + 1)
    };
    let buffer_size = std::cmp::min(2usize.pow(log_size as u32), data_len_bounded_size) / channels;
    let buffer_mask = buffer_size - 1;
    let mut buffer = vec![0.; buffer_size];

    for channel in 0..channels {
        if factor > 1. {
            // initialize the buffer to avoid initial silence
            for i in 0..buffer.len() {
                buffer[i] = audio.data[channel + i * channels];
            }
        }

        let mut read: f32 = 0.;
        let mut write = 0;
        for i in 0..samples_per_channel {
            buffer[write & buffer_mask] = audio.data[channel + i * channels];
            write += 1;

            let first = read.floor();
            let second = read.ceil();
            let ratio = read - first;
            audio.data[channel + i * channels] = buffer[first as usize] * ratio
                + buffer[second as usize & buffer_mask] * (1. - ratio);

            read += factor;

            if read >= buffer_size as f32 {
                read -= buffer_size as f32;
            }
        }
    }

    audio
}

pub fn speed(mut audio: AudioBuffer, speed: f32) -> AudioBuffer {
    if speed == 1. {
        return audio;
    }

    let chs = audio.metadata.channels as usize;
    let new_len = (audio.data.len() as f32 / speed) as usize;
    let samples_per_channel = new_len / chs;

    if speed > 1. {
        for ch in 0..chs {
            let mut read: f32 = 0.;
            for i in 0..samples_per_channel {
                let first = read.floor();
                let second = read.ceil();
                let ratio = read - first;
                audio.data[ch + i * chs] = audio.data[ch + first as usize * chs] * ratio
                    + audio.data[ch + second as usize * chs] * (1. - ratio);
                read += speed;
            }
        }
        audio.data.truncate(new_len);
        audio.data.shrink_to_fit();
    } else {
        let mut buffer = vec![1.; new_len];
        for ch in 0..chs {
            let mut read: f32 = 0.;
            for i in 0..samples_per_channel {
                let first = read.floor();
                let second = read.ceil();
                let ratio = read - first;
                buffer[ch + i * chs] = audio.data[ch + first as usize * chs] * ratio
                    + audio.data[ch + second as usize * chs] * (1. - ratio);
                read += speed;
            }
        }
        audio.data = buffer;
    }

    audio
}
