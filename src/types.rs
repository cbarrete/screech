#[derive(Clone)]
pub struct AudioMetadata {
    pub channels: u16,
    pub sample_rate: u32,
}

pub enum WavFormat {
    PCM = 1,
    FLOAT = 3,
}

#[derive(Clone)]
pub struct AudioBuffer {
    pub metadata: AudioMetadata,
    pub data: Vec<f32>,
}

impl AudioBuffer {
    pub const FMT_CHUNK_SIZE: u32 = 16;

    pub fn file_size(&self) -> u32 {
        4 + 20 + 8 + 4 * self.data.len() as u32
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Complex {
    pub r: f32,
    pub i: f32,
}

impl Complex {
    pub fn zero() -> Self {
        Self { r: 0., i: 0. }
    }

    pub fn new(r: f32, i: f32) -> Self {
        Self { r, i }
    }
}

impl std::ops::Mul<Complex> for Complex {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r + self.i * rhs.i, // ac - bd
            i: self.r * rhs.i + self.r * rhs.i, // ad + bc
        }
    }
}

impl std::ops::Add<f32> for Complex {
    type Output = Complex;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r + rhs,
            i: self.i,
        }
    }
}

impl std::ops::Mul<Complex> for f32 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Self::Output {
            r: self * rhs.r,
            i: self * rhs.i,
        }
    }
}
