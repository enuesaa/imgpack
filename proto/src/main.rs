use anyhow::Result;
use color_quant::NeuQuant;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use oxipng::{InFile, Options, OutFile};

fn main() -> Result<()> {
    let input_path = "input.png";
    let output_path = "output.png";

    let palette_size = 256; // 減色後の色数
    println!("Reducing {} to {} colors...", input_path, palette_size);

    // 入力画像読み込み
    let img = image::open(input_path)?;
    let (width, height) = img.dimensions();

    // RGBAに変換
    let rgba_pixels: Vec<u8> = img.to_rgba8().into_vec();

    // NeuQuant で減色
    let nq = NeuQuant::new(10, palette_size, &rgba_pixels);

    // 各ピクセルをパレットインデックスに置き換え
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

    // 減色結果で画像を再構築
    let mut output_img: RgbaImage = ImageBuffer::new(width, height);
    for (i, idx) in indexed_pixels.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        output_img.put_pixel(x, y, palette[*idx as usize]);
    }

    // 保存
    output_img.save(output_path)?;
    println!("Saved reduced-color image to {}", output_path);

    let outfile = "out.png";

    let mut opts = Options::max_compression();
    opts.interlace = None;
    opts.strip = oxipng::StripChunks::All;

    oxipng::optimize(
        &InFile::Path(output_path.into()),
        &OutFile::from_path(outfile.into()),
        &opts,
    )?;

    Ok(())
}

