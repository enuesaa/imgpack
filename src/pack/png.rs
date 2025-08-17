use anyhow::Result;
use color_quant::NeuQuant;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use oxipng::Options;
use std::io::Cursor;

use crate::fs::Compressable;

pub fn pack_png(file: &Compressable) -> Result<()> {
    let inpath = file.path();
    let outpath = file.outpath()?;

    let palette_size = 256; // 減色後の色数

    let img = image::open(inpath)?;
    let (width, height) = img.dimensions();

    // RGBAへ変換
    let rgba_pixels: Vec<u8> = img.to_rgba8().into_vec();

    // NeuQuant で減色
    let nq = NeuQuant::new(10, palette_size, &rgba_pixels);

    // 各ピクセルをインデックス
    let mut indexed_pixels = Vec::with_capacity((width * height) as usize);
    for i in 0..(width * height) {
        let offset = (i * 4) as usize;
        let pixel = &rgba_pixels[offset..offset + 4];
        let idx = if pixel[3] == 0 { 0 } else { nq.index_of(pixel) };
        indexed_pixels.push(idx);
    }

    // パレットを作成
    let palette_bytes = nq.color_map_rgb();
    let mut palette: Vec<Rgba<u8>> = Vec::with_capacity(palette_size);
    for i in 0..palette_size {
        let r = palette_bytes[i * 3];
        let g = palette_bytes[i * 3 + 1];
        let b = palette_bytes[i * 3 + 2];
        palette.push(Rgba([r, g, b, 255]));
    }

    // 減色して画像を組み立て
    let mut tmp_img: RgbaImage = ImageBuffer::new(width, height);
    for (i, idx) in indexed_pixels.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        tmp_img.put_pixel(x, y, palette[*idx as usize]);
    }
    let mut buf = Vec::new();
    tmp_img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)?;

    // 圧縮
    let mut opts = Options::from_preset(3);
    opts.interlace = None;
    opts.strip = oxipng::StripChunks::All;

    let optimized = oxipng::optimize_from_memory(&buf, &opts)?;
    std::fs::write(outpath, optimized)?;
    Ok(())
}
