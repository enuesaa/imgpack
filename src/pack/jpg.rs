use anyhow::Result;
use image::codecs::jpeg::JpegEncoder;
use image::ExtendedColorType;
use image::ImageReader;
use std::fs::File;

use crate::fs::compressable::Compressable;

pub fn compress_jpg(file: &Compressable) -> Result<()> {
    let inpath = file.path();
    let outpath = file.outpath()?;

    // 0〜100、低いほど圧縮率高
    let quality = 60;

    let img = ImageReader::open(inpath)?.decode()?;
    let mut out_file = File::create(outpath)?;

    // RGB8 に変換
    let rgb = img.to_rgb8();

    // JPEG エンコーダで圧縮
    let mut encoder = JpegEncoder::new_with_quality(&mut out_file, quality);
    encoder.encode(&rgb, rgb.width(), rgb.height(), ExtendedColorType::Rgb8)?;
    Ok(())
}
