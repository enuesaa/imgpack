use anyhow::Result;
use image::codecs::jpeg::JpegEncoder;
use image::{ExtendedColorType, ImageReader};
use std::fs::File;

use crate::fs::Compressable;

pub fn pack_jpg(file: &Compressable) -> Result<()> {
    let inpath = file.inpath()?;
    let outpath = file.outpath()?;

    let img = ImageReader::open(inpath)?.decode()?;
    let mut out_file = File::create(outpath)?;

    // RGB8 へ変換
    let rgb = img.to_rgb8();

    // 圧縮
    let quality = 60;
    let mut encoder = JpegEncoder::new_with_quality(&mut out_file, quality);
    encoder.encode(&rgb, rgb.width(), rgb.height(), ExtendedColorType::Rgb8)?;

    Ok(())
}
