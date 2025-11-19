//! Compression analysis and recommendations (BEAD-034)
//! 
//! Estimates compression savings, identifies compressible files,
//! provides compression recommendations, and integrates with cleanup.

use crate::error::DiskBlotResult;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use log::{info, debug};
use std::collections::HashMap;

/// Compression algorithm types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionAlgorithm {
    /// ZIP compression
    Zip,
    /// GZIP compression
    Gzip,
    /// 7-Zip compression
    SevenZip,
    /// TAR with GZIP
    TarGz,
    /// RAR compression
    Rar,
    /// Brotli compression
    Brotli,
    /// Zstandard compression
    Zstd,
}

/// File compressibility category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Compressibility {
    /// Highly compressible (text, logs, source code)
    High,
    /// Moderately compressible (executables, databases)
    Medium,
    /// Low compressibility (images, videos)
    Low,
    /// Already compressed (zip, mp3, jpg)
    AlreadyCompressed,
    /// Should not compress (system files)
    DoNotCompress,
}

/// Compression analysis result for a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionAnalysis {
    /// File path
    pub path: PathBuf,
    /// Original size in bytes
    pub original_size: u64,
    /// Estimated compressed size
    pub estimated_compressed_size: u64,
    /// Potential savings in bytes
    pub potential_savings: u64,
    /// Compression ratio (0.0 to 1.0)
    pub compression_ratio: f64,
    /// Compressibility category
    pub compressibility: Compressibility,
    /// Recommended algorithm
    pub recommended_algorithm: CompressionAlgorithm,
    /// Whether compression is recommended
    pub should_compress: bool,
    /// Reason for recommendation
    pub recommendation_reason: String,
}

/// Compression statistics for a directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    /// Total files analyzed
    pub total_files: usize,
    /// Total size in bytes
    pub total_size: u64,
    /// Total potential savings
    pub total_potential_savings: u64,
    /// Files by compressibility
    pub files_by_compressibility: HashMap<Compressibility, usize>,
    /// Size by compressibility
    pub size_by_compressibility: HashMap<Compressibility, u64>,
    /// Top compressible files
    pub top_compressible_files: Vec<CompressionAnalysis>,
}

/// Compression analyzer
pub struct CompressionAnalyzer {
    /// Minimum file size to consider for compression (bytes)
    min_file_size: u64,
    /// Maximum number of files to analyze in detail
    max_files_to_analyze: usize,
}

impl CompressionAnalyzer {
    /// Create a new compression analyzer
    pub fn new() -> Self {
        Self {
            min_file_size: 1024 * 10, // 10 KB minimum
            max_files_to_analyze: 10000,
        }
    }

    /// Analyze a single file for compression potential
    pub fn analyze_file(&self, path: &Path) -> DiskBlotResult<CompressionAnalysis> {
        let metadata = std::fs::metadata(path)?;
        let original_size = metadata.len();
        
        // Skip small files
        if original_size < self.min_file_size {
            return Ok(CompressionAnalysis {
                path: path.to_path_buf(),
                original_size,
                estimated_compressed_size: original_size,
                potential_savings: 0,
                compression_ratio: 1.0,
                compressibility: Compressibility::DoNotCompress,
                recommended_algorithm: CompressionAlgorithm::Zip,
                should_compress: false,
                recommendation_reason: "File too small to compress".to_string(),
            });
        }
        
        // Determine compressibility based on file extension and content
        let compressibility = self.determine_compressibility(path);
        let (estimated_ratio, algorithm) = self.estimate_compression_ratio(&compressibility, path);
        let estimated_compressed_size = (original_size as f64 * estimated_ratio) as u64;
        let potential_savings = original_size.saturating_sub(estimated_compressed_size);
        
        let should_compress = matches!(compressibility, Compressibility::High | Compressibility::Medium)
            && potential_savings > 1024 * 100 // Save at least 100KB
            && estimated_ratio < 0.8; // At least 20% compression
        
        let recommendation_reason = match compressibility {
            Compressibility::High => "Highly compressible text-based file".to_string(),
            Compressibility::Medium => "Moderately compressible file".to_string(),
            Compressibility::Low => "Low compression potential".to_string(),
            Compressibility::AlreadyCompressed => "File is already compressed".to_string(),
            Compressibility::DoNotCompress => "System or critical file".to_string(),
        };
        
        Ok(CompressionAnalysis {
            path: path.to_path_buf(),
            original_size,
            estimated_compressed_size,
            potential_savings,
            compression_ratio: estimated_ratio,
            compressibility,
            recommended_algorithm: algorithm,
            should_compress,
            recommendation_reason,
        })
    }

    /// Determine file compressibility based on extension and patterns
    fn determine_compressibility(&self, path: &Path) -> Compressibility {
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // Already compressed files
        let compressed_extensions = ["zip", "gz", "7z", "rar", "bz2", "xz", "zst", "br",
                                    "jpg", "jpeg", "png", "gif", "webp", "avif",
                                    "mp3", "mp4", "m4a", "aac", "flac", "ogg",
                                    "avi", "mkv", "mov", "webm"];
        if compressed_extensions.contains(&extension.as_str()) {
            return Compressibility::AlreadyCompressed;
        }
        
        // System files that shouldn't be compressed
        let system_extensions = ["sys", "dll", "so", "dylib", "framework"];
        if system_extensions.contains(&extension.as_str()) {
            return Compressibility::DoNotCompress;
        }
        
        // Highly compressible text files
        let text_extensions = ["txt", "log", "md", "rst", "tex", "csv", "tsv",
                              "xml", "json", "yaml", "yml", "toml", "ini", "conf",
                              "html", "htm", "css", "js", "ts", "jsx", "tsx",
                              "py", "rb", "go", "rs", "java", "c", "cpp", "h", "hpp",
                              "sql", "sh", "bash", "ps1", "bat"];
        if text_extensions.contains(&extension.as_str()) {
            return Compressibility::High;
        }
        
        // Log files by name pattern
        if file_name.contains("log") || file_name.ends_with(".log") {
            return Compressibility::High;
        }
        
        // Moderately compressible files
        let medium_extensions = ["exe", "app", "deb", "rpm", "dmg", "pkg",
                               "db", "sqlite", "mdb", "accdb",
                               "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx"];
        if medium_extensions.contains(&extension.as_str()) {
            return Compressibility::Medium;
        }
        
        // Default to medium for unknown files
        Compressibility::Medium
    }

    /// Estimate compression ratio based on file type
    fn estimate_compression_ratio(&self, compressibility: &Compressibility, path: &Path) -> (f64, CompressionAlgorithm) {
        match compressibility {
            Compressibility::High => {
                // Text files compress very well
                // Could sample file content for better estimation
                (0.2, CompressionAlgorithm::Gzip)
            }
            Compressibility::Medium => {
                // Binary files compress moderately
                (0.6, CompressionAlgorithm::Zip)
            }
            Compressibility::Low => {
                // Poor compression
                (0.9, CompressionAlgorithm::Zip)
            }
            Compressibility::AlreadyCompressed => {
                // No additional compression
                (1.0, CompressionAlgorithm::Zip)
            }
            Compressibility::DoNotCompress => {
                // Should not compress
                (1.0, CompressionAlgorithm::Zip)
            }
        }
    }

    /// Analyze a directory for compression opportunities
    pub async fn analyze_directory(&self, root: &Path) -> DiskBlotResult<CompressionStats> {
        let mut total_files = 0;
        let mut total_size = 0u64;
        let mut total_potential_savings = 0u64;
        let mut files_by_compressibility = HashMap::new();
        let mut size_by_compressibility = HashMap::new();
        let mut all_analyses = Vec::new();
        
        // Initialize maps
        for comp in [Compressibility::High, Compressibility::Medium, 
                    Compressibility::Low, Compressibility::AlreadyCompressed,
                    Compressibility::DoNotCompress] {
            files_by_compressibility.insert(comp.clone(), 0);
            size_by_compressibility.insert(comp, 0);
        }
        
        for entry in walkdir::WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .take(self.max_files_to_analyze)
        {
            if let Ok(analysis) = self.analyze_file(entry.path()) {
                total_files += 1;
                total_size += analysis.original_size;
                total_potential_savings += analysis.potential_savings;
                
                *files_by_compressibility.get_mut(&analysis.compressibility).unwrap() += 1;
                *size_by_compressibility.get_mut(&analysis.compressibility).unwrap() += analysis.original_size;
                
                if analysis.should_compress {
                    all_analyses.push(analysis);
                }
            }
        }
        
        // Sort by potential savings to get top files
        all_analyses.sort_by_key(|a| std::cmp::Reverse(a.potential_savings));
        let top_compressible_files = all_analyses.into_iter().take(20).collect();
        
        Ok(CompressionStats {
            total_files,
            total_size,
            total_potential_savings,
            files_by_compressibility,
            size_by_compressibility,
            top_compressible_files,
        })
    }

    /// Compress a file with the recommended algorithm
    pub async fn compress_file(&self, analysis: &CompressionAnalysis, output_path: &Path) -> DiskBlotResult<u64> {
        info!("Compressing {} with {:?}", analysis.path.display(), analysis.recommended_algorithm);
        
        match analysis.recommended_algorithm {
            CompressionAlgorithm::Gzip => {
                self.compress_with_gzip(&analysis.path, output_path).await
            }
            CompressionAlgorithm::Zip => {
                self.compress_with_zip(&analysis.path, output_path).await
            }
            _ => {
                // For other algorithms, would need additional dependencies
                Err(crate::error::DiskBlotError::NotImplemented(
                    format!("Compression algorithm {:?} not yet implemented", analysis.recommended_algorithm)
                ))
            }
        }
    }

    /// Compress with gzip
    async fn compress_with_gzip(&self, input: &Path, output: &Path) -> DiskBlotResult<u64> {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use std::io::Write;
        
        let input_data = tokio::fs::read(input).await?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&input_data)?;
        let compressed = encoder.finish()?;
        
        tokio::fs::write(output, &compressed).await?;
        Ok(compressed.len() as u64)
    }

    /// Compress with zip
    async fn compress_with_zip(&self, input: &Path, output: &Path) -> DiskBlotResult<u64> {
        // Would use zip crate in production
        // For now, return a placeholder
        Err(crate::error::DiskBlotError::NotImplemented(
            "ZIP compression not yet implemented".to_string()
        ))
    }

    /// Create compression recommendations for cleanup integration
    pub fn create_recommendations(&self, stats: &CompressionStats) -> Vec<CompressionRecommendation> {
        let mut recommendations = Vec::new();
        
        // Recommend compressing high-value targets
        if stats.total_potential_savings > 1024 * 1024 * 100 { // 100MB+
            recommendations.push(CompressionRecommendation {
                title: "Large compression opportunity detected".to_string(),
                description: format!(
                    "You can save {} by compressing {} files",
                    byte_unit::Byte::from_u128(stats.total_potential_savings as u128)
                        .expect("Invalid byte value")
                        .get_appropriate_unit(byte_unit::UnitType::Decimal),
                    stats.top_compressible_files.len()
                ),
                priority: RecommendationPriority::High,
                action: RecommendationAction::CompressFiles(
                    stats.top_compressible_files.iter().map(|a| a.path.clone()).collect()
                ),
            });
        }
        
        // Recommend archiving old logs
        let log_files: Vec<_> = stats.top_compressible_files.iter()
            .filter(|a| a.path.to_string_lossy().contains("log"))
            .collect();
        if !log_files.is_empty() {
            recommendations.push(CompressionRecommendation {
                title: "Archive old log files".to_string(),
                description: format!("Found {} uncompressed log files that could be archived", log_files.len()),
                priority: RecommendationPriority::Medium,
                action: RecommendationAction::ArchiveLogs,
            });
        }
        
        recommendations
    }
}

/// Compression recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionRecommendation {
    /// Recommendation title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Recommended action
    pub action: RecommendationAction,
}

/// Recommendation priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    High,
    Medium,
    Low,
}

/// Recommended actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationAction {
    /// Compress specific files
    CompressFiles(Vec<PathBuf>),
    /// Archive old logs
    ArchiveLogs,
    /// Enable automatic compression
    EnableAutoCompression,
}

impl Default for CompressionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_compressibility_detection() {
        let analyzer = CompressionAnalyzer::new();
        
        // Test various file types
        let test_cases = vec![
            ("test.txt", Compressibility::High),
            ("test.log", Compressibility::High),
            ("test.rs", Compressibility::High),
            ("test.exe", Compressibility::Medium),
            ("test.pdf", Compressibility::Medium),
            ("test.zip", Compressibility::AlreadyCompressed),
            ("test.jpg", Compressibility::AlreadyCompressed),
            ("test.dll", Compressibility::DoNotCompress),
        ];
        
        for (filename, expected) in test_cases {
            let path = Path::new(filename);
            let result = analyzer.determine_compressibility(path);
            assert_eq!(result, expected, "Failed for {}", filename);
        }
    }

    #[test]
    fn test_file_analysis() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = CompressionAnalyzer::new();
        
        // Create a test file
        let test_file = temp_dir.path().join("test.txt");
        let content = "This is a test file that should be compressible.\n".repeat(1000);
        fs::write(&test_file, content).unwrap();
        
        let analysis = analyzer.analyze_file(&test_file).unwrap();
        
        assert!(analysis.original_size > 0);
        assert!(analysis.estimated_compressed_size < analysis.original_size);
        assert_eq!(analysis.compressibility, Compressibility::High);
        assert!(analysis.should_compress);
    }

    #[tokio::test]
    async fn test_directory_analysis() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = CompressionAnalyzer::new();
        
        // Create various test files
        fs::write(temp_dir.path().join("text.txt"), "A".repeat(10000)).unwrap();
        fs::write(temp_dir.path().join("log.log"), "Log entry\n".repeat(1000)).unwrap();
        fs::write(temp_dir.path().join("already.zip"), vec![0; 1000]).unwrap();
        
        let stats = analyzer.analyze_directory(temp_dir.path()).await.unwrap();
        
        assert_eq!(stats.total_files, 3);
        assert!(stats.total_potential_savings > 0);
        assert!(stats.files_by_compressibility[&Compressibility::High] >= 2);
        assert!(stats.files_by_compressibility[&Compressibility::AlreadyCompressed] >= 1);
    }

    #[test]
    fn test_recommendations() {
        let analyzer = CompressionAnalyzer::new();
        
        let mut stats = CompressionStats {
            total_files: 100,
            total_size: 1024 * 1024 * 1024, // 1GB
            total_potential_savings: 1024 * 1024 * 200, // 200MB
            files_by_compressibility: HashMap::new(),
            size_by_compressibility: HashMap::new(),
            top_compressible_files: vec![
                CompressionAnalysis {
                    path: PathBuf::from("/test/large.log"),
                    original_size: 1024 * 1024 * 100,
                    estimated_compressed_size: 1024 * 1024 * 20,
                    potential_savings: 1024 * 1024 * 80,
                    compression_ratio: 0.2,
                    compressibility: Compressibility::High,
                    recommended_algorithm: CompressionAlgorithm::Gzip,
                    should_compress: true,
                    recommendation_reason: "Highly compressible text-based file".to_string(),
                }
            ],
        };
        
        let recommendations = analyzer.create_recommendations(&stats);
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().any(|r| matches!(r.priority, RecommendationPriority::High)));
    }
}