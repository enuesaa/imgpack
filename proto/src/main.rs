use anyhow::Result;
use oxipng::{InFile, OutFile, Options};

fn main() -> Result<()> {
    let infile = "input.png";
    let outfile = "out.png";

    let opts = Options {
        strip: oxipng::StripChunks::All,
        ..Options::max_compression()
    };

    oxipng::optimize(
        &InFile::Path(infile.into()),
        &OutFile::from_path(outfile.into()),
        &opts,
    )?;

    println!("Compressed PNG saved to {}", outfile);
    Ok(())
}
