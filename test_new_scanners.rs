// Test the new scanner commands
use std::path::PathBuf;

// Test data structures (simplified versions)
#[derive(Debug)]
struct CacheEntry {
    path: String,
    size_mb: f32,
    cache_type: String,
    safety: String,
    description: String,
}

#[derive(Debug)]
struct GitEntry {
    path: String,
    size_mb: f32,
    entry_type: String,
    description: String,
    safety: String,
    actionable: bool,
}

fn main() {
    println!("Testing new scanner functionality...");
    
    // Test 1: Check if we can find cache directories in current project
    println!("\n=== Testing Cache Detection ===");
    
    let cache_patterns = [
        ("node_modules", "nodejs", "Node.js Dependencies"),
        ("target", "rust", "Cargo Target"),
        ("__pycache__", "python", "Python Bytecode"),
        (".git", "git", ".git Repository"),
    ];
    
    for (pattern, category, display_name) in &cache_patterns {
        if PathBuf::from(pattern).exists() {
            println!("✅ Found {}: {}", display_name, pattern);
            
            // Try to get size
            if let Ok(metadata) = std::fs::metadata(pattern) {
                let size_mb = metadata.len() as f32 / 1_048_576.0;
                println!("   Size: {:.1} MB", size_mb);
            }
        } else {
            println!("❌ {}: {} not found", display_name, pattern);
        }
    }
    
    // Test 2: Check for .git repositories
    println!("\n=== Testing Git Repository Detection ===");
    
    if PathBuf::from(".git").exists() {
        println!("✅ Found .git repository in current directory");
        
        // Check .git subdirectories
        let git_subdirs = ["objects", "refs", "logs", "pack"];
        for subdir in &git_subdirs {
            let git_path = PathBuf::from(".git").join(subdir);
            if git_path.exists() {
                if let Ok(metadata) = std::fs::metadata(&git_path) {
                    let size_mb = metadata.len() as f32 / 1_048_576.0;
                    println!("   ✅ .git/{}: {:.1} MB", subdir, size_mb);
                }
            } else {
                println!("   ❌ .git/{}: not found", subdir);
            }
        }
        
        // Try to run git command to check for large files
        println!("\n=== Testing Git Large File Detection ===");
        if let Ok(output) = std::process::Command::new("git")
            .args(&["rev-list", "--objects", "--all", "--", "."])
            .output()
        {
            if output.status.success() {
                let lines = String::from_utf8_lossy(&output.stdout);
                let line_count = lines.lines().count();
                println!("✅ Git command working - found {} objects", line_count);
                
                // Show first few objects
                for (i, line) in lines.lines().take(5).enumerate() {
                    if let Some((hash, path)) = line.split_once(' ') {
                        println!("   {}: {} -> {}", i + 1, &hash[..8], path);
                    }
                }
            } else {
                println!("❌ Git command failed");
            }
        } else {
            println!("❌ Could not run git command");
        }
    } else {
        println!("❌ No .git repository found");
    }
    
    // Test 3: Check directory size calculation
    println!("\n=== Testing Directory Size Calculation ===");
    
    let test_dirs = ["src", "src-tauri", "node_modules", "target"];
    for dir in &test_dirs {
        if PathBuf::from(dir).exists() {
            let size = get_dir_size(dir);
            println!("✅ {}: {:.1} MB", dir, size);
        } else {
            println!("❌ {}: not found", dir);
        }
    }
    
    println!("\n=== Test Complete ===");
}

fn get_dir_size(path: &str) -> f32 {
    let mut total_size = 0u64;
    
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                if let Ok(metadata) = entry_path.metadata() {
                    total_size += metadata.len();
                }
            } else if entry_path.is_dir() {
                // Recursively calculate subdirectory size (limit depth to avoid infinite loops)
                let path_str = entry_path.to_string_lossy();
                if path_str.matches('/').count() < 10 { // Limit depth
                    total_size += get_dir_size_recursive(&entry_path) as u64;
                }
            }
        }
    }
    
    total_size as f32 / 1_048_576.0
}

fn get_dir_size_recursive(path: &std::path::Path) -> u64 {
    let mut total_size = 0u64;
    
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                if let Ok(metadata) = entry_path.metadata() {
                    total_size += metadata.len();
                }
            } else if entry_path.is_dir() {
                let path_str = entry_path.to_string_lossy();
                if path_str.matches('/').count() < 10 { // Limit depth
                    total_size += get_dir_size_recursive(&entry_path);
                }
            }
        }
    }
    
    total_size
}