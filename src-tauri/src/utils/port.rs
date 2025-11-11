use anyhow::Result;
use log::{info, warn};
use std::net::TcpListener;

/// Checks if a port is available on localhost; returns Ok(port) or finds alternative.
pub fn check_or_find_port(base_port: u16, max_tries: u16) -> Result<u16, String> {
    let mut port = base_port;
    for _ in 0..max_tries {
        let addr = format!("127.0.0.1:{port}");
        if TcpListener::bind(&addr).is_ok() {
            info!("Port {port} is available");
            return Ok(port);
        }
        warn!("Port {port} in use; trying next");
        port += 1;
    }
    Err(format!("No available port found after {max_tries} tries"))
}
