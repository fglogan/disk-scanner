//! Export functionality for scan results to CSV and JSON formats.
//!
//! This module provides utilities to export various scan results (duplicates,
//! large files, junk files, etc.) to portable formats for external analysis.

use crate::error::{ScannerError, ScannerResult};
use crate::models::{
    DuplicateGroup, FileEntry, GitRepoInfo, JunkCategory, NodeModulesInfo, ProjectScannerResponse,
};
use chrono::{DateTime, Local};
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Export format options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Json,
}

/// Export request from frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub format: ExportFormat,
    pub data_type: ExportDataType,
    pub output_path: String,
}

/// Type of data to export
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportDataType {
    Duplicates(Vec<DuplicateGroup>),
    LargeFiles(Vec<FileEntry>),
    JunkFiles(Vec<(FileEntry, JunkCategory)>),
    NodeModules(Vec<NodeModulesInfo>),
    GitRepos(Vec<GitRepoInfo>),
    ProjectScan(ProjectScannerResponse),
    AllResults(ExportAllData),
}

/// Container for all scan results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAllData {
    pub scan_timestamp: DateTime<Local>,
    pub scan_root: String,
    pub duplicates: Vec<DuplicateGroup>,
    pub large_files: Vec<FileEntry>,
    pub junk_files: Vec<(FileEntry, JunkCategory)>,
    pub node_modules: Vec<NodeModulesInfo>,
    pub git_repos: Vec<GitRepoInfo>,
}

/// Export scan results to file
pub fn export_data(request: ExportRequest) -> ScannerResult<String> {
    let output_path = Path::new(&request.output_path);
    
    // Ensure parent directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to create export directory: {}", e)))?;
    }

    match request.format {
        ExportFormat::Csv => export_to_csv(&request.data_type, output_path),
        ExportFormat::Json => export_to_json(&request.data_type, output_path),
    }
}

/// Export data to CSV format
fn export_to_csv(data_type: &ExportDataType, path: &Path) -> ScannerResult<String> {
    let file = File::create(path)
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to create CSV file: {}", e)))?;
    
    match data_type {
        ExportDataType::Duplicates(groups) => export_duplicates_csv(file, groups),
        ExportDataType::LargeFiles(files) => export_large_files_csv(file, files),
        ExportDataType::JunkFiles(files) => export_junk_files_csv(file, files),
        ExportDataType::NodeModules(modules) => export_node_modules_csv(file, modules),
        ExportDataType::GitRepos(repos) => export_git_repos_csv(file, repos),
        ExportDataType::ProjectScan(scan) => export_project_scan_csv(file, scan),
        ExportDataType::AllResults(all_data) => export_all_data_csv(file, all_data),
    }?;
    
    Ok(format!("Exported to: {}", path.display()))
}

/// Export data to JSON format
fn export_to_json(data_type: &ExportDataType, path: &Path) -> ScannerResult<String> {
    let json_data = match data_type {
        ExportDataType::Duplicates(groups) => serde_json::to_string_pretty(groups),
        ExportDataType::LargeFiles(files) => serde_json::to_string_pretty(files),
        ExportDataType::JunkFiles(files) => serde_json::to_string_pretty(files),
        ExportDataType::NodeModules(modules) => serde_json::to_string_pretty(modules),
        ExportDataType::GitRepos(repos) => serde_json::to_string_pretty(repos),
        ExportDataType::ProjectScan(scan) => serde_json::to_string_pretty(scan),
        ExportDataType::AllResults(all_data) => serde_json::to_string_pretty(all_data),
    }
    .map_err(|e| ScannerError::Other(format!("Failed to serialize to JSON: {}", e)))?;
    
    std::fs::write(path, json_data)
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write JSON file: {}", e)))?;
    
    Ok(format!("Exported to: {}", path.display()))
}

// CSV export implementations for each data type

fn export_duplicates_csv(file: File, groups: &[DuplicateGroup]) -> ScannerResult<()> {
    let mut wtr = Writer::from_writer(file);
    
    // Write header
    wtr.write_record(&["Hash", "File Path", "Size (bytes)", "Group Size", "Duplicate Count"])
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV header: {}", e)))?;
    
    for group in groups {
        for path in &group.paths {
            wtr.write_record(&[
                &group.hash,
                path,
                &group.size.to_string(),
                &group.total_size.to_string(),
                &group.count.to_string(),
            ])
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV record: {}", e)))?;
        }
    }
    
    wtr.flush()
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to flush CSV: {}", e)))?;
    Ok(())
}

fn export_large_files_csv(file: File, files: &[FileEntry]) -> ScannerResult<()> {
    let mut wtr = Writer::from_writer(file);
    
    wtr.write_record(&["Path", "Name", "Size (bytes)", "Modified"])
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV header: {}", e)))?;
    
    for file_entry in files {
        wtr.write_record(&[
            &file_entry.path,
            &file_entry.name,
            &file_entry.size.to_string(),
            &file_entry.modified.to_string(),
        ])
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV record: {}", e)))?;
    }
    
    wtr.flush()
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to flush CSV: {}", e)))?;
    Ok(())
}

fn export_junk_files_csv(file: File, files: &[(FileEntry, JunkCategory)]) -> ScannerResult<()> {
    let mut wtr = Writer::from_writer(file);
    
    wtr.write_record(&["Path", "Name", "Size (bytes)", "Category", "Modified"])
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV header: {}", e)))?;
    
    for (file_entry, category) in files {
        wtr.write_record(&[
            &file_entry.path,
            &file_entry.name,
            &file_entry.size.to_string(),
            &format!("{:?}", category),
            &file_entry.modified.to_string(),
        ])
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV record: {}", e)))?;
    }
    
    wtr.flush()
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to flush CSV: {}", e)))?;
    Ok(())
}

fn export_node_modules_csv(file: File, modules: &[NodeModulesInfo]) -> ScannerResult<()> {
    let mut wtr = Writer::from_writer(file);
    
    wtr.write_record(&["Path", "Size (bytes)", "Package Count", "Last Modified"])
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV header: {}", e)))?;
    
    for module in modules {
        wtr.write_record(&[
            &module.path,
            &module.size.to_string(),
            &module.package_count.to_string(),
            &module.last_modified.to_string(),
        ])
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV record: {}", e)))?;
    }
    
    wtr.flush()
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to flush CSV: {}", e)))?;
    Ok(())
}

fn export_git_repos_csv(file: File, repos: &[GitRepoInfo]) -> ScannerResult<()> {
    let mut wtr = Writer::from_writer(file);
    
    wtr.write_record(&[
        "Path",
        "Name", 
        ".git Size (bytes)",
        "Status",
        "Last Commit",
        "Branch",
        "Remote URL",
        "Uncommitted Files"
    ])
    .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV header: {}", e)))?;
    
    for repo in repos {
        wtr.write_record(&[
            &repo.path,
            &repo.name,
            &repo.git_size.to_string(),
            &repo.status,
            &repo.last_commit,
            &repo.branch,
            &repo.remote_url.clone().unwrap_or_default(),
            &repo.uncommitted_files.to_string(),
        ])
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write CSV record: {}", e)))?;
    }
    
    wtr.flush()
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to flush CSV: {}", e)))?;
    Ok(())
}

fn export_project_scan_csv(file: File, scan: &ProjectScannerResponse) -> ScannerResult<()> {
    let mut wtr = Writer::from_writer(file);
    
    // Write summary
    writeln!(&mut wtr.into_inner(), "Project Scan Summary")
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write summary: {}", e)))?;
    writeln!(&mut wtr.into_inner(), "Total Repositories,{}", scan.total_repos)
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write total: {}", e)))?;
    writeln!(&mut wtr.into_inner(), "Total Size (bytes),{}", scan.total_size)
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write size: {}", e)))?;
    writeln!(&mut wtr.into_inner(), "Scan Duration (ms),{}", scan.scan_duration_ms)
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write duration: {}", e)))?;
    writeln!(&mut wtr.into_inner(), "")
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write newline: {}", e)))?;
    
    // Write repositories
    let mut wtr = Writer::from_writer(wtr.into_inner());
    export_git_repos_csv(wtr.into_inner(), &scan.repositories)?;
    
    Ok(())
}

fn export_all_data_csv(file: File, all_data: &ExportAllData) -> ScannerResult<()> {
    let mut writer = file;
    
    // Write metadata
    writeln!(&mut writer, "Disk Bloat Scanner Export")
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write header: {}", e)))?;
    writeln!(&mut writer, "Scan Timestamp,{}", all_data.scan_timestamp)
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write timestamp: {}", e)))?;
    writeln!(&mut writer, "Scan Root,{}", all_data.scan_root)
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write root: {}", e)))?;
    writeln!(&mut writer, "")
        .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write newline: {}", e)))?;
    
    // Write each section
    if !all_data.duplicates.is_empty() {
        writeln!(&mut writer, "=== DUPLICATE FILES ===")
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write section: {}", e)))?;
        let wtr = Writer::from_writer(writer);
        export_duplicates_csv(wtr.into_inner(), &all_data.duplicates)?;
        writer = wtr.into_inner();
        writeln!(&mut writer, "")
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write newline: {}", e)))?;
    }
    
    if !all_data.large_files.is_empty() {
        writeln!(&mut writer, "=== LARGE FILES ===")
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write section: {}", e)))?;
        let wtr = Writer::from_writer(writer);
        export_large_files_csv(wtr.into_inner(), &all_data.large_files)?;
        writer = wtr.into_inner();
        writeln!(&mut writer, "")
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write newline: {}", e)))?;
    }
    
    if !all_data.junk_files.is_empty() {
        writeln!(&mut writer, "=== JUNK FILES ===")
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write section: {}", e)))?;
        let wtr = Writer::from_writer(writer);
        export_junk_files_csv(wtr.into_inner(), &all_data.junk_files)?;
        writer = wtr.into_inner();
        writeln!(&mut writer, "")
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write newline: {}", e)))?;
    }
    
    if !all_data.node_modules.is_empty() {
        writeln!(&mut writer, "=== NODE MODULES ===")
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write section: {}", e)))?;
        let wtr = Writer::from_writer(writer);
        export_node_modules_csv(wtr.into_inner(), &all_data.node_modules)?;
        writer = wtr.into_inner();
        writeln!(&mut writer, "")
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write newline: {}", e)))?;
    }
    
    if !all_data.git_repos.is_empty() {
        writeln!(&mut writer, "=== GIT REPOSITORIES ===")
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write section: {}", e)))?;
        let wtr = Writer::from_writer(writer);
        export_git_repos_csv(wtr.into_inner(), &all_data.git_repos)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_export_json() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test.json");
        
        let files = vec![FileEntry {
            path: "/test/file.txt".to_string(),
            name: "file.txt".to_string(),
            size: 1024,
            modified: 1234567890,
        }];
        
        let request = ExportRequest {
            format: ExportFormat::Json,
            data_type: ExportDataType::LargeFiles(files),
            output_path: output_path.to_string_lossy().to_string(),
        };
        
        let result = export_data(request);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }
    
    #[test]
    fn test_export_csv() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test.csv");
        
        let files = vec![FileEntry {
            path: "/test/file.txt".to_string(),
            name: "file.txt".to_string(),
            size: 1024,
            modified: 1234567890,
        }];
        
        let request = ExportRequest {
            format: ExportFormat::Csv,
            data_type: ExportDataType::LargeFiles(files),
            output_path: output_path.to_string_lossy().to_string(),
        };
        
        let result = export_data(request);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }
}