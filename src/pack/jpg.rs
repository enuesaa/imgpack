use anyhow::Result;
use image::codecs::jpeg::JpegEncoder;
use image::ExtendedColorType;
use image::ImageReader;
use std::fs::File;
use std::path::PathBuf;

pub fn compress_jpg(filepath: &PathBuf, output_filepath: &PathBuf) -> Result<()> {
    // 0〜100、低いほど圧縮率高
    let quality = 60;

    let img = ImageReader::open(filepath)?.decode()?;
    let mut out_file = File::create(output_filepath)?;

    // RGB8 に変換
    let rgb = img.to_rgb8();

    // JPEG エンコーダで圧縮
    let mut encoder = JpegEncoder::new_with_quality(&mut out_file, quality);
    encoder.encode(&rgb, rgb.width(), rgb.height(), ExtendedColorType::Rgb8)?;
    Ok(())
}
