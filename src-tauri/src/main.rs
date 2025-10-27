//! Disk Bloat Scanner - Tauri application entry point
//!
//! This binary serves as the main entry point for the Disk Bloat Scanner desktop application.
//! It initializes the Tauri runtime and delegates to the library's `run()` function.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    disk_bloat_scanner_lib::run();
}
