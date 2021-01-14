use std::io;
use std::io::Write;

use crate::types::*;

pub fn write_wav<W: Write>(writer: &mut W, audio_buffer: &AudioBuffer) -> Result<(), io::Error> {
    writer.write_all(b"RIFF")?;
    writer.write_all(&audio_buffer.file_size().to_le_bytes())?;
    writer.write_all(b"WAVE")?;

    writer.write_all(b"fmt ")?;
    writer.write_all(&AudioBuffer::FMT_CHUNK_SIZE.to_le_bytes())?;
    let md = &audio_buffer.metadata;
    writer.write_all(&(WavFormat::FLOAT as u16).to_le_bytes())?;
    writer.write_all(&md.channels.to_le_bytes())?;
    writer.write_all(&md.sample_rate.to_le_bytes())?;
    writer.write_all(&(md.sample_rate * 32 / 8).to_le_bytes())?;
    writer.write_all(&(4 * md.channels).to_le_bytes())?;
    writer.write_all(&(32_u16).to_le_bytes())?;

    writer.write_all(b"data")?;
    writer.write_all(&(4 * audio_buffer.data.len()).to_le_bytes())?;
    let (_, samples, _) = unsafe { audio_buffer.data.align_to::<u8>() };
    writer.write_all(samples)?;

    Ok(())
}
