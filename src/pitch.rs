use crate::types::*;

pub trait Pitch {
    fn delay_pitch(self, factor: f32) -> AudioBuffer;
    fn speed(self, speed: f32) -> AudioBuffer;
}

impl Pitch for AudioBuffer {
    // TODO use the factor to do something other than pitching down
    fn delay_pitch(mut self, factor: f32) -> AudioBuffer {
        // TODO might want to parametrize the size
        let buffer_size = std::cmp::min((self.metadata.sample_rate / 4) as usize, self.data.len());
        let mut buffer = vec![0.0; buffer_size];

        let channels = self.metadata.channels as usize;
        let samples_per_channel = self.data.len() / channels;
        for channel in 0..channels {
            if factor < 1.0 {
                // initialize the buffer to avoid initial silence
                for i in 0..buffer.len() {
                    buffer[i] = self.data[channel + i * channels];
                }
            }

            let mut divider = 0;
            let mut read_index = 0;
            let mut write_index = 0;
            for i in 0..samples_per_channel {
                buffer[write_index] = self.data[channel + i * channels];

                write_index += 1;
                if write_index == buffer_size {
                    write_index = 0;
                }

                self.data[channel + i * channels] = buffer[read_index];

                divider += 1;
                // TODO optimize with mask?
                if divider == 2 {
                    read_index += 1;
                    divider = 0;
                }

                if read_index == buffer_size {
                    read_index = 0;
                }
            }
        }

        self
    }

    fn speed(self, speed: f32) -> AudioBuffer {
        // only allocate if slower
        todo!()
    }
}
