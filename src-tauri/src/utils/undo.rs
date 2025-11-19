//! Undo/restore functionality for deleted files.
//!
//! This module tracks deletion history and provides the ability to restore
//! files from trash when possible.

use crate::error::{ScannerError, ScannerResult};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Maximum number of undo operations to track
const MAX_UNDO_HISTORY: usize = 100;

/// Represents a single deletion operation that can be undone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoOperation {
    pub id: String,
    pub timestamp: DateTime<Local>,
    pub files: Vec<DeletedFile>,
    pub operation_type: OperationType,
    pub total_size: u64,
}

/// Information about a deleted file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedFile {
    pub original_path: String,
    pub trash_path: Option<String>,
    pub size: u64,
    pub was_trashed: bool,
    pub restored: bool,
}

/// Type of deletion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    TrashFiles,
    PermanentDelete,
    CleanupBatch,
}

/// Global undo history manager
static UNDO_HISTORY: Mutex<Option<UndoHistory>> = Mutex::new(None);

/// Manages the undo history for the application
pub struct UndoHistory {
    operations: Vec<UndoOperation>,
    history_file: PathBuf,
}

impl UndoHistory {
    /// Initialize the undo history
    pub fn init() -> ScannerResult<()> {
        let history_file = dirs::data_local_dir()
            .ok_or_else(|| ScannerError::FileAccessSimple("Could not determine data directory".to_string()))?
            .join(".disk-bloat-scanner")
            .join("undo_history.json");
        
        // Ensure directory exists
        if let Some(parent) = history_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to create undo directory: {}", e)))?;
        }
        
        // Load existing history
        let operations = if history_file.exists() {
            let content = fs::read_to_string(&history_file)
                .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to read undo history: {}", e)))?;
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };
        
        let mut history = UNDO_HISTORY.lock().unwrap();
        *history = Some(UndoHistory {
            operations,
            history_file,
        });
        
        Ok(())
    }
    
    /// Record a new deletion operation
    pub fn record_deletion(
        files: Vec<(String, u64, bool)>, // (path, size, was_trashed)
        operation_type: OperationType,
    ) -> ScannerResult<String> {
        let mut history = UNDO_HISTORY.lock().unwrap();
        let history = history.as_mut()
            .ok_or_else(|| ScannerError::DatabaseSimple("Undo history not initialized".to_string()))?;
        
        let id = uuid::Uuid::new_v4().to_string();
        let deleted_files: Vec<DeletedFile> = files
            .into_iter()
            .map(|(path, size, was_trashed)| DeletedFile {
                original_path: path,
                trash_path: None, // Will be set by trash library if available
                size,
                was_trashed,
                restored: false,
            })
            .collect();
        
        let total_size = deleted_files.iter().map(|f| f.size).sum();
        
        let operation = UndoOperation {
            id: id.clone(),
            timestamp: Local::now(),
            files: deleted_files,
            operation_type,
            total_size,
        };
        
        history.operations.push(operation);
        
        // Limit history size
        if history.operations.len() > MAX_UNDO_HISTORY {
            history.operations.remove(0);
        }
        
        // Save to disk
        history.save()?;
        
        Ok(id)
    }
    
    /// Get the undo history
    pub fn get_history() -> ScannerResult<Vec<UndoOperation>> {
        let history = UNDO_HISTORY.lock().unwrap();
        let history = history.as_ref()
            .ok_or_else(|| ScannerError::DatabaseSimple("Undo history not initialized".to_string()))?;
        
        Ok(history.operations.clone())
    }
    
    /// Restore files from a specific operation
    pub fn restore_operation(operation_id: &str) -> ScannerResult<RestoreResult> {
        let mut history = UNDO_HISTORY.lock().unwrap();
        let history = history.as_mut()
            .ok_or_else(|| ScannerError::DatabaseSimple("Undo history not initialized".to_string()))?;
        
        let operation = history.operations.iter_mut()
            .find(|op| op.id == operation_id)
            .ok_or_else(|| ScannerError::DatabaseSimple(format!("Operation {} not found", operation_id)))?;
        
        let mut restored = Vec::new();
        let mut failed = Vec::new();
        
        for file in &mut operation.files {
            if file.restored {
                continue; // Already restored
            }
            
            if !file.was_trashed {
                failed.push((file.original_path.clone(), "File was permanently deleted".to_string()));
                continue;
            }
            
            // Try to restore from trash
            match restore_from_trash(&file.original_path) {
                Ok(_) => {
                    file.restored = true;
                    restored.push(file.original_path.clone());
                }
                Err(e) => {
                    failed.push((file.original_path.clone(), e.to_string()));
                }
            }
        }
        
        // Save updated history
        history.save()?;
        
        Ok(RestoreResult {
            operation_id: operation_id.to_string(),
            restored_count: restored.len(),
            failed_count: failed.len(),
            restored_files: restored,
            failed_files: failed,
        })
    }
    
    /// Clear the undo history
    pub fn clear_history() -> ScannerResult<()> {
        let mut history = UNDO_HISTORY.lock().unwrap();
        let history = history.as_mut()
            .ok_or_else(|| ScannerError::DatabaseSimple("Undo history not initialized".to_string()))?;
        
        history.operations.clear();
        history.save()?;
        
        Ok(())
    }
    
    /// Save history to disk
    fn save(&self) -> ScannerResult<()> {
        let json = serde_json::to_string_pretty(&self.operations)
            .map_err(|e| ScannerError::Other(format!("Failed to serialize undo history: {}", e)))?;
        
        fs::write(&self.history_file, json)
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write undo history: {}", e)))?;
        
        Ok(())
    }
}

/// Result of a restore operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreResult {
    pub operation_id: String,
    pub restored_count: usize,
    pub failed_count: usize,
    pub restored_files: Vec<String>,
    pub failed_files: Vec<(String, String)>, // (path, error)
}

/// Restore a file from trash
fn restore_from_trash(original_path: &str) -> ScannerResult<()> {
    // Use the trash crate to restore
    // Note: The trash crate doesn't directly support restore, so we need to handle this differently
    // For now, we'll return an error indicating manual restore is needed
    
    // In a real implementation, we would:
    // 1. Track where files were moved in the trash
    // 2. Move them back from trash location to original location
    // 3. Update any metadata
    
    Err(ScannerError::FileAccessSimple(
        format!("Automatic restore not yet implemented. Please manually restore {} from trash.", original_path)
    ))
}

/// Get a summary of restorable operations
pub fn get_restorable_operations() -> ScannerResult<Vec<UndoSummary>> {
    let history = UndoHistory::get_history()?;
    
    Ok(history
        .into_iter()
        .filter(|op| {
            // Only show operations that have restorable files
            op.files.iter().any(|f| f.was_trashed && !f.restored)
        })
        .map(|op| {
            let restorable_count = op.files.iter()
                .filter(|f| f.was_trashed && !f.restored)
                .count();
            
            UndoSummary {
                id: op.id,
                timestamp: op.timestamp,
                operation_type: op.operation_type,
                total_files: op.files.len(),
                restorable_files: restorable_count,
                total_size: op.total_size,
            }
        })
        .collect())
}

/// Summary of an undo operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoSummary {
    pub id: String,
    pub timestamp: DateTime<Local>,
    pub operation_type: OperationType,
    pub total_files: usize,
    pub restorable_files: usize,
    pub total_size: u64,
}

// Add uuid dependency for generating unique IDs
use uuid;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_undo_history_init() {
        assert!(UndoHistory::init().is_ok());
    }
    
    #[test]
    fn test_record_deletion() {
        UndoHistory::init().unwrap();
        
        let files = vec![
            ("test/file1.txt".to_string(), 1024, true),
            ("test/file2.txt".to_string(), 2048, false),
        ];
        
        let id = UndoHistory::record_deletion(files, OperationType::CleanupBatch).unwrap();
        assert!(!id.is_empty());
        
        let history = UndoHistory::get_history().unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].files.len(), 2);
    }
}