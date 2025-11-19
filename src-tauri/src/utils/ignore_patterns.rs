//! Custom ignore patterns for scan operations.
//!
//! This module provides .gitignore-style pattern matching to exclude files
//! and directories from scanning operations.

use crate::error::{ScannerError, ScannerResult};
use ignore::{gitignore::Gitignore, gitignore::GitignoreBuilder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Default ignore patterns for common system and development files
const DEFAULT_PATTERNS: &[&str] = &[
    // System files
    ".DS_Store",
    "Thumbs.db",
    "desktop.ini",
    
    // Version control (already handled by git repos scanner)
    ".git/",
    ".svn/",
    ".hg/",
    
    // IDE/Editor
    ".idea/",
    ".vscode/",
    "*.swp",
    "*.swo",
    "*~",
    
    // Build outputs (already handled by caches scanner)
    "node_modules/",
    "target/",
    "dist/",
    "build/",
    "__pycache__/",
    "*.pyc",
    
    // OS temporary files
    ".Trash/",
    ".Trash-*/",
    "lost+found/",
    
    // Package manager caches (already handled)
    ".npm/",
    ".cargo/",
    ".gradle/",
    ".maven/",
];

/// Custom ignore pattern configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgnoreConfig {
    /// User-defined patterns
    pub custom_patterns: Vec<String>,
    /// Whether to use default patterns
    pub use_defaults: bool,
    /// Path to a .gitignore-style file
    pub ignore_file: Option<String>,
}

impl Default for IgnoreConfig {
    fn default() -> Self {
        Self {
            custom_patterns: Vec::new(),
            use_defaults: true,
            ignore_file: None,
        }
    }
}

/// Global ignore pattern manager
static IGNORE_MANAGER: Mutex<Option<IgnorePatternManager>> = Mutex::new(None);

/// Manages ignore patterns for the application
pub struct IgnorePatternManager {
    config_file: PathBuf,
    gitignore: Arc<Gitignore>,
    config: IgnoreConfig,
}

impl IgnorePatternManager {
    /// Initialize the ignore pattern manager
    pub fn init() -> ScannerResult<()> {
        let config_file = dirs::config_dir()
            .ok_or_else(|| ScannerError::FileAccessSimple("Could not determine config directory".to_string()))?
            .join("disk-bloat-scanner")
            .join("ignore_patterns.json");
        
        // Ensure directory exists
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to create config directory: {}", e)))?;
        }
        
        // Load existing config or use defaults
        let config = if config_file.exists() {
            let content = fs::read_to_string(&config_file)
                .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to read ignore config: {}", e)))?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            IgnoreConfig::default()
        };
        
        // Build gitignore matcher
        let gitignore = Self::build_gitignore(&config)?;
        
        let mut manager = IGNORE_MANAGER.lock().unwrap();
        *manager = Some(IgnorePatternManager {
            config_file,
            gitignore: Arc::new(gitignore),
            config,
        });
        
        Ok(())
    }
    
    /// Build a Gitignore matcher from config
    fn build_gitignore(config: &IgnoreConfig) -> ScannerResult<Gitignore> {
        let mut builder = GitignoreBuilder::new("");
        
        // Add default patterns if enabled
        if config.use_defaults {
            for pattern in DEFAULT_PATTERNS {
                builder.add_line(None, pattern)
                    .map_err(|e| ScannerError::Other(format!("Invalid pattern: {}", e)))?;
            }
        }
        
        // Add custom patterns
        for pattern in &config.custom_patterns {
            builder.add_line(None, pattern)
                .map_err(|e| ScannerError::Other(format!("Invalid pattern {}: {}", pattern, e)))?;
        }
        
        // Add patterns from file if specified
        if let Some(file_path) = &config.ignore_file {
            if Path::new(file_path).exists() {
                let content = fs::read_to_string(file_path)
                    .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to read ignore file: {}", e)))?;
                
                for line in content.lines() {
                    let line = line.trim();
                    if !line.is_empty() && !line.starts_with('#') {
                        builder.add_line(None, line)
                            .map_err(|e| ScannerError::Other(format!("Invalid pattern in file: {}", e)))?;
                    }
                }
            }
        }
        
        builder.build()
            .map_err(|e| ScannerError::Other(format!("Failed to build gitignore: {}", e)))
    }
    
    /// Get the current ignore configuration
    pub fn get_config() -> ScannerResult<IgnoreConfig> {
        let manager = IGNORE_MANAGER.lock().unwrap();
        let manager = manager.as_ref()
            .ok_or_else(|| ScannerError::DatabaseSimple("Ignore manager not initialized".to_string()))?;
        
        Ok(manager.config.clone())
    }
    
    /// Update the ignore configuration
    pub fn update_config(config: IgnoreConfig) -> ScannerResult<()> {
        let mut manager = IGNORE_MANAGER.lock().unwrap();
        let manager = manager.as_mut()
            .ok_or_else(|| ScannerError::DatabaseSimple("Ignore manager not initialized".to_string()))?;
        
        // Build new gitignore
        let gitignore = Self::build_gitignore(&config)?;
        
        // Update manager
        manager.config = config;
        manager.gitignore = Arc::new(gitignore);
        
        // Save to disk
        let json = serde_json::to_string_pretty(&manager.config)
            .map_err(|e| ScannerError::Other(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(&manager.config_file, json)
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write config: {}", e)))?;
        
        Ok(())
    }
    
    /// Check if a path should be ignored
    pub fn should_ignore(path: &Path) -> bool {
        let manager = IGNORE_MANAGER.lock().unwrap();
        if let Some(manager) = manager.as_ref() {
            let gitignore = Arc::clone(&manager.gitignore);
            // Drop the lock before checking (to avoid holding it during path operations)
            drop(manager);
            
            gitignore.matched(path, path.is_dir()).is_ignore()
        } else {
            false
        }
    }
    
    /// Add a custom pattern
    pub fn add_pattern(pattern: String) -> ScannerResult<()> {
        let mut config = Self::get_config()?;
        if !config.custom_patterns.contains(&pattern) {
            config.custom_patterns.push(pattern);
            Self::update_config(config)?;
        }
        Ok(())
    }
    
    /// Remove a custom pattern
    pub fn remove_pattern(pattern: &str) -> ScannerResult<()> {
        let mut config = Self::get_config()?;
        config.custom_patterns.retain(|p| p != pattern);
        Self::update_config(config)?;
        Ok(())
    }
    
    /// Clear all custom patterns
    pub fn clear_patterns() -> ScannerResult<()> {
        let mut config = Self::get_config()?;
        config.custom_patterns.clear();
        Self::update_config(config)?;
        Ok(())
    }
    
    /// Get all active patterns (defaults + custom)
    pub fn get_all_patterns() -> ScannerResult<Vec<String>> {
        let config = Self::get_config()?;
        let mut patterns = Vec::new();
        
        if config.use_defaults {
            patterns.extend(DEFAULT_PATTERNS.iter().map(|s| s.to_string()));
        }
        
        patterns.extend(config.custom_patterns);
        
        Ok(patterns)
    }
}

/// Apply ignore patterns to a list of paths
pub fn filter_ignored_paths(paths: Vec<String>) -> Vec<String> {
    paths.into_iter()
        .filter(|path| !IgnorePatternManager::should_ignore(Path::new(path)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_ignore_patterns_init() {
        assert!(IgnorePatternManager::init().is_ok());
    }
    
    #[test]
    fn test_default_patterns() {
        IgnorePatternManager::init().unwrap();
        
        // Test that default patterns work
        assert!(IgnorePatternManager::should_ignore(Path::new(".DS_Store")));
        assert!(IgnorePatternManager::should_ignore(Path::new("node_modules")));
        assert!(IgnorePatternManager::should_ignore(Path::new(".git")));
        
        // Test that non-ignored paths pass through
        assert!(!IgnorePatternManager::should_ignore(Path::new("important.txt")));
    }
    
    #[test]
    fn test_custom_patterns() {
        IgnorePatternManager::init().unwrap();
        
        // Add custom pattern
        IgnorePatternManager::add_pattern("*.tmp".to_string()).unwrap();
        
        // Test it works
        assert!(IgnorePatternManager::should_ignore(Path::new("test.tmp")));
        assert!(!IgnorePatternManager::should_ignore(Path::new("test.txt")));
        
        // Remove pattern
        IgnorePatternManager::remove_pattern("*.tmp").unwrap();
        assert!(!IgnorePatternManager::should_ignore(Path::new("test.tmp")));
    }
}