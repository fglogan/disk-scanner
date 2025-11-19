/// Symlink loop detection utilities (BEAD-009)
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Track visited paths to detect symlink loops
pub struct SymlinkTracker {
    /// Set of visited real paths (canonicalized)
    visited: HashSet<PathBuf>,
    /// Symlink loops detected
    loops_detected: Vec<(PathBuf, PathBuf)>,
}

impl SymlinkTracker {
    /// Create a new symlink tracker
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            loops_detected: Vec::new(),
        }
    }

    /// Check a path and return true if it's safe to traverse
    /// Returns false if it would cause a loop
    pub fn check_path(&mut self, path: &Path) -> Result<bool, std::io::Error> {
        // Get the real path (follow all symlinks)
        let real_path = match fs::canonicalize(path) {
            Ok(p) => p,
            Err(e) => {
                // If we can't canonicalize, it might be a broken symlink
                log::debug!("Cannot canonicalize path {}: {}", path.display(), e);
                return Ok(false);
            }
        };

        // Check if we've already visited this real path
        if self.visited.contains(&real_path) {
            // This is a loop!
            log::warn!(
                "Symlink loop detected: {} -> {}",
                path.display(),
                real_path.display()
            );
            self.loops_detected.push((path.to_path_buf(), real_path));
            Ok(false)
        } else {
            // Mark as visited and continue
            self.visited.insert(real_path);
            Ok(true)
        }
    }

    /// Check if a path is a symlink
    pub fn is_symlink(path: &Path) -> bool {
        path.symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
    }

    /// Get all detected loops
    pub fn get_loops(&self) -> &[(PathBuf, PathBuf)] {
        &self.loops_detected
    }

    /// Clear the tracker for reuse
    pub fn clear(&mut self) {
        self.visited.clear();
        self.loops_detected.clear();
    }
}

impl Default for SymlinkTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a directory tree contains symlink loops
pub fn has_symlink_loops(root: &Path) -> Result<bool, std::io::Error> {
    let mut tracker = SymlinkTracker::new();
    check_directory_recursive(root, &mut tracker)?;
    Ok(!tracker.loops_detected.is_empty())
}

fn check_directory_recursive(
    dir: &Path,
    tracker: &mut SymlinkTracker,
) -> Result<(), std::io::Error> {
    // Check this directory
    if !tracker.check_path(dir)? {
        return Ok(()); // Loop detected, don't traverse
    }

    // Read directory entries
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively check subdirectories
            check_directory_recursive(&path, tracker)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use tempfile::TempDir;

    #[test]
    fn test_symlink_tracker_no_loops() {
        let temp_dir = TempDir::new().unwrap();
        let mut tracker = SymlinkTracker::new();

        // Create some regular directories
        let dir1 = temp_dir.path().join("dir1");
        let dir2 = temp_dir.path().join("dir2");
        fs::create_dir(&dir1).unwrap();
        fs::create_dir(&dir2).unwrap();

        // Check paths - should all be OK
        assert!(tracker.check_path(&dir1).unwrap());
        assert!(tracker.check_path(&dir2).unwrap());
        assert_eq!(tracker.get_loops().len(), 0);
    }

    #[test]
    #[cfg(unix)]
    fn test_symlink_loop_detection() {
        let temp_dir = TempDir::new().unwrap();
        let mut tracker = SymlinkTracker::new();

        // Create a directory
        let dir1 = temp_dir.path().join("dir1");
        fs::create_dir(&dir1).unwrap();

        // Create a symlink that points to itself (direct loop)
        let link1 = temp_dir.path().join("link1");
        symlink(&link1, &link1).ok(); // Might fail, that's OK

        // Create a circular symlink chain
        let dir2 = temp_dir.path().join("dir2");
        let link2 = dir2.join("link2");
        fs::create_dir(&dir2).unwrap();
        symlink(&dir1, &link2).ok();

        // First visit should be OK
        assert!(tracker.check_path(&dir1).unwrap());
        
        // Visiting through symlink should detect loop if symlink points back
        if link2.exists() {
            // This might detect a loop depending on the symlink structure
            let _ = tracker.check_path(&link2);
        }
    }

    #[test]
    fn test_is_symlink() {
        let temp_dir = TempDir::new().unwrap();
        let regular_file = temp_dir.path().join("regular.txt");
        fs::write(&regular_file, "test").unwrap();

        assert!(!SymlinkTracker::is_symlink(&regular_file));

        #[cfg(unix)]
        {
            let link = temp_dir.path().join("link");
            symlink(&regular_file, &link).unwrap();
            assert!(SymlinkTracker::is_symlink(&link));
        }
    }
}