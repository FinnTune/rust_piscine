mod log_init;
use log::{error, info, warn};

fn main() {
    log_init::init_logger();
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
}
