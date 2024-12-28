use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let amt = 3*10_usize.pow(9);
    let mut buffer = BufWriter::new(File::create("woo.bin")?);
    buffer.write_all(&vec!(0;amt))?;
    Ok(())
}
