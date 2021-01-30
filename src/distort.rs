use crate::types::*;

pub trait Distort {
    fn decimate(self, depth: f32) -> Self;
    fn fold(self) -> Self;
    fn hard_clip(self, gain: f32) -> Self;
    fn soft_clip(self, amount: f32) -> Self;
    fn fractalize(&self, depth: u32) -> Self;
    fn interpolate(&self) -> Self;
    fn expand(self) -> Self;
    fn reverse_pseudo_cycles(self) -> Self;
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

    fn fractalize(&self, depth: u32) -> Self {
        let mut new_data = vec![0.; self.data.len()];
        let chs = self.metadata.channels as usize;
        let spc = self.data.len() / chs;
        for ch in 0..chs {
            let mut i = 0;
            let mut cycle_beg = 0;
            while i < spc {
                // go over the next pseudo-cycle
                while i < spc && self.data[ch + chs * i] >= 0. {
                    i += 1
                }
                while i < spc && self.data[ch + chs * i] <= 0. {
                    i += 1
                }

                for current_depth in 1..=depth as usize {
                    let fractal_len = (i - cycle_beg) / current_depth;
                    for j in 0..fractal_len {
                        let value =
                            self.data[ch + chs * (cycle_beg + current_depth * j)] / depth as f32;
                        for cycle in 0..current_depth {
                            new_data[ch + chs * (cycle_beg + j + cycle * fractal_len)] += value
                        }
                    }
                }
                cycle_beg = i;
            }
        }
        Self {
            metadata: self.metadata.clone(),
            data: new_data,
        }
    }

    fn interpolate(&self) -> Self {
        let frac_chans: Vec<Vec<f32>> = self
            .get_channels()
            .iter()
            .map(|c| interpolate_channel(c))
            .collect();
        from_channels(&frac_chans, self.metadata.sample_rate)
    }

    fn expand(mut self) -> Self {
        let chs = self.metadata.channels as usize;
        let spc = self.data.len() / chs;
        for ch in 0..chs {
            let mut cycle_end = 0;
            let mut cycle_beg = 0;
            while cycle_end < spc {
                // go over the next pseudo-cycle
                while cycle_end < spc && self.data[ch + chs * cycle_end] >= 0. {
                    cycle_end += 1
                }
                while cycle_end < spc && self.data[ch + chs * cycle_end] <= 0. {
                    cycle_end += 1
                }

                let mut max = 0.;
                for i in cycle_beg..cycle_end {
                    let current = self.data[ch + i * chs].abs();
                    if current > max {
                        max = current;
                    }
                }

                for i in cycle_beg..cycle_end {
                    self.data[ch + i * chs] /= max;
                }

                cycle_beg = cycle_end;
            }
        }
        self
    }

    fn reverse_pseudo_cycles(mut self) -> Self {
        let chs = self.metadata.channels as usize;
        let spc = self.data.len() / chs;

        let mut buffer = Vec::new();

        for ch in 0..chs {
            let mut cycle_end = 0;
            let mut cycle_beg = 0;

            while cycle_end < spc {
                // go over the next pseudo-cycle
                while cycle_end < spc && self.data[ch + chs * cycle_end] >= 0. {
                    cycle_end += 1
                }
                while cycle_end < spc && self.data[ch + chs * cycle_end] <= 0. {
                    cycle_end += 1
                }

                let buffer_len = cycle_end - cycle_beg;
                buffer.resize(buffer_len, 0.);

                for i in 0..buffer_len {
                    buffer[i] = self.data[ch + (cycle_beg + i) * chs];
                }

                for i in 0..buffer_len {
                    self.data[ch + (cycle_beg + i) * chs] = buffer[buffer_len - i - 1];
                }

                cycle_beg = cycle_end;
            }
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

fn interpolate_channel(channel: &[f32]) -> Vec<f32> {
    let mut out = Vec::with_capacity(channel.len());
    let mut i = 0;

    // skip the first pseudo cycle
    while i < channel.len() && channel[i] >= 0. {
        out.push(channel[i]);
        i += 1
    }
    while i < channel.len() && channel[i] <= 0. {
        out.push(channel[i]);
        i += 1
    }

    let mut first_cycle_beg = 0;
    let mut first_cycle_end = i - 1;
    let mut second_cycle_beg = i;

    while i < channel.len() {
        // go over the next pseudo-cycle
        while i < channel.len() && channel[i] >= 0. {
            i += 1
        }
        while i < channel.len() && channel[i] <= 0. {
            i += 1
        }

        let second_cycle_end = i - 1;

        let first_cycle_len = 1 + first_cycle_end - first_cycle_beg;
        let second_cycle_len = 1 + second_cycle_end - second_cycle_beg;
        let ratio = first_cycle_len as f32 / second_cycle_len as f32;
        for j in 0..second_cycle_len {
            let f = channel[first_cycle_beg + (ratio * j as f32) as usize];
            let s = channel[second_cycle_beg + j];
            out.push((f + s) / 2.)
        }

        first_cycle_beg = second_cycle_beg;
        first_cycle_end = second_cycle_end;
        second_cycle_beg = first_cycle_end + 1;
    }

    out.shrink_to_fit();
    out
}
