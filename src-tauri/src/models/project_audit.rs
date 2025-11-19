/// Project audit data structures for PACS (Project Auditor & Compliance Scanner)
///
/// This module defines the core data structures used throughout the PACS system
/// for representing project audits, compliance status, specifications, and violations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt;
use chrono::{DateTime, Utc};

/// Compliance level classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceLevel {
    /// Critical compliance violations requiring immediate attention
    Critical,
    /// High severity violations that should be addressed soon
    High,
    /// Medium severity violations that should be addressed in next cycle
    Medium,
    /// Low severity violations or advisory items
    Low,
}

impl ComplianceLevel {
    /// Get numeric score for compliance level (lower is worse)
    pub fn score(&self) -> f32 {
        match self {
            ComplianceLevel::Critical => 0.0,
            ComplianceLevel::High => 25.0,
            ComplianceLevel::Medium => 50.0,
            ComplianceLevel::Low => 75.0,
        }
    }

    /// Get color code for UI display
    pub fn color(&self) -> &'static str {
        match self {
            ComplianceLevel::Critical => "red",
            ComplianceLevel::High => "orange",
            ComplianceLevel::Medium => "yellow",
            ComplianceLevel::Low => "green",
        }
    }

    /// Get description text
    pub fn description(&self) -> &'static str {
        match self {
            ComplianceLevel::Critical => "Critical violations requiring immediate attention",
            ComplianceLevel::High => "High severity violations that should be addressed soon",
            ComplianceLevel::Medium => "Medium severity violations for next cycle",
            ComplianceLevel::Low => "Low severity violations or advisory items",
        }
    }
}

impl fmt::Display for ComplianceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplianceLevel::Critical => write!(f, "Critical"),
            ComplianceLevel::High => write!(f, "High"),
            ComplianceLevel::Medium => write!(f, "Medium"),
            ComplianceLevel::Low => write!(f, "Low"),
        }
    }
}

/// EDGS phase information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgsPhase {
    /// Current phase number (0-3)
    pub phase: u8,
    /// Phase gates completed
    pub gates_completed: Vec<String>,
    /// Last gate approval date
    pub last_approval: Option<DateTime<Utc>>,
    /// Phase regression detected
    pub regression_detected: bool,
}

impl EdgsPhase {
    /// Create new EDGS phase
    pub fn new(phase: u8) -> Self {
        Self {
            phase,
            gates_completed: Vec::new(),
            last_approval: None,
            regression_detected: false,
        }
    }

    /// Check if phase is valid (0-3)
    pub fn is_valid(&self) -> bool {
        self.phase <= 3
    }

    /// Get phase name
    pub fn phase_name(&self) -> &'static str {
        match self.phase {
            0 => "Foundation",
            1 => "Core Implementation",
            2 => "Monitoring & Validation",
            3 => "Production Ready",
            _ => "Unknown",
        }
    }
}

/// LAIO classification information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaioClassification {
    /// Domain number (1-10)
    pub domain: u8,
    /// Maturity level (L0-L4)
    pub maturity: String,
    /// Constitution file path
    pub constitution_path: Option<PathBuf>,
    /// Last classification update
    pub last_updated: Option<DateTime<Utc>>,
}

impl LaioClassification {
    /// Create new LAIO classification
    pub fn new(domain: u8, maturity: String) -> Self {
        Self {
            domain,
            maturity,
            constitution_path: None,
            last_updated: Some(Utc::now()),
        }
    }

    /// Check if domain is valid (1-10)
    pub fn is_valid_domain(&self) -> bool {
        self.domain >= 1 && self.domain <= 10
    }

    /// Check if maturity level is valid
    pub fn is_valid_maturity(&self) -> bool {
        matches!(self.maturity.as_str(), "L0" | "L1" | "L2" | "L3" | "L4")
    }

    /// Get domain name
    pub fn domain_name(&self) -> &'static str {
        match self.domain {
            1 => "Domain 1: Infrastructure & Operations",
            2 => "Domain 2: Data Management & Analytics",
            3 => "Domain 3: Application Development",
            4 => "Domain 4: Security & Compliance",
            5 => "Domain 5: Project Management & Governance",
            6 => "Domain 6: User Experience & Interface",
            7 => "Domain 7: Integration & APIs",
            8 => "Domain 8: Testing & Quality Assurance",
            9 => "Domain 9: Documentation & Knowledge",
            10 => "Domain 10: Innovation & Research",
            _ => "Unknown Domain",
        }
    }
}

/// Overall compliance status for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    /// Overall compliance score (0-100)
    pub score: f32,
    /// Compliance level
    pub level: ComplianceLevel,
    /// EDGS phase information
    pub edgs_phase: EdgsPhase,
    /// LAIO classification
    pub laio_classification: LaioClassification,
    /// TES-2025 compliance status
    pub tes_compliant: bool,
    /// Bloom phase coverage
    pub bloom_phases_covered: Vec<String>,
    /// Last audit timestamp
    pub last_audit: Option<DateTime<Utc>>,
    /// Total violations count
    pub total_violations: usize,
    /// Violations by severity
    pub violations_by_severity: HashMap<String, usize>,
}

impl ComplianceStatus {
    /// Create new compliance status
    pub fn new() -> Self {
        Self {
            score: 0.0,
            level: ComplianceLevel::Critical,
            edgs_phase: EdgsPhase::new(0),
            laio_classification: LaioClassification::new(1, "L0".to_string()),
            tes_compliant: false,
            bloom_phases_covered: Vec::new(),
            last_audit: Some(Utc::now()),
            total_violations: 0,
            violations_by_severity: HashMap::new(),
        }
    }

    /// Calculate compliance score from violations
    pub fn calculate_score(&mut self, violations: &[ComplianceViolation]) {
        self.total_violations = violations.len();
        
        // Count violations by severity
        self.violations_by_severity.clear();
        for violation in violations {
            let count = self.violations_by_severity
                .entry(violation.severity.to_string())
                .or_insert(0);
            *count += 1;
        }

        // Calculate score based on violation severity
        let mut score: f32 = 100.0;
        for violation in violations {
            score -= match violation.severity {
                ComplianceLevel::Critical => 25.0,
                ComplianceLevel::High => 15.0,
                ComplianceLevel::Medium => 8.0,
                ComplianceLevel::Low => 3.0,
            };
        }
        
        self.score = score.max(0.0);
        
        // Determine compliance level
        self.level = if self.score >= 80.0 {
            ComplianceLevel::Low
        } else if self.score >= 60.0 {
            ComplianceLevel::Medium
        } else if self.score >= 40.0 {
            ComplianceLevel::High
        } else {
            ComplianceLevel::Critical
        };
    }

    /// Check if project is compliant
    pub fn is_compliant(&self) -> bool {
        self.score >= 70.0 && self.tes_compliant && !self.edgs_phase.regression_detected
    }
}

/// Specification format types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpecFormat {
    /// OpenSpec canonical format
    OpenSpec,
    /// Traditional Markdown with frontmatter
    Markdown,
    /// YAML specification
    Yaml,
    /// JSON specification
    Json,
    /// TOML specification
    Toml,
    /// Unknown or custom format
    Unknown,
}

/// Specification type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpecType {
    /// Design specification
    Design,
    /// API specification
    Api,
    /// Architecture specification
    Architecture,
    /// Deployment specification
    Deployment,
    /// Testing specification
    Testing,
    /// Compliance specification
    Compliance,
    /// Schedule specification
    Schedule,
    /// User documentation
    Documentation,
    /// Unknown type
    Unknown,
}

/// Specification metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecMetadata {
    /// File path
    pub path: PathBuf,
    /// Specification format
    pub format: SpecFormat,
    /// Specification type
    pub spec_type: SpecType,
    /// Title from frontmatter or filename
    pub title: String,
    /// Version if specified
    pub version: Option<String>,
    /// Status if specified
    pub status: Option<String>,
    /// Author if specified
    pub author: Option<String>,
    /// Creation/modification dates
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    /// File size in bytes
    pub size_bytes: u64,
    /// SHA-256 hash for integrity
    pub hash: Option<String>,
    /// Tags from frontmatter
    pub tags: Vec<String>,
    /// Dependencies on other specs
    pub dependencies: Vec<String>,
}

impl SpecMetadata {
    /// Create new specification metadata
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            format: SpecFormat::Unknown,
            spec_type: SpecType::Unknown,
            title: String::new(),
            version: None,
            status: None,
            author: None,
            created: None,
            modified: None,
            size_bytes: 0,
            hash: None,
            tags: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    /// Check if specification is valid
    pub fn is_valid(&self) -> bool {
        !self.title.is_empty() && self.size_bytes > 0
    }

    /// Get file extension
    pub fn extension(&self) -> Option<String> {
        self.path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }
}

/// Specification inventory for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecInventory {
    /// Total number of specifications found
    pub total_specs: usize,
    /// Specifications by format
    pub by_format: HashMap<String, usize>,
    /// Specifications by type
    pub by_type: HashMap<String, usize>,
    /// List of all specifications
    pub specifications: Vec<SpecMetadata>,
    /// Missing required specifications
    pub missing_specs: Vec<String>,
    /// Outdated specifications
    pub outdated_specs: Vec<String>,
    /// Duplicate specifications
    pub duplicate_specs: Vec<String>,
}

impl SpecInventory {
    /// Create new specification inventory
    pub fn new() -> Self {
        Self {
            total_specs: 0,
            by_format: HashMap::new(),
            by_type: HashMap::new(),
            specifications: Vec::new(),
            missing_specs: Vec::new(),
            outdated_specs: Vec::new(),
            duplicate_specs: Vec::new(),
        }
    }

    /// Add specification to inventory
    pub fn add_spec(&mut self, spec: SpecMetadata) {
        // Update format counts
        let format_count = self.by_format
            .entry(format!("{:?}", spec.format))
            .or_insert(0);
        *format_count += 1;

        // Update type counts
        let type_count = self.by_type
            .entry(format!("{:?}", spec.spec_type))
            .or_insert(0);
        *type_count += 1;

        // Add to specifications list
        self.specifications.push(spec);
        self.total_specs = self.specifications.len();
    }

    /// Find specifications by type
    pub fn find_by_type(&self, spec_type: SpecType) -> Vec<&SpecMetadata> {
        self.specifications
            .iter()
            .filter(|spec| spec.spec_type == spec_type)
            .collect()
    }

    /// Find specifications by format
    pub fn find_by_format(&self, format: SpecFormat) -> Vec<&SpecMetadata> {
        self.specifications
            .iter()
            .filter(|spec| spec.format == format)
            .collect()
    }
}

/// Compliance violation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Unique violation identifier
    pub id: String,
    /// Violation severity
    pub severity: ComplianceLevel,
    /// Violation category (EDGS, LAIO, TES, etc.)
    pub category: String,
    /// Violation code for machine reference
    pub code: String,
    /// Human-readable description
    pub description: String,
    /// File or resource affected
    pub affected_path: Option<PathBuf>,
    /// Line number if applicable
    pub line_number: Option<u32>,
    /// Remediation suggestion
    pub remediation: String,
    /// Reference to standard or requirement
    pub reference: Option<String>,
    /// Timestamp when violation was detected
    pub detected_at: DateTime<Utc>,
    /// Whether violation is auto-fixable
    pub auto_fixable: bool,
    /// Additional context data
    pub context: HashMap<String, String>,
}

impl ComplianceViolation {
    /// Create new compliance violation
    pub fn new(
        severity: ComplianceLevel,
        category: String,
        code: String,
        description: String,
        remediation: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            severity,
            category,
            code,
            description,
            affected_path: None,
            line_number: None,
            remediation,
            reference: None,
            detected_at: Utc::now(),
            auto_fixable: false,
            context: HashMap::new(),
        }
    }

    /// Add affected path to violation
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.affected_path = Some(path);
        self
    }

    /// Add line number to violation
    pub fn with_line(mut self, line: u32) -> Self {
        self.line_number = Some(line);
        self
    }

    /// Add reference to violation
    pub fn with_reference(mut self, reference: String) -> Self {
        self.reference = Some(reference);
        self
    }

    /// Mark as auto-fixable
    pub fn auto_fixable(mut self) -> Self {
        self.auto_fixable = true;
        self
    }

    /// Add context data
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }
}

/// Generated specification information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedSpec {
    /// Generated specification ID
    pub id: String,
    /// Original source path (if converted)
    pub source_path: Option<PathBuf>,
    /// Generated file path
    pub output_path: PathBuf,
    /// Specification type
    pub spec_type: SpecType,
    /// Generation reason
    pub reason: String,
    /// Generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Whether spec was automatically created
    pub auto_generated: bool,
}

/// Generated Beads issue information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedBeadsIssue {
    /// Issue ID
    pub id: String,
    /// Issue title
    pub title: String,
    /// Issue description
    pub description: String,
    /// Priority level
    pub priority: String,
    /// Issue category
    pub category: String,
    /// Related violation IDs
    pub related_violations: Vec<String>,
    /// Generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Estimated effort (if available)
    pub estimated_effort: Option<String>,
}

/// Complete audit report for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    /// Project name
    pub project_name: String,
    /// Project path
    pub project_path: PathBuf,
    /// Audit timestamp
    pub audit_timestamp: DateTime<Utc>,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// Specification inventory
    pub spec_inventory: SpecInventory,
    /// All violations found
    pub violations: Vec<ComplianceViolation>,
    /// Generated specifications
    pub generated_specs: Vec<GeneratedSpec>,
    /// Generated Beads issues
    pub generated_beads: Vec<GeneratedBeadsIssue>,
    /// Baseline image path (if created)
    pub baseline_image_path: Option<PathBuf>,
    /// Audit execution time in milliseconds
    pub execution_time_ms: u64,
    /// Audit configuration used
    pub audit_config: AuditConfig,
    /// Summary statistics
    pub summary: AuditSummary,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Whether to capture baseline
    pub capture_baseline: bool,
    /// Whether to generate specifications
    pub generate_specs: bool,
    /// Whether to create Beads issues
    pub create_beads: bool,
    /// Output directory for reports
    pub output_dir: Option<PathBuf>,
    /// Severity threshold for violations
    pub severity_threshold: Option<ComplianceLevel>,
    /// Custom validation rules
    pub custom_rules: Vec<String>,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            capture_baseline: false,
            generate_specs: false,
            create_beads: false,
            output_dir: None,
            severity_threshold: None,
            custom_rules: Vec::new(),
        }
    }
}

/// Audit summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSummary {
    /// Total files scanned
    pub files_scanned: usize,
    /// Total directories scanned
    pub directories_scanned: usize,
    /// Total data size scanned
    pub total_size_bytes: u64,
    /// Violations by category
    pub violations_by_category: HashMap<String, usize>,
    /// Most common violation types
    pub top_violations: Vec<(String, usize)>,
    /// Compliance trend (if historical data available)
    pub compliance_trend: Option<f32>,
    /// Recommendations count
    pub recommendations_count: usize,
}

impl AuditReport {
    /// Create new audit report
    pub fn new(project_name: String, project_path: PathBuf, config: AuditConfig) -> Self {
        Self {
            project_name,
            project_path,
            audit_timestamp: Utc::now(),
            compliance_status: ComplianceStatus::new(),
            spec_inventory: SpecInventory::new(),
            violations: Vec::new(),
            generated_specs: Vec::new(),
            generated_beads: Vec::new(),
            baseline_image_path: None,
            execution_time_ms: 0,
            audit_config: config,
            summary: AuditSummary {
                files_scanned: 0,
                directories_scanned: 0,
                total_size_bytes: 0,
                violations_by_category: HashMap::new(),
                top_violations: Vec::new(),
                compliance_trend: None,
                recommendations_count: 0,
            },
        }
    }

    /// Add violation to audit report
    pub fn add_violation(&mut self, violation: ComplianceViolation) {
        // Update category counts
        let count = self.summary.violations_by_category
            .entry(violation.category.clone())
            .or_insert(0);
        *count += 1;

        self.violations.push(violation);
    }

    /// Finalize audit report (calculate scores, etc.)
    pub fn finalize(&mut self) {
        // Calculate compliance score
        self.compliance_status.calculate_score(&self.violations);

        // Update top violations
        let mut category_counts: Vec<(String, usize)> = self.summary.violations_by_category
            .iter()
            .map(|(cat, count)| (cat.clone(), *count))
            .collect();
        category_counts.sort_by(|a, b| b.1.cmp(&a.1));
        self.summary.top_violations = category_counts.into_iter().take(5).collect();

        // Update compliance status last audit time
        self.compliance_status.last_audit = Some(self.audit_timestamp);
    }

    /// Get violations by severity
    pub fn violations_by_severity(&self) -> HashMap<ComplianceLevel, usize> {
        let mut counts = HashMap::new();
        for violation in &self.violations {
            let count = counts.entry(violation.severity.clone()).or_insert(0);
            *count += 1;
        }
        counts
    }

    /// Get critical violations
    pub fn critical_violations(&self) -> Vec<&ComplianceViolation> {
        self.violations
            .iter()
            .filter(|v| matches!(v.severity, ComplianceLevel::Critical))
            .collect()
    }

    /// Get auto-fixable violations
    pub fn auto_fixable_violations(&self) -> Vec<&ComplianceViolation> {
        self.violations
            .iter()
            .filter(|v| v.auto_fixable)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_compliance_level_score() {
        assert_eq!(ComplianceLevel::Critical.score(), 0.0);
        assert_eq!(ComplianceLevel::High.score(), 25.0);
        assert_eq!(ComplianceLevel::Medium.score(), 50.0);
        assert_eq!(ComplianceLevel::Low.score(), 75.0);
    }

    #[test]
    fn test_compliance_level_color() {
        assert_eq!(ComplianceLevel::Critical.color(), "red");
        assert_eq!(ComplianceLevel::High.color(), "orange");
        assert_eq!(ComplianceLevel::Medium.color(), "yellow");
        assert_eq!(ComplianceLevel::Low.color(), "green");
    }

    #[test]
    fn test_edgs_phase() {
        let phase = EdgsPhase::new(2);
        assert_eq!(phase.phase, 2);
        assert_eq!(phase.phase_name(), "Monitoring & Validation");
        assert!(phase.is_valid());
        assert!(!phase.regression_detected);
    }

    #[test]
    fn test_edgs_phase_invalid() {
        let phase = EdgsPhase::new(5);
        assert!(!phase.is_valid());
        assert_eq!(phase.phase_name(), "Unknown");
    }

    #[test]
    fn test_laio_classification() {
        let laio = LaioClassification::new(5, "L2".to_string());
        assert_eq!(laio.domain, 5);
        assert_eq!(laio.maturity, "L2");
        assert!(laio.is_valid_domain());
        assert!(laio.is_valid_maturity());
        assert_eq!(laio.domain_name(), "Domain 5: Project Management & Governance");
    }

    #[test]
    fn test_laio_classification_invalid() {
        let laio = LaioClassification::new(11, "L5".to_string());
        assert!(!laio.is_valid_domain());
        assert!(!laio.is_valid_maturity());
    }

    #[test]
    fn test_compliance_status_calculation() {
        let mut status = ComplianceStatus::new();
        let violations = vec![
            ComplianceViolation::new(
                ComplianceLevel::Critical,
                "EDGS".to_string(),
                "EDGS-001".to_string(),
                "Missing phase gate".to_string(),
                "Add phase gate documentation".to_string(),
            ),
            ComplianceViolation::new(
                ComplianceLevel::High,
                "LAIO".to_string(),
                "LAIO-001".to_string(),
                "Missing constitution".to_string(),
                "Create constitution.yaml".to_string(),
            ),
        ];

        status.calculate_score(&violations);
        assert_eq!(status.total_violations, 2);
        assert_eq!(status.score, 60.0); // 100 - 25 - 15
        assert_eq!(status.level, ComplianceLevel::High);
    }

    #[test]
    fn test_spec_metadata() {
        let spec = SpecMetadata::new(PathBuf::from("/test/spec.md"));
        assert_eq!(spec.format, SpecFormat::Unknown);
        assert_eq!(spec.spec_type, SpecType::Unknown);
        assert!(!spec.is_valid());
    }

    #[test]
    fn test_spec_metadata_valid() {
        let mut spec = SpecMetadata::new(PathBuf::from("/test/spec.md"));
        spec.title = "Test Specification".to_string();
        spec.size_bytes = 1024;
        assert!(spec.is_valid());
    }

    #[test]
    fn test_spec_inventory() {
        let mut inventory = SpecInventory::new();
        assert_eq!(inventory.total_specs, 0);

        let mut spec = SpecMetadata::new(PathBuf::from("/test/spec.md"));
        spec.title = "Test Spec".to_string();
        spec.size_bytes = 1024;
        spec.format = SpecFormat::Markdown;
        spec.spec_type = SpecType::Design;

        inventory.add_spec(spec);
        assert_eq!(inventory.total_specs, 1);
        assert_eq!(inventory.by_format.get("Markdown"), Some(&1));
        assert_eq!(inventory.by_type.get("Design"), Some(&1));
    }

    #[test]
    fn test_compliance_violation() {
        let violation = ComplianceViolation::new(
            ComplianceLevel::Critical,
            "EDGS".to_string(),
            "EDGS-001".to_string(),
            "Test violation".to_string(),
            "Fix it".to_string(),
        )
        .with_path(PathBuf::from("/test/file.md"))
        .with_line(42)
        .with_reference("EDGS-Standard".to_string())
        .auto_fixable()
        .with_context("key".to_string(), "value".to_string());

        assert_eq!(violation.severity, ComplianceLevel::Critical);
        assert_eq!(violation.category, "EDGS");
        assert_eq!(violation.code, "EDGS-001");
        assert!(violation.affected_path.is_some());
        assert_eq!(violation.line_number, Some(42));
        assert!(violation.auto_fixable);
        assert_eq!(violation.context.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_audit_report() {
        let config = AuditConfig::default();
        let mut report = AuditReport::new(
            "test-project".to_string(),
            PathBuf::from("/test/project"),
            config,
        );

        assert_eq!(report.project_name, "test-project");
        assert_eq!(report.violations.len(), 0);
        assert_eq!(report.spec_inventory.total_specs, 0);

        let violation = ComplianceViolation::new(
            ComplianceLevel::Medium,
            "TEST".to_string(),
            "TEST-001".to_string(),
            "Test violation".to_string(),
            "Fix it".to_string(),
        );

        report.add_violation(violation);
        report.finalize();

        assert_eq!(report.violations.len(), 1);
        assert_eq!(report.compliance_status.total_violations, 1);
        assert_eq!(report.summary.violations_by_category.get("TEST"), Some(&1));
    }

    #[test]
    fn test_audit_report_violations_by_severity() {
        let config = AuditConfig::default();
        let mut report = AuditReport::new(
            "test-project".to_string(),
            PathBuf::from("/test/project"),
            config,
        );

        report.add_violation(ComplianceViolation::new(
            ComplianceLevel::Critical,
            "TEST".to_string(),
            "TEST-001".to_string(),
            "Critical".to_string(),
            "Fix".to_string(),
        ));

        report.add_violation(ComplianceViolation::new(
            ComplianceLevel::High,
            "TEST".to_string(),
            "TEST-002".to_string(),
            "High".to_string(),
            "Fix".to_string(),
        ));

        let by_severity = report.violations_by_severity();
        assert_eq!(by_severity.get(&ComplianceLevel::Critical), Some(&1));
        assert_eq!(by_severity.get(&ComplianceLevel::High), Some(&1));
    }

    #[test]
    fn test_serialization() {
        let status = ComplianceStatus::new();
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: ComplianceStatus = serde_json::from_str(&json).unwrap();
        
        assert_eq!(status.score, deserialized.score);
        assert_eq!(status.level, deserialized.level);
    }
}