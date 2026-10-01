use super::RgbaImage;
use crate::model::CommandError;

const BYTES_PER_RGBA_PIXEL: u64 = 4;

pub fn encode_png(image: &RgbaImage) -> Result<Vec<u8>, CommandError> {
    let expected_bytes = u64::from(image.width) * u64::from(image.height) * BYTES_PER_RGBA_PIXEL;
    if expected_bytes == 0 || expected_bytes != image.rgba.len() as u64 {
        return Err(CommandError::Validation {
            field: "image".to_owned(),
        });
    }
    let mut png = Vec::new();
    let mut encoder = png::Encoder::new(&mut png, image.width, image.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .and_then(|mut writer| {
            writer.write_image_data(&image.rgba)?;
            writer.finish()
        })
        .map_err(|_| CommandError::Internal)?;
    Ok(png)
}
