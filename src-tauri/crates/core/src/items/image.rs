use png::{BitDepth, Decoder, Encoder, Transformations};
use std::io::Cursor;

pub struct NormalisedPng {
    pub png: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn normalise_png(source: &[u8]) -> Option<NormalisedPng> {
    let mut decoder = Decoder::new(Cursor::new(source));
    decoder.set_transformations(Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().ok()?;
    let mut pixels = vec![0; reader.output_buffer_size()?];
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
