//! Pattern detection for identifying bloat and junk files.
//!
//! This module provides pattern definitions and matching logic for:
//! - Build artifact directories (node_modules, target, venv, etc.)
//! - Junk files (.DS_Store, *.log, *.bak, etc.)
//! - Cache directories (npm, pip, gradle, etc.)

use crate::models::{BloatPattern, JunkPattern};
use std::path::Path;

// ============================================================================
// Bloat Pattern Detection
// ============================================================================

/// All recognized build artifact patterns
pub const BLOAT_PATTERNS: &[BloatPattern] = &[
    BloatPattern {
        category_id: "node_modules",
        display_name: "Node.js",
        dir_names: &["node_modules"],
    },
    BloatPattern {
        category_id: "rust_target",
        display_name: "Rust",
        dir_names: &["target"],
    },
    BloatPattern {
        category_id: "python_venv",
        display_name: "Python",
        dir_names: &[
            "venv",
            ".venv",
            "__pycache__",
            ".pytest_cache",
            ".mypy_cache",
        ],
    },
    BloatPattern {
        category_id: "git",
        display_name: ".git",
        dir_names: &[".git"],
    },
    BloatPattern {
        category_id: "build_artifacts",
        display_name: "Build Artifacts",
        dir_names: &["dist", "build", ".next", ".nuxt", "out", ".output"],
    },
    BloatPattern {
        category_id: "vendor",
        display_name: "Vendor",
        dir_names: &["vendor"],
    },
    BloatPattern {
        category_id: "java_gradle",
        display_name: "Java/Gradle",
        dir_names: &[".gradle", ".m2"],
    },
];

/// Detect if a path matches a known bloat pattern.
///
/// # Arguments
/// * `path` - Path to check for bloat pattern match
///
/// # Returns
/// * `Some((category_id, display_name))` if matched
/// * `None` if no bloat pattern matches
///
/// # Example
/// ```ignore
/// if let Some((id, name)) = detect_bloat_category(Path::new("./node_modules")) {
///     println!("Found bloat: {}", name); // Output: "Found bloat: Node.js"
/// }
/// ```
pub fn detect_bloat_category(path: &Path) -> Option<(&'static str, &'static str)> {
    if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
        for pattern in BLOAT_PATTERNS {
            if pattern.dir_names.contains(&dir_name) {
                return Some((pattern.category_id, pattern.display_name));
            }
        }
    }
    None
}

// ============================================================================
// Junk File Pattern Detection
// ============================================================================

/// All recognized junk file patterns
pub const JUNK_PATTERNS: &[JunkPattern] = &[
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
    JunkPattern {
        pattern: "desktop.ini",
        category_id: "system",
        display_name: "System Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: ".localized",
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
    JunkPattern {
        pattern: "*.pyo",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.class",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.o",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.obj",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    // Tier 3: Editor Junk
    JunkPattern {
        pattern: "*.swp",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.swo",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.swn",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*~",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.bak",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.backup",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
];

/// Check if a filename matches a junk file pattern.
///
/// Supports glob-style patterns:
/// - Exact match: ".DS_Store"
/// - Extension match: "*.pyc"
/// - Suffix match: "*~"
/// - Prefix match: "test*"
///
/// # Arguments
/// * `filename` - Filename to check
/// * `pattern` - Pattern string to match against
///
/// # Returns
/// * `true` if filename matches pattern
/// * `false` otherwise
///
/// # Example
/// ```ignore
/// assert!(matches_junk_pattern("file.pyc", "*.pyc"));
/// assert!(matches_junk_pattern("file~", "*~"));
/// assert!(matches_junk_pattern(".DS_Store", ".DS_Store"));
/// ```
pub fn matches_junk_pattern(filename: &str, pattern: &str) -> bool {
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

/// Detect if a filename is junk based on known patterns.
///
/// # Arguments
/// * `filename` - Filename to check
///
/// # Returns
/// * `Some((category_id, display_name, safety))` if matched
/// * `None` if no junk pattern matches
///
/// # Example
/// ```ignore
/// if let Some((id, name, safety)) = detect_junk_file(".DS_Store") {
///     println!("Found junk: {} ({})", name, safety);
/// }
/// ```
pub fn detect_junk_file(filename: &str) -> Option<(&'static str, &'static str, &'static str)> {
    for pattern in JUNK_PATTERNS {
        if matches_junk_pattern(filename, pattern.pattern) {
            return Some((pattern.category_id, pattern.display_name, pattern.safety));
        }
    }
    None
}

// ============================================================================
// Cache Patterns
// ============================================================================

/// Known cache directory patterns and their safety levels
pub const CACHE_PATTERNS: &[(&str, &str, &str, &str)] = &[
    // Format: (pattern, category_id, display_name, safety)
    ("node_modules/.cache", "npm_cache", "npm Cache", "safe"),
    (".npm", "npm_cache", "npm Cache", "safe"),
    (".yarn/cache", "yarn_cache", "Yarn Cache", "safe"),
    (".cache/pip", "pip_cache", "Python pip Cache", "safe"),
    (".cache/pip-build", "pip_cache", "Python pip Cache", "safe"),
    (".gradle", "gradle_cache", "Gradle Cache", "caution"),
    (".m2/repository", "maven_cache", "Maven Cache", "caution"),
    (".cache/go-build", "go_cache", "Go Build Cache", "safe"),
    ("Library/Caches", "macos_cache", "macOS System Cache", "dangerous"),
    ("AppData/Local/Cache", "windows_cache", "Windows System Cache", "dangerous"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_bloat_node_modules() {
        let path = Path::new("./node_modules");
        let result = detect_bloat_category(path);
        assert!(result.is_some());
        let (id, name) = result.unwrap();
        assert_eq!(id, "node_modules");
        assert_eq!(name, "Node.js");
    }

    #[test]
    fn test_detect_bloat_rust_target() {
        let path = Path::new("/project/target");
        let result = detect_bloat_category(path);
        assert!(result.is_some());
        let (id, name) = result.unwrap();
        assert_eq!(id, "rust_target");
        assert_eq!(name, "Rust");
    }

    #[test]
    fn test_detect_bloat_no_match() {
        let path = Path::new("/random/directory");
        let result = detect_bloat_category(path);
        assert!(result.is_none());
    }

    #[test]
    fn test_junk_exact_match() {
        let result = detect_junk_file(".DS_Store");
        assert!(result.is_some());
        let (id, name, safety) = result.unwrap();
        assert_eq!(id, "system");
        assert_eq!(safety, "safe");
    }

    #[test]
    fn test_junk_extension_match() {
        let result = detect_junk_file("compiled.pyc");
        assert!(result.is_some());
        let (id, _, safety) = result.unwrap();
        assert_eq!(id, "build");
        assert_eq!(safety, "safe");
    }

    #[test]
    fn test_junk_suffix_match() {
        let result = detect_junk_file("file~");
        assert!(result.is_some());
        let (id, _, safety) = result.unwrap();
        assert_eq!(id, "editor");
        assert_eq!(safety, "safe");
    }

    #[test]
    fn test_junk_no_match() {
        let result = detect_junk_file("regular_file.txt");
        assert!(result.is_none());
    }

    #[test]
    fn test_matches_junk_pattern_extension() {
        assert!(matches_junk_pattern("file.pyc", "*.pyc"));
        assert!(!matches_junk_pattern("file.py", "*.pyc"));
    }

    #[test]
    fn test_matches_junk_pattern_suffix() {
        assert!(matches_junk_pattern("file~", "*~"));
        assert!(!matches_junk_pattern("file", "*~"));
    }

    #[test]
    fn test_matches_junk_pattern_prefix() {
        // Note: prefix patterns are not common in JUNK_PATTERNS, but testing logic
        assert!(matches_junk_pattern("test_file", "test*"));
        assert!(!matches_junk_pattern("other_file", "test*"));
    }

    #[test]
    fn test_matches_junk_pattern_exact() {
        assert!(matches_junk_pattern(".DS_Store", ".DS_Store"));
        assert!(!matches_junk_pattern("DS_Store", ".DS_Store"));
    }

    #[test]
    fn test_bloat_patterns_not_empty() {
        assert!(!BLOAT_PATTERNS.is_empty());
        assert!(BLOAT_PATTERNS.len() >= 7);
    }

    #[test]
    fn test_junk_patterns_not_empty() {
        assert!(!JUNK_PATTERNS.is_empty());
        assert!(JUNK_PATTERNS.len() >= 14);
    }

    #[test]
    fn test_cache_patterns_not_empty() {
        assert!(!CACHE_PATTERNS.is_empty());
    }
}
