use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
pub fn log_to_file(message: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log");
    if let Ok(file) = file {
        let mut writer = BufWriter::new(&file);
        writeln!(writer, "{}", message).expect("Error writing to log file");
    }
}
