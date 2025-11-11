// Project Auditor & Compliance Scanner (PACS) Module
// Implementation of deep project analysis and compliance monitoring
#![allow(
    clippy::use_self,
    clippy::uninlined_format_args,
    clippy::redundant_closure_for_method_calls,
    clippy::unused_self,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::doc_markdown,
    clippy::for_kv_map,
    clippy::manual_clamp,
    clippy::format_push_string
)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
// Note: Database integration will be added when needed

/// Deep Project Scanner - Core PACS component for single project analysis
pub struct DeepProjectScanner {
    project_path: PathBuf,
    config: PACSConfig,
    findings: Vec<ComplianceFinding>,
    baseline: Option<ProjectBaseline>,
}

/// PACS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PACSConfig {
    /// Enable automatic spec generation
    pub auto_generate_specs: bool,
    /// Enable automatic Beads issue creation
    pub auto_create_beads: bool,
    /// Compliance standards to validate against
    pub standards: Vec<ComplianceStandard>,
    /// Output directory for generated artifacts
    pub output_dir: String,
    /// Maximum scan depth
    pub max_depth: usize,
}

/// Compliance standards to validate against
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceStandard {
    TES2025,
    EDGS,
    LAIO,
    BloomPlaybook,
    OpenSpec,
}

impl fmt::Display for ComplianceStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplianceStandard::TES2025 => write!(f, "TES-2025"),
            ComplianceStandard::EDGS => write!(f, "EDGS"),
            ComplianceStandard::LAIO => write!(f, "LAIO"),
            ComplianceStandard::BloomPlaybook => write!(f, "Bloom Playbook"),
            ComplianceStandard::OpenSpec => write!(f, "OpenSpec"),
        }
    }
}

/// Project compliance finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFinding {
    pub id: String,
    pub severity: FindingSeverity,
    pub category: FindingCategory,
    pub title: String,
    pub description: String,
    pub file_path: Option<String>,
    pub line_number: Option<usize>,
    pub recommendation: String,
    pub auto_fixable: bool,
    pub standard: ComplianceStandard,
    pub detected_at: DateTime<Utc>,
}

/// Finding severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Finding categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingCategory {
    MissingDocumentation,
    InvalidFormat,
    ComplianceViolation,
    SecurityIssue,
    ArchitecturalDrift,
    SpecificationGap,
    VersioningIssue,
    MetadataIncomplete,
}

/// Project baseline for drift detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBaseline {
    pub project_path: String,
    pub captured_at: DateTime<Utc>,
    pub version: String,
    pub file_inventory: HashMap<String, FileMetadata>,
    pub compliance_score: f64,
    pub architecture_hash: String,
    pub dependencies: Vec<String>,
    pub standards_compliance: HashMap<ComplianceStandard, bool>,
}

/// File metadata for baseline tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub hash: String,
    pub file_type: FileType,
    pub compliance_relevant: bool,
}

/// File types for classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    Documentation,
    Specification,
    Configuration,
    Source,
    Test,
    Build,
    Other,
}

/// Project audit report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAuditReport {
    pub project_path: String,
    pub scanned_at: DateTime<Utc>,
    pub scanner_version: String,
    pub compliance_score: f64,
    pub findings: Vec<ComplianceFinding>,
    pub baseline: Option<ProjectBaseline>,
    pub recommendations: Vec<String>,
    pub auto_fixes_available: usize,
    pub standards_summary: HashMap<ComplianceStandard, ComplianceStatus>,
}

/// Compliance status for each standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub compliant: bool,
    pub score: f64,
    pub issues_count: usize,
    pub critical_issues: usize,
}

impl DeepProjectScanner {
    /// Create new scanner for project
    pub fn new(project_path: impl AsRef<Path>, config: PACSConfig) -> Self {
        Self {
            project_path: project_path.as_ref().to_path_buf(),
            config,
            findings: Vec::new(),
            baseline: None,
        }
    }

    /// Load existing baseline if available
    pub fn load_baseline(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let baseline_path = self.project_path.join(".pacs").join("baseline.json");

        if baseline_path.exists() {
            let content = std::fs::read_to_string(&baseline_path)?;
            self.baseline = Some(serde_json::from_str(&content)?);
            log::info!("Loaded existing baseline from {}", baseline_path.display());
        } else {
            log::info!("No existing baseline found, will create new one");
        }

        Ok(())
    }

    /// Perform deep project scan
    pub async fn scan(&mut self) -> Result<ProjectAuditReport, Box<dyn std::error::Error>> {
        log::info!(
            "Starting deep project scan for: {}",
            self.project_path.display()
        );

        // Clear previous findings
        self.findings.clear();

        // Step 1: Inventory all files
        let file_inventory = self.inventory_files().await?;
        log::info!("Inventoried {} files", file_inventory.len());

        // Step 2: Analyze documentation structure
        self.analyze_documentation_structure(&file_inventory)
            .await?;

        // Step 3: Validate against compliance standards
        self.validate_compliance_standards(&file_inventory).await?;

        // Step 4: Check for specification gaps
        self.check_specification_gaps(&file_inventory).await?;

        // Step 5: Detect architectural drift if baseline exists
        if let Some(baseline) = self.baseline.clone() {
            self.detect_drift(&baseline, &file_inventory).await?;
        }

        // Step 6: Generate recommendations
        let recommendations = self.generate_recommendations();

        // Step 7: Calculate compliance score
        let compliance_score = self.calculate_compliance_score();

        // Step 8: Create new baseline
        let new_baseline = self.create_baseline(&file_inventory).await?;

        // Step 9: Generate standards summary
        let standards_summary = self.generate_standards_summary();

        let report = ProjectAuditReport {
            project_path: self.project_path.to_string_lossy().to_string(),
            scanned_at: Utc::now(),
            scanner_version: "1.0.0".to_string(),
            compliance_score,
            findings: self.findings.clone(),
            baseline: Some(new_baseline),
            recommendations,
            auto_fixes_available: self.findings.iter().filter(|f| f.auto_fixable).count(),
            standards_summary,
        };

        log::info!(
            "Deep project scan completed. Score: {:.1}/100",
            compliance_score
        );

        Ok(report)
    }

    /// Inventory all files in project
    async fn inventory_files(
        &self,
    ) -> Result<HashMap<String, FileMetadata>, Box<dyn std::error::Error>> {
        use std::fs;
        use walkdir::WalkDir;

        let mut inventory = HashMap::new();

        for entry in WalkDir::new(&self.project_path)
            .max_depth(self.config.max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                let relative_path = path
                    .strip_prefix(&self.project_path)?
                    .to_string_lossy()
                    .to_string();

                // Skip hidden files and common ignore patterns
                if self.should_skip_file(&relative_path) {
                    continue;
                }

                let metadata = fs::metadata(path)?;
                let content = fs::read(path).unwrap_or_default();
                let hash = format!("{:x}", md5::compute(&content));

                let file_metadata = FileMetadata {
                    path: relative_path.clone(),
                    size: metadata.len(),
                    modified: DateTime::from(metadata.modified()?),
                    hash,
                    file_type: self.classify_file_type(&relative_path),
                    compliance_relevant: self.is_compliance_relevant(&relative_path),
                };

                inventory.insert(relative_path, file_metadata);
            }
        }

        Ok(inventory)
    }

    /// Check if file should be skipped
    fn should_skip_file(&self, path: &str) -> bool {
        let skip_patterns = [
            ".git/",
            "node_modules/",
            "target/",
            ".DS_Store",
            "*.log",
            "*.tmp",
            ".env",
            "*.lock",
        ];

        skip_patterns.iter().any(|pattern| {
            if pattern.contains('*') {
                path.ends_with(&pattern.replace('*', ""))
            } else {
                path.contains(pattern)
            }
        })
    }

    /// Classify file type based on path and extension
    fn classify_file_type(&self, path: &str) -> FileType {
        let path_lower = path.to_lowercase();

        if path_lower.starts_with("docs/") || path_lower.ends_with(".md") {
            FileType::Documentation
        } else if path_lower.contains("openspec/") || path_lower.contains("spec") {
            FileType::Specification
        } else if path_lower.ends_with(".toml")
            || path_lower.ends_with(".json")
            || path_lower.ends_with(".yaml")
        {
            FileType::Configuration
        } else if path_lower.ends_with(".rs")
            || path_lower.ends_with(".js")
            || path_lower.ends_with(".ts")
        {
            FileType::Source
        } else if path_lower.contains("test") || path_lower.ends_with("_test.rs") {
            FileType::Test
        } else if path_lower.contains("build") || path_lower.contains("target/") {
            FileType::Build
        } else {
            FileType::Other
        }
    }

    /// Check if file is relevant for compliance
    fn is_compliance_relevant(&self, path: &str) -> bool {
        let relevant_patterns = [
            "docs/",
            "openspec/",
            "AGENTS.md",
            "README.md",
            ".beads/",
            "Cargo.toml",
            "package.json",
        ];

        relevant_patterns
            .iter()
            .any(|pattern| path.contains(pattern))
    }

    /// Analyze documentation structure
    async fn analyze_documentation_structure(
        &mut self,
        inventory: &HashMap<String, FileMetadata>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Analyzing documentation structure...");

        // Check for required documentation directories
        let required_dirs = ["docs/", "docs/design/", "docs/analysis/"];

        for dir in required_dirs {
            let has_files = inventory.keys().any(|path| path.starts_with(dir));

            if !has_files {
                self.findings.push(ComplianceFinding {
                    id: format!("missing-{}", dir.replace('/', "-")),
                    severity: FindingSeverity::High,
                    category: FindingCategory::MissingDocumentation,
                    title: format!("Missing {} directory", dir),
                    description: format!(
                        "Required documentation directory {} is missing or empty",
                        dir
                    ),
                    file_path: None,
                    line_number: None,
                    recommendation: format!(
                        "Create {} directory with appropriate documentation",
                        dir
                    ),
                    auto_fixable: true,
                    standard: ComplianceStandard::TES2025,
                    detected_at: Utc::now(),
                });
            }
        }

        // Check for AGENTS.md
        if !inventory.contains_key("AGENTS.md") {
            self.findings.push(ComplianceFinding {
                id: "missing-agents-md".to_string(),
                severity: FindingSeverity::Medium,
                category: FindingCategory::MissingDocumentation,
                title: "Missing AGENTS.md".to_string(),
                description: "Project lacks agent coordination documentation".to_string(),
                file_path: None,
                line_number: None,
                recommendation: "Create AGENTS.md with agent coordination methodology".to_string(),
                auto_fixable: true,
                standard: ComplianceStandard::EDGS,
                detected_at: Utc::now(),
            });
        }

        Ok(())
    }

    /// Validate against compliance standards
    async fn validate_compliance_standards(
        &mut self,
        inventory: &HashMap<String, FileMetadata>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Validating compliance standards...");

        let standards = self.config.standards.clone();
        for standard in standards {
            match standard {
                ComplianceStandard::TES2025 => self.validate_tes2025(inventory).await?,
                ComplianceStandard::EDGS => self.validate_edgs(inventory).await?,
                ComplianceStandard::LAIO => self.validate_laio(inventory).await?,
                ComplianceStandard::OpenSpec => self.validate_openspec(inventory).await?,
                ComplianceStandard::BloomPlaybook => self.validate_bloom(inventory).await?,
            }
        }

        Ok(())
    }

    /// Validate TES-2025 compliance
    async fn validate_tes2025(
        &mut self,
        inventory: &HashMap<String, FileMetadata>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check for required TES-2025 documentation
        let required_files = [
            "docs/design/TDS-*.md",
            "docs/schedules/EDGS_SCHEDULE.md",
            "docs/analysis/ANALYSIS.md",
        ];

        for pattern in required_files {
            let found = if pattern.contains('*') {
                let prefix = pattern.split('*').next().unwrap_or("");
                inventory.keys().any(|path| path.starts_with(prefix))
            } else {
                inventory.contains_key(pattern)
            };

            if !found {
                self.findings.push(ComplianceFinding {
                    id: format!("tes2025-missing-{}", pattern.replace(['/', '*'], "-")),
                    severity: FindingSeverity::High,
                    category: FindingCategory::ComplianceViolation,
                    title: format!("Missing TES-2025 required file: {}", pattern),
                    description: format!(
                        "TES-2025 standard requires {} but it was not found",
                        pattern
                    ),
                    file_path: None,
                    line_number: None,
                    recommendation: format!(
                        "Create {} according to TES-2025 specification",
                        pattern
                    ),
                    auto_fixable: false,
                    standard: ComplianceStandard::TES2025,
                    detected_at: Utc::now(),
                });
            }
        }

        Ok(())
    }

    /// Validate EDGS compliance
    async fn validate_edgs(
        &mut self,
        inventory: &HashMap<String, FileMetadata>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check for EDGS schedule
        if !inventory.contains_key("docs/schedules/EDGS_SCHEDULE.md") {
            self.findings.push(ComplianceFinding {
                id: "edgs-missing-schedule".to_string(),
                severity: FindingSeverity::Medium,
                category: FindingCategory::ComplianceViolation,
                title: "Missing EDGS Schedule".to_string(),
                description: "Project lacks Event-Driven Gate Scheduling documentation".to_string(),
                file_path: None,
                line_number: None,
                recommendation: "Create EDGS_SCHEDULE.md with event-driven task scheduling"
                    .to_string(),
                auto_fixable: true,
                standard: ComplianceStandard::EDGS,
                detected_at: Utc::now(),
            });
        }

        Ok(())
    }

    /// Validate LAIO compliance
    async fn validate_laio(
        &mut self,
        _inventory: &HashMap<String, FileMetadata>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // LAIO validation logic would go here
        Ok(())
    }

    /// Validate OpenSpec compliance
    async fn validate_openspec(
        &mut self,
        inventory: &HashMap<String, FileMetadata>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let has_openspec = inventory.keys().any(|path| path.starts_with("openspec/"));

        if !has_openspec {
            self.findings.push(ComplianceFinding {
                id: "openspec-missing".to_string(),
                severity: FindingSeverity::Low,
                category: FindingCategory::SpecificationGap,
                title: "No OpenSpec documentation found".to_string(),
                description: "Project would benefit from OpenSpec change management".to_string(),
                file_path: None,
                line_number: None,
                recommendation: "Consider adding OpenSpec documentation for change management"
                    .to_string(),
                auto_fixable: false,
                standard: ComplianceStandard::OpenSpec,
                detected_at: Utc::now(),
            });
        }

        Ok(())
    }

    /// Validate Bloom Playbook compliance
    async fn validate_bloom(
        &mut self,
        _inventory: &HashMap<String, FileMetadata>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Bloom Playbook validation logic would go here
        Ok(())
    }

    /// Check for specification gaps
    async fn check_specification_gaps(
        &mut self,
        inventory: &HashMap<String, FileMetadata>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Checking for specification gaps...");

        // Check if project has any design documentation
        let has_design_docs = inventory.keys().any(|path| {
            path.starts_with("docs/design/") || path.contains("design") || path.contains("spec")
        });

        if !has_design_docs {
            self.findings.push(ComplianceFinding {
                id: "spec-gap-design".to_string(),
                severity: FindingSeverity::High,
                category: FindingCategory::SpecificationGap,
                title: "No design documentation found".to_string(),
                description: "Project lacks design specifications or architecture documentation"
                    .to_string(),
                file_path: None,
                line_number: None,
                recommendation: "Create design documentation in docs/design/ directory".to_string(),
                auto_fixable: false,
                standard: ComplianceStandard::TES2025,
                detected_at: Utc::now(),
            });
        }

        Ok(())
    }

    /// Detect architectural drift from baseline
    async fn detect_drift(
        &mut self,
        baseline: &ProjectBaseline,
        current_inventory: &HashMap<String, FileMetadata>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Detecting architectural drift from baseline...");

        // Check for removed files
        for (baseline_path, _) in &baseline.file_inventory {
            if !current_inventory.contains_key(baseline_path) {
                self.findings.push(ComplianceFinding {
                    id: format!("drift-removed-{}", baseline_path.replace(['/', '.'], "-")),
                    severity: FindingSeverity::Medium,
                    category: FindingCategory::ArchitecturalDrift,
                    title: format!("File removed: {}", baseline_path),
                    description: format!(
                        "File {} was present in baseline but is now missing",
                        baseline_path
                    ),
                    file_path: Some(baseline_path.clone()),
                    line_number: None,
                    recommendation: "Verify if file removal was intentional".to_string(),
                    auto_fixable: false,
                    standard: ComplianceStandard::TES2025,
                    detected_at: Utc::now(),
                });
            }
        }

        // Check for modified files
        for (current_path, current_meta) in current_inventory {
            if let Some(baseline_meta) = baseline.file_inventory.get(current_path) {
                if baseline_meta.hash != current_meta.hash && current_meta.compliance_relevant {
                    self.findings.push(ComplianceFinding {
                        id: format!("drift-modified-{}", current_path.replace(['/', '.'], "-")),
                        severity: FindingSeverity::Low,
                        category: FindingCategory::ArchitecturalDrift,
                        title: format!("Compliance-relevant file modified: {}", current_path),
                        description: format!(
                            "File {} has been modified since baseline",
                            current_path
                        ),
                        file_path: Some(current_path.clone()),
                        line_number: None,
                        recommendation: "Review changes for compliance impact".to_string(),
                        auto_fixable: false,
                        standard: ComplianceStandard::TES2025,
                        detected_at: Utc::now(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Generate recommendations based on findings
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        let critical_count = self
            .findings
            .iter()
            .filter(|f| matches!(f.severity, FindingSeverity::Critical))
            .count();
        let high_count = self
            .findings
            .iter()
            .filter(|f| matches!(f.severity, FindingSeverity::High))
            .count();
        let auto_fixable = self.findings.iter().filter(|f| f.auto_fixable).count();

        if critical_count > 0 {
            recommendations.push(format!(
                "Address {} critical compliance issues immediately",
                critical_count
            ));
        }

        if high_count > 0 {
            recommendations.push(format!(
                "Resolve {} high-priority compliance issues",
                high_count
            ));
        }

        if auto_fixable > 0 {
            recommendations.push(format!("Apply {} automatic fixes available", auto_fixable));
        }

        if self.findings.is_empty() {
            recommendations
                .push("Project is fully compliant with all configured standards".to_string());
        }

        recommendations
    }

    /// Calculate overall compliance score
    fn calculate_compliance_score(&self) -> f64 {
        if self.findings.is_empty() {
            return 100.0;
        }

        let total_weight: f64 = self
            .findings
            .iter()
            .map(|f| match f.severity {
                FindingSeverity::Critical => 20.0,
                FindingSeverity::High => 10.0,
                FindingSeverity::Medium => 5.0,
                FindingSeverity::Low => 2.0,
                FindingSeverity::Info => 1.0,
            })
            .sum();

        // Base score of 100, subtract weighted penalties
        let score = 100.0 - total_weight;
        score.max(0.0).min(100.0)
    }

    /// Create new baseline from current state
    async fn create_baseline(
        &self,
        inventory: &HashMap<String, FileMetadata>,
    ) -> Result<ProjectBaseline, Box<dyn std::error::Error>> {
        let compliance_score = self.calculate_compliance_score();

        // Create architecture hash from compliance-relevant files
        let mut arch_content = String::new();
        for (path, meta) in inventory {
            if meta.compliance_relevant {
                arch_content.push_str(&format!("{}:{}", path, meta.hash));
            }
        }
        let architecture_hash = format!("{:x}", md5::compute(arch_content.as_bytes()));

        // Extract dependencies (simplified)
        let dependencies = vec![
            "rust".to_string(),
            "tauri".to_string(),
            "svelte".to_string(),
        ];

        // Standards compliance summary
        let mut standards_compliance = HashMap::new();
        for standard in &self.config.standards {
            let compliant = !self.findings.iter().any(|f| {
                f.standard == *standard
                    && matches!(
                        f.severity,
                        FindingSeverity::Critical | FindingSeverity::High
                    )
            });
            standards_compliance.insert(standard.clone(), compliant);
        }

        Ok(ProjectBaseline {
            project_path: self.project_path.to_string_lossy().to_string(),
            captured_at: Utc::now(),
            version: "1.0.0".to_string(),
            file_inventory: inventory.clone(),
            compliance_score,
            architecture_hash,
            dependencies,
            standards_compliance,
        })
    }

    /// Generate standards compliance summary
    fn generate_standards_summary(&self) -> HashMap<ComplianceStandard, ComplianceStatus> {
        let mut summary = HashMap::new();

        for standard in &self.config.standards {
            let standard_findings: Vec<_> = self
                .findings
                .iter()
                .filter(|f| f.standard == *standard)
                .collect();

            let issues_count = standard_findings.len();
            let critical_issues = standard_findings
                .iter()
                .filter(|f| matches!(f.severity, FindingSeverity::Critical))
                .count();

            let compliant = critical_issues == 0 && issues_count < 3;

            let score = if issues_count == 0 {
                100.0
            } else {
                let penalty: f64 = standard_findings
                    .iter()
                    .map(|f| match f.severity {
                        FindingSeverity::Critical => 25.0,
                        FindingSeverity::High => 15.0,
                        FindingSeverity::Medium => 8.0,
                        FindingSeverity::Low => 3.0,
                        FindingSeverity::Info => 1.0,
                    })
                    .sum();
                (100.0 - penalty).max(0.0)
            };

            summary.insert(
                standard.clone(),
                ComplianceStatus {
                    compliant,
                    score,
                    issues_count,
                    critical_issues,
                },
            );
        }

        summary
    }

    /// Save audit report to file
    pub async fn save_report(
        &self,
        report: &ProjectAuditReport,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = Path::new(&self.config.output_dir);
        std::fs::create_dir_all(output_dir)?;

        // Save JSON report
        let json_path = output_dir.join("project-compliance-report.json");
        let json_content = serde_json::to_string_pretty(report)?;
        std::fs::write(&json_path, json_content)?;

        // Save baseline
        if let Some(baseline) = &report.baseline {
            let baseline_dir = self.project_path.join(".pacs");
            std::fs::create_dir_all(&baseline_dir)?;

            let baseline_path = baseline_dir.join("baseline.json");
            let baseline_content = serde_json::to_string_pretty(baseline)?;
            std::fs::write(&baseline_path, baseline_content)?;
        }

        log::info!("Saved audit report to: {}", json_path.display());

        Ok(())
    }
}

impl Default for PACSConfig {
    fn default() -> Self {
        Self {
            auto_generate_specs: true,
            auto_create_beads: true,
            standards: vec![
                ComplianceStandard::TES2025,
                ComplianceStandard::EDGS,
                ComplianceStandard::OpenSpec,
            ],
            output_dir: ".pacs/reports".to_string(),
            max_depth: 10,
        }
    }
}
