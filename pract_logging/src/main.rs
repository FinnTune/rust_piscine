mod log_init;
use log::{error, info, warn};
use std::fs::rename;
use chrono::Local;

fn main() {

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let new_filename = format!("log_{}.txt", timestamp);

    let _ = rename("log.txt", &new_filename);

    log_init::init_logger();

    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
}
