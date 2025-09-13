use anyhow::{Result, anyhow};
use color_quant::NeuQuant;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use oxipng::Options;
use std::io::Cursor;

use crate::fs::Compressable;

pub fn pack_png(file: &Compressable) -> Result<()> {
    let inpath = file.inpath()?;
    let outpath = file.outpath()?;

    let original = image::open(inpath)?;
    let pixels: Vec<u8> = original.to_rgba8().into_vec();

    let (indexed_pixels, palette) = extpng_quantize(&pixels);
    let buf = extpng_build(original, &indexed_pixels, &palette)?;
    let optimized = extpng_optimize(buf)?;

    std::fs::write(outpath, optimized)?;
    Ok(())
}

// 減色
fn extpng_quantize(pixels: &[u8]) -> (Vec<u8>, Vec<Rgba<u8>>) {
    let palette_size = 256; // 減色後の色数
    let nq = NeuQuant::new(10, palette_size, pixels);

    let indexed: Vec<u8> = pixels
        .chunks_exact(4)
        .map(|pixel| {
            // 透明度は無視
            if pixel[3] == 0 {
                return 0;
            };
            nq.index_of(pixel) as u8
        })
        .collect();

    // パレット
    let palette_bytes_arr = nq.color_map_rgba();
    let palette: Vec<Rgba<u8>> = palette_bytes_arr
        .chunks_exact(4)
        .map(|pixel| {
            let [r, g, b, a]: [u8; 4] = pixel.try_into().unwrap();
            Rgba([r, g, b, a])
        })
        .collect();

    (indexed, palette)
}

// indexed を palette の色で置き換え
fn extpng_build(original: DynamicImage, indexed: &[u8], palette: &[Rgba<u8>]) -> Result<Vec<u8>> {
    let pixels: Vec<u8> = indexed
        .iter()
        .flat_map(|&idx| palette[idx as usize].0)
        .collect();

    let (width, height) = original.dimensions();
    let img: RgbaImage = ImageBuffer::from_vec(width, height, pixels)
        .ok_or_else(|| anyhow!("failed to map pixels"))?;

    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)?;
    Ok(buf)
}

fn extpng_optimize(buf: Vec<u8>) -> Result<Vec<u8>> {
    let mut opts = Options::from_preset(3);
    opts.interlace = None;
    opts.strip = oxipng::StripChunks::All;
    let optimized = oxipng::optimize_from_memory(&buf, &opts)?;
    Ok(optimized)
}
