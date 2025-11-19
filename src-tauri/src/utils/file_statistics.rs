//! File type statistics and visualization support (BEAD-035)
//! 
//! Categorizes files by type, provides visual breakdown with charts,
//! shows size by category, and supports drill-down analysis.

use crate::error::DiskBlotResult;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use log::{info, debug};

/// File categories for statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FileCategory {
    /// Source code files
    SourceCode,
    /// Documents (PDF, Word, etc.)
    Documents,
    /// Images (JPG, PNG, etc.)
    Images,
    /// Videos
    Videos,
    /// Audio files
    Audio,
    /// Archives (ZIP, TAR, etc.)
    Archives,
    /// Executables and binaries
    Executables,
    /// Configuration files
    Configuration,
    /// Log files
    Logs,
    /// Database files
    Databases,
    /// Cache files
    Cache,
    /// Temporary files
    Temporary,
    /// System files
    System,
    /// Other/Unknown
    Other,
}

/// Detailed file type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeInfo {
    /// File extension
    pub extension: String,
    /// Category
    pub category: FileCategory,
    /// MIME type if known
    pub mime_type: Option<String>,
    /// Human-readable description
    pub description: String,
    /// Whether this is a binary format
    pub is_binary: bool,
    /// Whether files of this type are typically large
    pub typically_large: bool,
}

/// Statistics for a file category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    /// Category name
    pub category: FileCategory,
    /// Number of files
    pub file_count: usize,
    /// Total size in bytes
    pub total_size: u64,
    /// Average file size
    pub average_size: u64,
    /// Largest file
    pub largest_file: Option<FileInfo>,
    /// File extensions in this category
    pub extensions: HashMap<String, ExtensionStats>,
    /// Percentage of total size
    pub size_percentage: f64,
    /// Percentage of total files
    pub count_percentage: f64,
}

/// Statistics for a specific file extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionStats {
    /// Extension (without dot)
    pub extension: String,
    /// Number of files
    pub count: usize,
    /// Total size
    pub total_size: u64,
    /// Average size
    pub average_size: u64,
}

/// Individual file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// File path
    pub path: PathBuf,
    /// File size
    pub size: u64,
    /// File extension
    pub extension: String,
    /// Last modified time
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Overall file statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatistics {
    /// Total number of files analyzed
    pub total_files: usize,
    /// Total size in bytes
    pub total_size: u64,
    /// Statistics by category
    pub categories: Vec<CategoryStats>,
    /// Top 10 largest files
    pub largest_files: Vec<FileInfo>,
    /// Most common extensions
    pub common_extensions: Vec<ExtensionStats>,
    /// Directory analyzed
    pub root_path: PathBuf,
    /// Analysis timestamp
    pub analyzed_at: chrono::DateTime<chrono::Utc>,
    /// Chart data for visualization
    pub chart_data: ChartData,
}

/// Data formatted for chart visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    /// Pie chart data for size by category
    pub size_by_category: Vec<ChartSegment>,
    /// Bar chart data for count by category
    pub count_by_category: Vec<ChartSegment>,
    /// Top extensions by size
    pub top_extensions_by_size: Vec<ChartSegment>,
    /// Size distribution histogram
    pub size_distribution: Vec<SizeRange>,
}

/// Chart segment for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartSegment {
    /// Label
    pub label: String,
    /// Value
    pub value: f64,
    /// Color (hex)
    pub color: String,
    /// Additional data
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Size range for histogram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeRange {
    /// Range label (e.g., "0-1MB")
    pub label: String,
    /// Number of files in range
    pub count: usize,
    /// Total size in range
    pub total_size: u64,
}

/// File statistics analyzer
pub struct FileStatisticsAnalyzer {
    /// Extension to category mapping
    extension_map: HashMap<String, FileCategory>,
    /// Category colors for charts
    category_colors: HashMap<FileCategory, String>,
}

impl FileStatisticsAnalyzer {
    /// Create a new analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            extension_map: HashMap::new(),
            category_colors: HashMap::new(),
        };
        
        analyzer.initialize_mappings();
        analyzer
    }

    /// Initialize extension mappings and colors
    fn initialize_mappings(&mut self) {
        // Source code
        for ext in ["rs", "py", "js", "ts", "java", "cpp", "c", "h", "go", "rb", "php", "swift", "kt", "scala", "r", "m", "mm"] {
            self.extension_map.insert(ext.to_string(), FileCategory::SourceCode);
        }
        
        // Documents
        for ext in ["pdf", "doc", "docx", "odt", "txt", "rtf", "tex", "md", "rst"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Documents);
        }
        
        // Images
        for ext in ["jpg", "jpeg", "png", "gif", "webp", "bmp", "svg", "ico", "tiff", "raw", "psd", "ai"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Images);
        }
        
        // Videos
        for ext in ["mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Videos);
        }
        
        // Audio
        for ext in ["mp3", "wav", "flac", "aac", "ogg", "wma", "m4a", "opus", "aiff"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Audio);
        }
        
        // Archives
        for ext in ["zip", "tar", "gz", "7z", "rar", "bz2", "xz", "tgz", "deb", "rpm", "dmg", "pkg"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Archives);
        }
        
        // Executables
        for ext in ["exe", "app", "dll", "so", "dylib", "bat", "sh", "command"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Executables);
        }
        
        // Configuration
        for ext in ["json", "yaml", "yml", "toml", "ini", "conf", "config", "xml", "plist"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Configuration);
        }
        
        // Logs
        for ext in ["log", "out", "err"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Logs);
        }
        
        // Databases
        for ext in ["db", "sqlite", "mdb", "accdb", "dbf", "sql"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Databases);
        }
        
        // Cache
        for ext in ["cache", "tmp", "temp"] {
            self.extension_map.insert(ext.to_string(), FileCategory::Cache);
        }
        
        // Category colors
        self.category_colors.insert(FileCategory::SourceCode, "#3B82F6".to_string());      // Blue
        self.category_colors.insert(FileCategory::Documents, "#10B981".to_string());       // Green
        self.category_colors.insert(FileCategory::Images, "#F59E0B".to_string());          // Amber
        self.category_colors.insert(FileCategory::Videos, "#EF4444".to_string());          // Red
        self.category_colors.insert(FileCategory::Audio, "#8B5CF6".to_string());           // Purple
        self.category_colors.insert(FileCategory::Archives, "#6B7280".to_string());        // Gray
        self.category_colors.insert(FileCategory::Executables, "#059669".to_string());     // Emerald
        self.category_colors.insert(FileCategory::Configuration, "#7C3AED".to_string());   // Violet
        self.category_colors.insert(FileCategory::Logs, "#DC2626".to_string());            // Red-600
        self.category_colors.insert(FileCategory::Databases, "#0891B2".to_string());       // Cyan
        self.category_colors.insert(FileCategory::Cache, "#D97706".to_string());           // Amber-600
        self.category_colors.insert(FileCategory::Temporary, "#7C2D12".to_string());       // Orange-900
        self.category_colors.insert(FileCategory::System, "#1F2937".to_string());          // Gray-800
        self.category_colors.insert(FileCategory::Other, "#9CA3AF".to_string());           // Gray-400
    }

    /// Categorize a file based on its extension
    fn categorize_file(&self, path: &Path) -> FileCategory {
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        
        // Check extension mapping
        if let Some(category) = self.extension_map.get(&extension) {
            return category.clone();
        }
        
        // Check file name patterns
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        if file_name.contains("cache") || file_name.starts_with('.') {
            return FileCategory::Cache;
        }
        
        if file_name.contains("temp") || file_name.contains("tmp") {
            return FileCategory::Temporary;
        }
        
        if file_name.contains("log") && !file_name.contains("logo") {
            return FileCategory::Logs;
        }
        
        FileCategory::Other
    }

    /// Analyze a directory and generate statistics
    pub async fn analyze_directory(&self, root: &Path) -> DiskBlotResult<FileStatistics> {
        info!("Analyzing directory for file statistics: {}", root.display());
        
        let mut category_data: HashMap<FileCategory, CategoryData> = HashMap::new();
        let mut all_files = Vec::new();
        let mut extension_totals: HashMap<String, ExtensionStats> = HashMap::new();
        
        // Initialize categories
        for category in [
            FileCategory::SourceCode, FileCategory::Documents, FileCategory::Images,
            FileCategory::Videos, FileCategory::Audio, FileCategory::Archives,
            FileCategory::Executables, FileCategory::Configuration, FileCategory::Logs,
            FileCategory::Databases, FileCategory::Cache, FileCategory::Temporary,
            FileCategory::System, FileCategory::Other
        ] {
            category_data.insert(category, CategoryData::default());
        }
        
        // Walk directory and collect statistics
        for entry in walkdir::WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            
            let size = metadata.len();
            let category = self.categorize_file(path);
            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "no_ext".to_string());
            
            let modified = metadata.modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .and_then(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0));
            
            let file_info = FileInfo {
                path: path.to_path_buf(),
                size,
                extension: extension.clone(),
                modified,
            };
            
            // Update category data
            if let Some(cat_data) = category_data.get_mut(&category) {
                cat_data.files.push(file_info.clone());
                cat_data.total_size += size;
                
                *cat_data.extensions.entry(extension.clone()).or_insert(0) += size;
            }
            
            // Update extension totals
            let ext_stat = extension_totals.entry(extension.clone()).or_insert(ExtensionStats {
                extension: extension.clone(),
                count: 0,
                total_size: 0,
                average_size: 0,
            });
            ext_stat.count += 1;
            ext_stat.total_size += size;
            
            all_files.push(file_info);
        }
        
        // Calculate totals
        let total_files = all_files.len();
        let total_size: u64 = category_data.values().map(|c| c.total_size).sum();
        
        // Build category statistics
        let mut categories = Vec::new();
        for (category, data) in category_data {
            if data.files.is_empty() {
                continue;
            }
            
            let mut extensions = HashMap::new();
            for (ext, size) in data.extensions {
                if let Some(ext_total) = extension_totals.get(&ext) {
                    extensions.insert(ext.clone(), ExtensionStats {
                        extension: ext,
                        count: ext_total.count,
                        total_size: size,
                        average_size: if ext_total.count > 0 { size / ext_total.count as u64 } else { 0 },
                    });
                }
            }
            
            let largest_file = data.files.iter().max_by_key(|f| f.size).cloned();
            
            categories.push(CategoryStats {
                category: category.clone(),
                file_count: data.files.len(),
                total_size: data.total_size,
                average_size: if !data.files.is_empty() { data.total_size / data.files.len() as u64 } else { 0 },
                largest_file,
                extensions,
                size_percentage: if total_size > 0 { (data.total_size as f64 / total_size as f64) * 100.0 } else { 0.0 },
                count_percentage: if total_files > 0 { (data.files.len() as f64 / total_files as f64) * 100.0 } else { 0.0 },
            });
        }
        
        // Sort categories by size
        categories.sort_by_key(|c| std::cmp::Reverse(c.total_size));
        
        // Get largest files
        all_files.sort_by_key(|f| std::cmp::Reverse(f.size));
        let largest_files = all_files.into_iter().take(10).collect();
        
        // Get most common extensions
        let mut ext_vec: Vec<_> = extension_totals.into_iter().map(|(_, v)| v).collect();
        for ext in &mut ext_vec {
            ext.average_size = if ext.count > 0 { ext.total_size / ext.count as u64 } else { 0 };
        }
        ext_vec.sort_by_key(|e| std::cmp::Reverse(e.total_size));
        let common_extensions = ext_vec.into_iter().take(10).collect();
        
        // Create chart data
        let chart_data = self.create_chart_data(&categories, total_size);
        
        Ok(FileStatistics {
            total_files,
            total_size,
            categories,
            largest_files,
            common_extensions,
            root_path: root.to_path_buf(),
            analyzed_at: chrono::Utc::now(),
            chart_data,
        })
    }

    /// Create chart data for visualization
    fn create_chart_data(&self, categories: &[CategoryStats], total_size: u64) -> ChartData {
        // Size by category pie chart
        let size_by_category: Vec<ChartSegment> = categories.iter()
            .filter(|c| c.size_percentage > 1.0) // Only show categories > 1%
            .map(|c| {
                let mut metadata = HashMap::new();
                metadata.insert("file_count".to_string(), serde_json::Value::Number(c.file_count.into()));
                metadata.insert("average_size".to_string(), serde_json::Value::Number(c.average_size.into()));
                
                ChartSegment {
                    label: format!("{:?}", c.category),
                    value: c.size_percentage,
                    color: self.category_colors.get(&c.category).cloned().unwrap_or_else(|| "#9CA3AF".to_string()),
                    metadata,
                }
            })
            .collect();
        
        // Count by category bar chart
        let count_by_category: Vec<ChartSegment> = categories.iter()
            .map(|c| {
                let mut metadata = HashMap::new();
                metadata.insert("total_size".to_string(), serde_json::Value::Number(c.total_size.into()));
                
                ChartSegment {
                    label: format!("{:?}", c.category),
                    value: c.file_count as f64,
                    color: self.category_colors.get(&c.category).cloned().unwrap_or_else(|| "#9CA3AF".to_string()),
                    metadata,
                }
            })
            .collect();
        
        // Top extensions by size
        let top_extensions_by_size: Vec<ChartSegment> = categories.iter()
            .flat_map(|c| c.extensions.values())
            .filter(|e| e.total_size > 1024 * 1024) // Only show > 1MB
            .take(15)
            .map(|e| {
                let mut metadata = HashMap::new();
                metadata.insert("count".to_string(), serde_json::Value::Number(e.count.into()));
                
                ChartSegment {
                    label: format!(".{}", e.extension),
                    value: e.total_size as f64 / 1024.0 / 1024.0, // Convert to MB
                    color: "#60A5FA".to_string(), // Blue-400
                    metadata,
                }
            })
            .collect();
        
        // Size distribution histogram
        let size_ranges = vec![
            (0, 1024 * 10, "0-10KB"),
            (1024 * 10, 1024 * 100, "10KB-100KB"),
            (1024 * 100, 1024 * 1024, "100KB-1MB"),
            (1024 * 1024, 1024 * 1024 * 10, "1MB-10MB"),
            (1024 * 1024 * 10, 1024 * 1024 * 100, "10MB-100MB"),
            (1024 * 1024 * 100, 1024 * 1024 * 1024, "100MB-1GB"),
            (1024 * 1024 * 1024, u64::MAX, "1GB+"),
        ];
        
        let mut size_distribution = Vec::new();
        for (min, max, label) in size_ranges {
            let files_in_range: Vec<_> = categories.iter()
                .flat_map(|c| &c.largest_file)
                .filter(|f| f.size >= min && f.size < max)
                .collect();
            
            if !files_in_range.is_empty() {
                size_distribution.push(SizeRange {
                    label: label.to_string(),
                    count: files_in_range.len(),
                    total_size: files_in_range.iter().map(|f| f.size).sum(),
                });
            }
        }
        
        ChartData {
            size_by_category,
            count_by_category,
            top_extensions_by_size,
            size_distribution,
        }
    }

    /// Get detailed statistics for a specific category
    pub fn get_category_details<'a>(&self, stats: &'a FileStatistics, category: &FileCategory) -> Option<&'a CategoryStats> {
        stats.categories.iter().find(|c| c.category == *category)
    }

    /// Get color for a category
    pub fn get_category_color(&self, category: &FileCategory) -> String {
        self.category_colors.get(category).cloned().unwrap_or_else(|| "#9CA3AF".to_string())
    }
}

/// Internal data structure for collecting category data
#[derive(Default)]
struct CategoryData {
    files: Vec<FileInfo>,
    total_size: u64,
    extensions: HashMap<String, u64>,
}

impl Default for FileStatisticsAnalyzer {
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
    fn test_file_categorization() {
        let analyzer = FileStatisticsAnalyzer::new();
        
        let test_cases = vec![
            ("test.rs", FileCategory::SourceCode),
            ("document.pdf", FileCategory::Documents),
            ("image.jpg", FileCategory::Images),
            ("video.mp4", FileCategory::Videos),
            ("song.mp3", FileCategory::Audio),
            ("archive.zip", FileCategory::Archives),
            ("program.exe", FileCategory::Executables),
            ("config.json", FileCategory::Configuration),
            ("app.log", FileCategory::Logs),
            ("data.db", FileCategory::Databases),
            ("unknown.xyz", FileCategory::Other),
        ];
        
        for (filename, expected) in test_cases {
            let path = Path::new(filename);
            let category = analyzer.categorize_file(path);
            assert_eq!(category, expected, "Failed for {}", filename);
        }
    }

    #[tokio::test]
    async fn test_directory_analysis() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = FileStatisticsAnalyzer::new();
        
        // Create test files
        fs::write(temp_dir.path().join("code.rs"), "fn main() {}").unwrap();
        fs::write(temp_dir.path().join("doc.pdf"), vec![0; 1000]).unwrap();
        fs::write(temp_dir.path().join("image.png"), vec![0; 5000]).unwrap();
        fs::write(temp_dir.path().join("data.json"), "{}").unwrap();
        
        let stats = analyzer.analyze_directory(temp_dir.path()).await.unwrap();
        
        assert_eq!(stats.total_files, 4);
        assert!(stats.total_size > 0);
        assert!(!stats.categories.is_empty());
        
        // Check that we have different categories
        let category_types: Vec<_> = stats.categories.iter().map(|c| &c.category).collect();
        assert!(category_types.contains(&&FileCategory::SourceCode));
        assert!(category_types.contains(&&FileCategory::Images));
    }

    #[test]
    fn test_chart_data_generation() {
        let analyzer = FileStatisticsAnalyzer::new();
        
        let categories = vec![
            CategoryStats {
                category: FileCategory::SourceCode,
                file_count: 100,
                total_size: 1024 * 1024 * 50, // 50MB
                average_size: 1024 * 512,
                largest_file: None,
                extensions: HashMap::new(),
                size_percentage: 50.0,
                count_percentage: 40.0,
            },
            CategoryStats {
                category: FileCategory::Images,
                file_count: 50,
                total_size: 1024 * 1024 * 30, // 30MB
                average_size: 1024 * 600,
                largest_file: None,
                extensions: HashMap::new(),
                size_percentage: 30.0,
                count_percentage: 20.0,
            },
        ];
        
        let chart_data = analyzer.create_chart_data(&categories, 1024 * 1024 * 100);
        
        assert!(!chart_data.size_by_category.is_empty());
        assert!(!chart_data.count_by_category.is_empty());
        assert_eq!(chart_data.size_by_category[0].value, 50.0);
    }
}