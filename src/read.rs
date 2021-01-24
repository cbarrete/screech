use std::io;
use std::io::Read;

use crate::types::*;

pub fn read_wav<R: Read>(reader: &mut R) -> Result<AudioBuffer, io::Error>
where
    R: Read,
{
    WavReader::new(reader).read()
}

struct WavReader<R: Read> {
    reader: R,
    format: WavFormat,
    bit_depth: u16,
}

trait ReadByte: Read {
    fn skip_bytes(&mut self, n: u64);
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_u32(&mut self) -> io::Result<u32>;
}

impl<R: Read> ReadByte for R {
    fn skip_bytes(&mut self, n: u64) {
        let _ = io::copy(&mut self.take(n), &mut io::sink());
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0_u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0_u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0_u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl<R: Read> WavReader<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            bit_depth: 32,
            format: WavFormat::FLOAT,
        }
    }

    fn read(mut self) -> Result<AudioBuffer, io::Error> {
        let mut buf = [0_u8; 4];
        self.reader.read_exact(&mut buf)?;
        if &buf != b"RIFF" {
            panic!("no RIFF tag found");
        }

        // filesize - 8, not used right now but could be used to verify the file length
        self.reader.skip_bytes(4);

        self.reader.read_exact(&mut buf)?;
        if &buf != b"WAVE" {
            panic!("no WAVE tag found");
        }

        read_chunks(&mut self.reader)
    }
}

fn read_data<R: Read>(
    mut wr: WavReader<R>,
    audio_buffer: &mut AudioBuffer,
) -> Result<&AudioBuffer, io::Error> {
    let length = wr.reader.read_u32()? as usize;

    let mut v = Vec::with_capacity(length);
    wr.reader.take(length as u64).read_to_end(&mut v)?;
    match wr.format {
        WavFormat::FLOAT => {
            if wr.bit_depth != 32 {
                panic!("wrong bit depth: only 32 is supported");
            }
            audio_buffer.data = v
                .chunks_exact(4)
                .map(|chunks| {
                    f32::from_le_bytes([chunks[0], chunks[1], chunks[2], chunks[3]]) as f32
                })
                .collect();
            Ok(audio_buffer)
        }
        WavFormat::PCM => {
            match wr.bit_depth {
                16 => {
                    audio_buffer.data = v
                        .chunks_exact(2)
                        .map(|chunks| {
                            f32::from(i16::from_le_bytes([chunks[0], chunks[1]]))
                                / f32::from(i16::max_value())
                        })
                        .collect();
                }
                24 => {
                    audio_buffer.data = v
                        .chunks_exact(3)
                        .map(|chunks| {
                            let arr = [0, chunks[0], chunks[1], chunks[2]];
                            (i32::from_le_bytes(arr)) as f32 / i32::max_value() as f32
                        })
                        .collect();
                }
                _ => panic!("{} bit not supported", wr.bit_depth),
            }
            Ok(audio_buffer)
        }
    }
}

fn read_chunks<R: Read>(reader: R) -> Result<AudioBuffer, io::Error> {
    fn _read_chunks<R: Read>(
        mut wr: WavReader<R>,
        audio_buffer: &mut AudioBuffer,
    ) -> Result<&AudioBuffer, io::Error> {
        let mut buf = [0_u8; 4];
        wr.reader.read_exact(&mut buf)?;
        match &buf {
            b"fact" => {
                let chunk_size = wr.reader.read_u32()?;
                wr.reader.skip_bytes(u64::from(chunk_size));
                _read_chunks(wr, audio_buffer)
            }
            b"fmt " => {
                let chunk_size = wr.reader.read_u32()?;
                wr.format = match wr.reader.read_u16()? {
                    0x0001 => WavFormat::PCM,
                    0x0003 => WavFormat::FLOAT,
                    x => panic!("unsupported format {}", x),
                };
                audio_buffer.metadata.channels = wr.reader.read_u16()?;
                audio_buffer.metadata.sample_rate = wr.reader.read_u32()?;
                wr.reader.skip_bytes(6);
                wr.bit_depth = wr.reader.read_u16()?;
                wr.reader.skip_bytes(u64::from(chunk_size) - 16);
                _read_chunks(wr, audio_buffer)
            }
            b"data" => read_data(wr, audio_buffer),
            tag => {
                eprintln!(
                    "unknown chunk {}",
                    std::str::from_utf8(tag).unwrap_or("which can't be printed")
                );
                let chunk_size = wr.reader.read_u32()?;
                wr.reader.skip_bytes(u64::from(chunk_size));
                _read_chunks(wr, audio_buffer)
            }
        }
    }

    let mut audio_buffer = AudioBuffer {
        metadata: AudioMetadata {
            channels: 2,
            sample_rate: 44100,
        },
        data: Vec::new(),
    };
    _read_chunks(WavReader::new(reader), &mut audio_buffer)?;
    Ok(audio_buffer)
}
