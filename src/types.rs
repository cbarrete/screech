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

    pub fn get_channels(&self) -> Vec<Vec<f32>> {
        let n_channels = self.metadata.channels as usize;
        let mut channels = Vec::with_capacity(n_channels);
        for _ in 0..n_channels {
            channels.push(Vec::with_capacity(self.data.len() / n_channels));
        }

        for (i, sample) in self.data.iter().enumerate() {
            channels[(i % n_channels)].push(*sample);
        }
        channels
    }
}

pub fn from_channels(channels: &[Vec<f32>], sample_rate: u32) -> AudioBuffer {
    let chs = channels.len();
    let size: usize = channels.iter().map(Vec::len).sum();
    let spc = channels.iter().map(Vec::len).min().unwrap_or(0);

    let mut data = Vec::with_capacity(size);
    for s in 0..spc {
        for c in 0..chs {
            data.push(channels[c][s]);
        }
    }

    AudioBuffer { data, metadata: AudioMetadata { channels: chs as u16, sample_rate } }
}
