pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => {
            server.unwrap() // Using unwrap to generate the default panic message
        },
        Security::High => {
            server.clone().unwrap_or_else(|_| panic!("ERROR: program stops"))
        },
        Security::Medium => {
            server.clone().unwrap_or("WARNING: check the server".to_string())
        },
        Security::Low => {
            server.clone().unwrap_or_else(|e| format!("Not found: {}", e))
        },
        Security::BlockServer => {
            if let Ok(message) = server.clone() {
                panic!("{}", message);
            } else {
                server.unwrap_err()
            }
        },
    }
}
