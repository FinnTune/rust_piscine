extern crate log;
extern crate log4rs;
extern crate serde_yaml;

pub fn init_logger() {
    // Initialize the logger using the log4rs.yaml configuration file
    log4rs::init_file("src/log4rs.yaml", Default::default()).expect("Failed to initialize logger");
}
