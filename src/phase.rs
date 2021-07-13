use std::f32::consts::PI;

use crate::types::*;

pub trait Phase {
    fn delay_rotate(self, delay: usize, feedback: f32, frequency: f32) -> Self;
}

impl Phase for AudioBuffer {
    // TODO log size and ring buffer instead of delay
    fn delay_rotate(mut self, delay: usize, feedback: f32, frequency: f32) -> Self {
        let mut delay_buffer = Vec::with_capacity(delay);
        delay_buffer.resize(delay, Complex::zero());
        let lfo_step = 2. * PI * frequency / self.metadata.sample_rate as f32;

        let channels = self.metadata.channels as usize;
        let samples_per_channel = self.data.len() / channels;
        for channel in 0..channels {
            let mut t = 0;
            let mut i = 0;
            for sample in 0..samples_per_channel {
                // TODO only compute cos/sin every once in a while to save compute, maybe have a
                //      quality parameter
                let lfo = 1. + (t as f32 * lfo_step + PI * channel as f32).cos();
                t += 1;
                let out_complex = feedback * Complex::new(lfo.cos(), lfo.sin()) * delay_buffer[i]
                    + (1. - feedback) * self.data[channel + channels * sample];
                delay_buffer[i] = out_complex;
                i += 1;
                if i >= delay {
                    i = 0;
                }

                self.data[channel + channels * sample] = out_complex.r;
            }
        }
        self
    }
}
