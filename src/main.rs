use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let help_msg =
        "use `dummyfile <size>` where size is a number followed by either B, KB, MB or GB";

    let units_names = ["kb", "mb", "gb", "b"];
    let units_values = [1000, 1000000, 1000000000, 1];

    if args.len() != 2 {
        println!("wrong amount args. {help_msg}");
        return Ok(());
    }
    let size_arg = &args[1].to_lowercase();
    let mut valid_unit = false;
    let mut unit_value = 0;
    let mut size = 0;

    for (index, unit_name) in units_names.iter().enumerate() {
        if size_arg.ends_with(unit_name) {
            valid_unit = true;
            unit_value = units_values[index];
            let value = &size_arg[0..size_arg.len() - unit_name.len()];
            let value = value.parse();
            match value {
                Ok(value) => size = value,
                Err(_) => {
                    println!("bad number. {help_msg}");
                    return Ok(());
                }
            }
            break;
        }
    }
    if !valid_unit {
        println!("bad unit. {help_msg}");
        return Ok(());
    }
    let total_bytes = size * unit_value;
    println!("{total_bytes}");
    let mut buffer = BufWriter::new(File::create("dummy.bin")?);
    buffer.write_all(&vec![0; total_bytes])?;
    Ok(())
}
