use anyhow::Result;
use color_quant::NeuQuant;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use oxipng::Options;
use std::io::Cursor;

use crate::fs::Compressable;

pub fn pack_png(file: &Compressable) -> Result<()> {
    let inpath = file.inpath()?;
    let outpath = file.outpath()?;

    let img = image::open(inpath)?;
    let (width, height) = img.dimensions();

    let rgba_pixels: Vec<u8> = img.to_rgba8().into_vec();

    let (indexed_pixels, palette) = extpng_quantize(width, height, &rgba_pixels);
    let buf = extpng_build_image(width, height, &indexed_pixels, &palette)?;
    let optimized = extpng_optimize(buf)?;

    std::fs::write(outpath, optimized)?;
    Ok(())
}

// 減色
fn extpng_quantize(width: u32, height: u32, rgba_pixels: &[u8]) -> (Vec<u8>, Vec<Rgba<u8>>) {
    let palette_size = 256; // 減色後の色数

    let nq = NeuQuant::new(10, palette_size, rgba_pixels);
    let mut indexed_pixels = Vec::with_capacity((width * height) as usize);

    for pixel in rgba_pixels.chunks_exact(4) {
        let idx = if pixel[3] == 0 { 0 } else { nq.index_of(pixel) } as u8;
        indexed_pixels.push(idx);
    }

    // パレット
    let palette_bytes_arr = nq.color_map_rgba();
    let palette: Vec<Rgba<u8>> = palette_bytes_arr
        .chunks_exact(4)
        .map(|color| {
            let [r, g, b, a]: [u8; 4] = color.try_into().unwrap();
            Rgba([r, g, b, a])
        })
        .collect();

    (indexed_pixels, palette)
}

fn extpng_build_image(width: u32, height: u32, indexed_pixels: &[u8], palette: &[Rgba<u8>]) -> Result<Vec<u8>> {
    let mut tmp_img: RgbaImage = ImageBuffer::new(width, height);
    for (i, idx) in indexed_pixels.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        tmp_img.put_pixel(x, y, palette[*idx as usize]);
    }

    let mut buf = Vec::new();
    tmp_img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)?;
    Ok(buf)
}

fn extpng_optimize(buf: Vec<u8>) -> Result<Vec<u8>> {
    let mut opts = Options::from_preset(3);
    opts.interlace = None;
    opts.strip = oxipng::StripChunks::All;
    let optimized = oxipng::optimize_from_memory(&buf, &opts)?;
    Ok(optimized)
}
