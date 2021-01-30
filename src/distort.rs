use crate::types::*;

pub trait Distort {
    fn decimate(self, depth: f32) -> Self;
    fn fold(self) -> Self;
    fn hard_clip(self, gain: f32) -> Self;
    fn soft_clip(self, amount: f32) -> Self;
    fn waveshape_tension(self, tension: f32) -> Self;
}

impl Distort for AudioBuffer {
    fn decimate(mut self, depth: f32) -> Self {
        for s in &mut self.data {
            *s = (*s * depth).round() / depth;
        }
        self
    }

    fn fold(mut self) -> Self {
        for s in &mut self.data {
            *s = s.sin();
        }
        self
    }

    fn hard_clip(mut self, thresh: f32) -> Self {
        for s in &mut self.data {
            *s = if s.abs() > thresh {
                thresh * s.signum()
            } else {
                *s
            }
        }
        self
    }

    fn soft_clip(mut self, amount: f32) -> Self {
        for s in &mut self.data {
            *s -= amount * s.powi(3) / 3.;
        }
        self
    }

    fn waveshape_tension(mut self, tension: f32) -> Self {
        for s in &mut self.data {
            *s = 1. - (1. - *s).powf(tension);
        }
        self
    }
}
