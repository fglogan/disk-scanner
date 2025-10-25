// Test junk file detection logic
use std::path::Path;

// Copy the junk detection logic
struct JunkPattern {
    pattern: &'static str,
    category_id: &'static str,
    display_name: &'static str,
    safety: &'static str,
}

const JUNK_PATTERNS: &[JunkPattern] = &[
    // Tier 1: System Junk (100% Safe)
    JunkPattern {
        pattern: ".DS_Store",
        category_id: "system",
        display_name: "System Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "Thumbs.db",
        category_id: "system",
        display_name: "System Files",
        safety: "safe",
    },
    // Tier 2: Build Artifacts
    JunkPattern {
        pattern: "*.pyc",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    // Tier 3: Editor Junk
    JunkPattern {
        pattern: "*~",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
];

fn matches_junk_pattern(filename: &str, pattern: &str) -> bool {
    if pattern.starts_with("*.") {
        // Extension match (e.g., "*.pyc")
        let ext = &pattern[1..]; // Remove the *
        filename.ends_with(ext)
    } else if pattern.starts_with('*') && !pattern.ends_with('*') {
        // Suffix match (e.g., "*~" matches "test~")
        let suffix = &pattern[1..]; // Remove the *
        filename.ends_with(suffix)
    } else if pattern.ends_with('*') {
        // Prefix match (e.g., "test*")
        let prefix = &pattern[..pattern.len() - 1];
        filename.starts_with(prefix)
    } else if pattern.contains('*') {
        // Middle wildcard (not implemented for now)
        false
    } else {
        // Exact match (e.g., ".DS_Store")
        filename == pattern
    }
}

fn detect_junk_file(filename: &str) -> Option<(&'static str, &'static str, &'static str)> {
    for pattern in JUNK_PATTERNS {
        if matches_junk_pattern(filename, pattern.pattern) {
            return Some((pattern.category_id, pattern.display_name, pattern.safety));
        }
    }
    None
}

fn main() {
    println!("Testing junk file detection...");
    
    let test_files = vec![
        ".DS_Store",
        "test.DS_Store", 
        "Thumbs.db",
        "test.pyc",
        "test.py",
        "backup~",
        "test~",
        "random.txt",
    ];
    
    for filename in test_files {
        match detect_junk_file(filename) {
            Some((category, display_name, safety)) => {
                println!("✅ '{}' -> {} ({}) - {}", filename, display_name, category, safety);
            }
            None => {
                println!("❌ '{}' -> Not detected as junk", filename);
            }
        }
    }
    
    // Test actual .DS_Store files
    println!("\nTesting actual .DS_Store files in current directory:");
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if filename.contains("DS_Store") {
                        match detect_junk_file(filename) {
                            Some((category, display_name, safety)) => {
                                println!("✅ '{}' -> {} ({}) - {}", filename, display_name, category, safety);
                            }
                            None => {
                                println!("❌ '{}' -> Not detected as junk", filename);
                            }
                        }
                    }
                }
            }
        }
    }
}