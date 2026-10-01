use png::{BitDepth, Decoder, Encoder, Transformations};
use std::io::Cursor;

const MAX_DECODED_BYTES: usize = 256 * 1024 * 1024;
const RGB_BYTES_PER_PALETTE_ENTRY: usize = 3;

pub struct NormalisedPng {
    pub png: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn normalise_png(source: &[u8]) -> Option<NormalisedPng> {
    let mut decoder = Decoder::new(Cursor::new(source));
    decoder.set_transformations(Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().ok()?;
    let decoded_bytes = reader
        .output_buffer_size()
        .filter(|&bytes| bytes <= MAX_DECODED_BYTES)?;
    if reader
        .info()
        .palette
        .as_ref()
        .is_some_and(|palette| palette.len() % RGB_BYTES_PER_PALETTE_ENTRY != 0)
    {
        return None;
    }
    let mut pixels = vec![0; decoded_bytes];
    let frame = reader.next_frame(&mut pixels).ok()?;

    let mut png = Vec::new();
    let mut encoder = Encoder::new(&mut png, frame.width, frame.height);
    encoder.set_color(frame.color_type);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().ok()?;
    writer
        .write_image_data(&pixels[..frame.buffer_size()])
        .ok()?;
    writer.finish().ok()?;

    Some(NormalisedPng {
        png,
        width: frame.width,
        height: frame.height,
    })
}
