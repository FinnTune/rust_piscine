use std::fs::{OpenOptions, metadata};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

pub fn open_or_create(file: &str, content: &str) {
    // Check if the file is read-only
    if let Ok(meta) = metadata(file) {
        if meta.permissions().mode() & 0o200 == 0 {
            panic!("Failed to write to file: File is read-only");
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file)
        .unwrap_or_else(|err| panic!("Failed to open or create file: {}", err));

    if let Err(err) = file.write_all(content.as_bytes()) {
        panic!("Failed to write to file: {}", err);
    }
}
