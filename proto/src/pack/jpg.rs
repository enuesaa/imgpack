use anyhow::Result;
use image::ExtendedColorType;
use image::{io::Reader as ImageReader};
use image::codecs::jpeg::{JpegEncoder};
use std::fs::File;

pub fn compress_jpeg() -> Result<()> {
    let input_path = "input.jpg";
    let output_path = "output.jpg";
    let quality = 60; // 0〜100、低いほど圧縮率高

    let img = ImageReader::open(input_path)?.decode()?;
    let mut out_file = File::create(output_path)?;

    // RGB8 に変換
    let rgb = img.to_rgb8();

    // JPEG エンコーダで圧縮
    let mut encoder = JpegEncoder::new_with_quality(&mut out_file, quality);
    encoder.encode(&rgb, rgb.width(), rgb.height(), ExtendedColorType::Rgb8)?;
    Ok(())
}
