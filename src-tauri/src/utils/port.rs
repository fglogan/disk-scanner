use anyhow::Result;
use log::{info, warn};
use std::net::TcpListener;

/// Checks if a port is available on localhost; returns Ok(port) or finds alternative.
pub fn check_or_find_port(base_port: u16, max_tries: u16) -> Result<u16, String> {
    let mut port = base_port;
    for _ in 0..max_tries {
        let addr = format!("127.0.0.1:{}", port);
        match TcpListener::bind(&addr) {
            Ok(_) => {
                info!("Port {} is available", port);
                return Ok(port); // Bind succeeds = available (drop listener immediately)
            }
            Err(_) => {
                warn!("Port {} in use; trying next", port);
                port += 1;
            }
        }
    }
    Err(format!("No available port found after {} tries", max_tries))
}
