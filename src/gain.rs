use crate::types::*;

pub trait Gain {
    fn gain(self, gain: f32) -> Self;
    fn add_dc(self, dc: f32) -> Self;
    fn remove_dc(self) -> Self;
    fn normalize(self) -> Self;
}

impl Gain for AudioBuffer {
    fn gain(mut self, gain: f32) -> Self {
        for s in &mut self.data {
            *s *= gain;
        }
        self
    }

    fn add_dc(mut self, dc: f32) -> Self {
        for s in &mut self.data {
            *s += dc;
        }
        self
    }

    fn remove_dc(mut self) -> Self {
        let dc = self.data.iter().sum::<f32>() / self.data.len() as f32;
        for s in &mut self.data {
            *s -= dc;
        }
        self
    }

    fn normalize(mut self) -> Self {
        let max_amplitude = self
            .data
            .iter()
            .map(|s| s.abs())
            .max_by(|x, y| x.partial_cmp(y).expect("Invalid NaN sample"));

        if let Some(max) = max_amplitude {
            for s in &mut self.data {
                *s /= max;
            }
        }

        self
    }
}
