use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("./output.txt")?;
    write!(&mut file, "Hello, file!\n")?;
    write!(&mut file, "Hello, again!\n")?;
    Ok(())
}