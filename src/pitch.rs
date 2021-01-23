use crate::types::*;

pub trait Pitch {
    fn delay_pitch(self, factor: f32, log_size: u8) -> Self;
    fn speed(self, speed: f32) -> Self;
}

impl Pitch for AudioBuffer {
    fn delay_pitch(mut self, factor: f32, log_size: u8) -> Self {
        if factor == 1.0 {
            return self
        }

        let channels = self.metadata.channels as usize;
        let samples_per_channel = self.data.len() / channels;

        let data_len = self.data.len();
        let data_len_bounded_size = if data_len.is_power_of_two() {
            data_len
        } else {
            1 + (usize::MAX >> data_len.leading_zeros() + 1)
        };
        let buffer_size = std::cmp::min(2usize.pow(log_size as u32), data_len_bounded_size) / channels;
        let buffer_mask = buffer_size - 1;
        let mut buffer = vec![0.0; buffer_size];

        for channel in 0..channels {
            if factor > 1.0 {
                // initialize the buffer to avoid initial silence
                for i in 0..buffer.len() {
                    buffer[i] = self.data[channel + i * channels];
                }
            }

            let mut read: f32 = 0.0;
            let mut write = 0;
            for i in 0..samples_per_channel {
                buffer[write] = self.data[channel + i * channels];

                write += 1;
                write &= buffer_mask;

                let first = read.floor();
                let second = read.ceil();
                let ratio = read - first;
                self.data[channel + i * channels] = buffer[first as usize] * ratio + buffer[second as usize & buffer_mask] * (1.0 - ratio);

                read += factor;

                if read >= buffer_size as f32 {
                    read -= buffer_size as f32;
                }
            }
        }

        self
    }

    fn speed(mut self, speed: f32) -> Self {
        if speed == 1.0 {
            return self
        }

        let channels = self.metadata.channels as usize;
        let new_len = (self.data.len() as f32 / speed) as usize;
        let samples_per_channel = new_len / channels;

        if speed > 1.0 {
            for channel in 0..channels {
                let mut read: f32 = 0.0;
                for i in 0..samples_per_channel {
                    let first = read.floor();
                    let second = read.ceil();
                    let ratio = read - first;
                    self.data[channel + i * channels] = self.data[channel + first  as usize * channels] * ratio
                                                      + self.data[channel + second as usize * channels] * (1.0 - ratio);
                    read += speed;
                }
            }
            self.data.resize(new_len, 0.0);
        } else {
            let mut buffer = vec![1.0; new_len];
            for channel in 0..channels {
                let mut read: f32 = 0.0;
                for i in 0..samples_per_channel {
                    let first = read.floor();
                    let second = read.ceil();
                    let ratio = read - first;
                    buffer[channel + i * channels] = self.data[channel + first  as usize * channels] * ratio
                                                   + self.data[channel + second as usize * channels] * (1.0 - ratio);
                    read += speed;
                }
            }
            self.data = buffer;
        }

        self
    }
}
