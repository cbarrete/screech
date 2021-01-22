use crate::types::*;

pub trait Pitch {
    fn delay_pitch(self, factor: f32) -> AudioBuffer;
    fn speed(self, speed: f32) -> AudioBuffer;
}

impl Pitch for AudioBuffer {
    fn delay_pitch(mut self, factor: f32) -> AudioBuffer {
        // TODO might want to parametrize the size and make it a power of 2 to use masks
        if factor == 1.0 {
            return self
        }

        let buffer_size = std::cmp::min((self.metadata.sample_rate / 4) as usize, self.data.len());
        let mut buffer = vec![0.0; buffer_size];
        let channels = self.metadata.channels as usize;
        let samples_per_channel = self.data.len() / channels;

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
                if write == buffer_size {
                    write = 0;
                }

                let first = read.floor();
                let second = read.ceil();
                let ratio = read - first;
                self.data[channel + i * channels] = buffer[first as usize] * ratio + buffer[second as usize % buffer_size] * (1.0 - ratio);

                read += factor;

                if read >= buffer_size as f32 {
                    read -= buffer_size as f32;
                }
            }
        }

        self
    }

    fn speed(mut self, speed: f32) -> AudioBuffer {
        if speed == 1.0 {
            return self
        }

        if speed > 1.0 {
            let channels = self.metadata.channels as usize;
            let new_len = (self.data.len() as f32 / speed) as usize;
            let samples_per_channel = self.data.len() / channels;

            for channel in 0..channels {
                let mut read: f32 = 0.0;
                for i in 0..(samples_per_channel as f32 / speed) as usize {
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
            todo!("slowing down is not supported yet")
        }

        self
    }
}
