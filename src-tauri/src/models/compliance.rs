/// Compliance validation models for PACS
///
/// This module defines data structures for compliance validation,
/// violation detection, and scoring algorithms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Re-export from project_audit module for convenience
pub use crate::models::project_audit::{ComplianceViolation, ComplianceLevel};

/// Compliance validation result for a specific validator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceValidationResult {
    /// Validator name (e.g., "EDGS", "LAIO", "TES-2025")
    pub validator: String,
    /// Validation timestamp
    pub validated_at: DateTime<Utc>,
    /// Overall validation result
    pub is_compliant: bool,
    /// Compliance score (0.0-1.0)
    pub compliance_score: f32,
    /// Violations found
    pub violations: Vec<ComplianceViolation>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
    /// Validator-specific metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Validation warning (less severe than violation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Warning code
    pub code: String,
    /// Warning category
    pub category: String,
    /// Warning message
    pub message: String,
    /// File or resource affected
    pub affected_path: Option<PathBuf>,
    /// Line number if applicable
    pub line_number: Option<u32>,
    /// Suggested improvement
    pub suggestion: Option<String>,
    /// Reference to standard or best practice
    pub reference: Option<String>,
    /// Timestamp when warning was generated
    pub generated_at: DateTime<Utc>,
}

impl ValidationWarning {
    /// Create new validation warning
    pub fn new(
        category: String,
        code: String,
        message: String,
    ) -> Self {
        Self {
            category,
            code,
            message,
            affected_path: None,
            line_number: None,
            suggestion: None,
            reference: None,
            generated_at: Utc::now(),
        }
    }

    /// Add affected path to warning
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.affected_path = Some(path);
        self
    }

    /// Add line number to warning
    pub fn with_line(mut self, line: u32) -> Self {
        self.line_number = Some(line);
        self
    }

    /// Add suggestion to warning
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// Add reference to warning
    pub fn with_reference(mut self, reference: String) -> Self {
        self.reference = Some(reference);
        self
    }
}

/// EDGS compliance validation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgsComplianceResult {
    /// Current EDGS phase (0-3)
    pub current_phase: u8,
    /// Phase gates completed
    pub gates_completed: Vec<String>,
    /// Missing required gates
    pub missing_gates: Vec<String>,
    /// Phase regression detected
    pub regression_detected: bool,
    /// Schedule file found and valid
    pub schedule_valid: bool,
    /// PoE (Proof of Execution) bundles present
    pub poe_bundles_present: bool,
    /// Last gate approval date
    pub last_approval_date: Option<DateTime<Utc>>,
    /// Phase progression timeline
    pub phase_timeline: Vec<PhaseMilestone>,
}

/// EDGS phase milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseMilestone {
    /// Phase number
    pub phase: u8,
    /// Phase name
    pub phase_name: String,
    /// Completion date
    pub completed_at: Option<DateTime<Utc>>,
    /// Approval status
    pub approval_status: String,
    /// Notes
    pub notes: Option<String>,
}

/// LAIO compliance validation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaioComplianceResult {
    /// Domain number (1-10)
    pub domain: u8,
    /// Maturity level (L0-L4)
    pub maturity_level: String,
    /// Constitution file found
    pub constitution_found: bool,
    /// Constitution file path
    pub constitution_path: Option<PathBuf>,
    /// Constitution valid YAML
    pub constitution_valid: bool,
    /// Domain classification correct
    pub domain_classification_valid: bool,
    /// Maturity assessment accurate
    pub maturity_assessment_valid: bool,
    /// Last classification update
    pub last_updated: Option<DateTime<Utc>>,
}

/// TES-2025 compliance validation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TesComplianceResult {
    /// Event-driven tasks only (no time estimates)
    pub event_driven_only: bool,
    /// Time estimates found (violations)
    pub time_estimates_found: Vec<String>,
    /// Audit trail present
    pub audit_trail_present: bool,
    /// Event sequencing correct
    pub event_sequencing_valid: bool,
    /// Task dependencies clear
    pub dependencies_clear: bool,
    /// Completion criteria defined
    pub completion_criteria_defined: bool,
    /// Progress tracking mechanism
    pub progress_tracking_valid: bool,
}

/// Bloom taxonomy compliance validation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloomComplianceResult {
    /// Knowledge phase documented
    pub knowledge_phase_documented: bool,
    /// Application phase documented
    pub application_phase_documented: bool,
    /// Evaluation phase documented
    pub evaluation_phase_documented: bool,
    /// All three phases present
    pub all_phases_present: bool,
    /// Phase progression logical
    pub progression_logical: bool,
    /// Assessment methods appropriate
    pub assessment_appropriate: bool,
    /// Learning objectives clear
    pub objectives_clear: bool,
}

/// OpenSpec compliance validation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenspecComplianceResult {
    /// Specifications follow canonical format
    pub canonical_format_followed: bool,
    /// Frontmatter complete
    pub frontmatter_complete: bool,
    /// Metadata standards followed
    pub metadata_standards_followed: bool,
    /// Document structure valid
    pub structure_valid: bool,
    /// Approval workflow present
    pub approval_workflow_present: bool,
    /// Version control proper
    pub version_control_proper: bool,
    /// Cross-references valid
    pub cross_references_valid: bool,
}

/// Project structure compliance validation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureComplianceResult {
    /// Docs directory organized
    pub docs_directory_organized: bool,
    /// No rogue markdown files in root
    pub no_rogue_markdown: bool,
    /// Directory structure follows standards
    pub directory_structure_valid: bool,
    /// File naming conventions followed
    pub naming_conventions_followed: bool,
    /// Required directories present
    pub required_directories_present: bool,
    /// Temporary files properly placed
    pub temp_files_contained: bool,
    /// Backup files organized
    pub backup_files_organized: bool,
}

/// Overall compliance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    /// Overall compliance score (0.0-100.0)
    pub overall_score: f32,
    /// Compliance level
    pub compliance_level: ComplianceLevel,
    /// Total violations count
    pub total_violations: usize,
    /// Violations by severity
    pub violations_by_severity: HashMap<ComplianceLevel, usize>,
    /// Violations by category
    pub violations_by_category: HashMap<String, usize>,
    /// Total warnings count
    pub total_warnings: usize,
    /// Warnings by category
    pub warnings_by_category: HashMap<String, usize>,
    /// Validator results
    pub validator_results: HashMap<String, ComplianceValidationResult>,
    /// Compliance trend (if historical data available)
    pub compliance_trend: Option<f32>,
    /// Last validation timestamp
    pub last_validated: DateTime<Utc>,
}

impl ComplianceMetrics {
    /// Create new compliance metrics
    pub fn new() -> Self {
        Self {
            overall_score: 0.0,
            compliance_level: ComplianceLevel::Critical,
            total_violations: 0,
            violations_by_severity: HashMap::new(),
            violations_by_category: HashMap::new(),
            total_warnings: 0,
            warnings_by_category: HashMap::new(),
            validator_results: HashMap::new(),
            compliance_trend: None,
            last_validated: Utc::now(),
        }
    }

    /// Calculate overall compliance score from validator results
    pub fn calculate_score(&mut self, validator_results: HashMap<String, ComplianceValidationResult>) {
        self.validator_results = validator_results;
        
        if self.validator_results.is_empty() {
            self.overall_score = 0.0;
            self.compliance_level = ComplianceLevel::Critical;
            return;
        }

        // Count violations and warnings
        self.total_violations = 0;
        self.total_warnings = 0;
        self.violations_by_severity.clear();
        self.violations_by_category.clear();
        self.warnings_by_category.clear();

        let mut total_score = 0.0;
        let mut validator_count = 0;

        for (validator_name, result) in &self.validator_results {
            total_score += result.compliance_score;
            validator_count += 1;

            // Count violations
            for violation in &result.violations {
                self.total_violations += 1;
                
                // Count by severity
                let severity_count = self.violations_by_severity
                    .entry(violation.severity.clone())
                    .or_insert(0);
                *severity_count += 1;

                // Count by category
                let category_count = self.violations_by_category
                    .entry(violation.category.clone())
                    .or_insert(0);
                *category_count += 1;
            }

            // Count warnings
            for warning in &result.warnings {
                self.total_warnings += 1;
                
                let category_count = self.warnings_by_category
                    .entry(warning.category.clone())
                    .or_insert(0);
                *category_count += 1;
            }
        }

        // Calculate average score
        self.overall_score = if validator_count > 0 {
            (total_score / validator_count as f32) * 100.0
        } else {
            0.0
        };

        // Determine compliance level
        self.compliance_level = if self.overall_score >= 90.0 {
            ComplianceLevel::Low
        } else if self.overall_score >= 75.0 {
            ComplianceLevel::Medium
        } else if self.overall_score >= 50.0 {
            ComplianceLevel::High
        } else {
            ComplianceLevel::Critical
        };

        self.last_validated = Utc::now();
    }

    /// Check if project is compliant
    pub fn is_compliant(&self) -> bool {
        self.overall_score >= 70.0 && 
        !self.violations_by_severity.contains_key(&ComplianceLevel::Critical)
    }

    /// Get violations by severity as string keys
    pub fn violations_by_severity_string(&self) -> HashMap<String, usize> {
        self.violations_by_severity
            .iter()
            .map(|(level, count)| (format!("{:?}", level), *count))
            .collect()
    }

    /// Get critical violations count
    pub fn critical_violations_count(&self) -> usize {
        self.violations_by_severity
            .get(&ComplianceLevel::Critical)
            .copied()
            .unwrap_or(0)
    }

    /// Get high violations count
    pub fn high_violations_count(&self) -> usize {
        self.violations_by_severity
            .get(&ComplianceLevel::High)
            .copied()
            .unwrap_or(0)
    }
}

/// Compliance validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceValidationConfig {
    /// Validators to run
    pub enabled_validators: Vec<String>,
    /// Severity threshold for violations
    pub severity_threshold: Option<ComplianceLevel>,
    /// Custom validation rules
    pub custom_rules: Vec<CustomValidationRule>,
    /// Exclude patterns
    pub exclude_patterns: Vec<String>,
    /// Include patterns (overrides exclude)
    pub include_patterns: Vec<String>,
    /// Validation timeout in seconds
    pub timeout_seconds: Option<u64>,
}

/// Custom validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomValidationRule {
    /// Rule identifier
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule pattern (regex or glob)
    pub pattern: String,
    /// Rule severity
    pub severity: ComplianceLevel,
    /// Rule category
    pub category: String,
    /// Validation logic (pseudo-code or reference)
    pub validation_logic: String,
    /// Whether rule is enabled
    pub enabled: bool,
}

impl Default for ComplianceValidationConfig {
    fn default() -> Self {
        Self {
            enabled_validators: vec![
                "EDGS".to_string(),
                "LAIO".to_string(),
                "TES-2025".to_string(),
                "Bloom".to_string(),
                "OpenSpec".to_string(),
                "Structure".to_string(),
            ],
            severity_threshold: None,
            custom_rules: Vec::new(),
            exclude_patterns: Vec::new(),
            include_patterns: Vec::new(),
            timeout_seconds: Some(300), // 5 minutes default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_warning_creation() {
        let warning = ValidationWarning::new(
            "TEST".to_string(),
            "TEST-001".to_string(),
            "Test warning".to_string(),
        )
        .with_path(PathBuf::from("/test/file.md"))
        .with_line(42)
        .with_suggestion("Fix it".to_string())
        .with_reference("Standard-1".to_string());

        assert_eq!(warning.category, "TEST");
        assert_eq!(warning.code, "TEST-001");
        assert_eq!(warning.message, "Test warning");
        assert!(warning.affected_path.is_some());
        assert_eq!(warning.line_number, Some(42));
        assert!(warning.suggestion.is_some());
        assert!(warning.reference.is_some());
    }

    #[test]
    fn test_edgs_compliance_result() {
        let result = EdgsComplianceResult {
            current_phase: 2,
            gates_completed: vec!["Gate 1".to_string(), "Gate 2".to_string()],
            missing_gates: vec!["Gate 3".to_string()],
            regression_detected: false,
            schedule_valid: true,
            poe_bundles_present: true,
            last_approval_date: Some(Utc::now()),
            phase_timeline: vec![
                PhaseMilestone {
                    phase: 1,
                    phase_name: "Foundation".to_string(),
                    completed_at: Some(Utc::now()),
                    approval_status: "Approved".to_string(),
                    notes: None,
                },
            ],
        };

        assert_eq!(result.current_phase, 2);
        assert_eq!(result.gates_completed.len(), 2);
        assert_eq!(result.missing_gates.len(), 1);
        assert!(!result.regression_detected);
        assert!(result.schedule_valid);
        assert!(result.poe_bundles_present);
    }

    #[test]
    fn test_laio_compliance_result() {
        let result = LaioComplianceResult {
            domain: 5,
            maturity_level: "L2".to_string(),
            constitution_found: true,
            constitution_path: Some(PathBuf::from("/.laio/constitution.yaml")),
            constitution_valid: true,
            domain_classification_valid: true,
            maturity_assessment_valid: true,
            last_updated: Some(Utc::now()),
        };

        assert_eq!(result.domain, 5);
        assert_eq!(result.maturity_level, "L2");
        assert!(result.constitution_found);
        assert!(result.constitution_valid);
        assert!(result.domain_classification_valid);
    }

    #[test]
    fn test_tes_compliance_result() {
        let result = TesComplianceResult {
            event_driven_only: true,
            time_estimates_found: vec!["task1.md".to_string()],
            audit_trail_present: true,
            event_sequencing_valid: true,
            dependencies_clear: true,
            completion_criteria_defined: true,
            progress_tracking_valid: true,
        };

        assert!(result.event_driven_only);
        assert_eq!(result.time_estimates_found.len(), 1);
        assert!(result.audit_trail_present);
        assert!(result.event_sequencing_valid);
    }

    #[test]
    fn test_compliance_metrics_calculation() {
        let mut metrics = ComplianceMetrics::new();
        
        let mut validator_results = HashMap::new();
        
        // Add EDGS validator result
        let edgs_result = ComplianceValidationResult {
            validator: "EDGS".to_string(),
            validated_at: Utc::now(),
            is_compliant: false,
            compliance_score: 0.8,
            violations: vec![
                ComplianceViolation::new(
                    ComplianceLevel::High,
                    "EDGS".to_string(),
                    "EDGS-001".to_string(),
                    "Missing gate".to_string(),
                    "Add gate".to_string(),
                ),
            ],
            warnings: vec![],
            metadata: HashMap::new(),
        };
        
        validator_results.insert("EDGS".to_string(), edgs_result);
        
        metrics.calculate_score(validator_results);
        
        assert_eq!(metrics.overall_score, 80.0);
        assert_eq!(metrics.total_violations, 1);
        assert_eq!(metrics.compliance_level, ComplianceLevel::Medium);
        assert!(metrics.violations_by_severity.contains_key(&ComplianceLevel::High));
    }

    #[test]
    fn test_compliance_metrics_is_compliant() {
        let mut metrics = ComplianceMetrics::new();
        metrics.overall_score = 85.0;
        metrics.compliance_level = ComplianceLevel::Medium;
        metrics.violations_by_severity.insert(ComplianceLevel::High, 2);
        
        assert!(metrics.is_compliant());
        
        // Add critical violation
        metrics.violations_by_severity.insert(ComplianceLevel::Critical, 1);
        assert!(!metrics.is_compliant());
    }

    #[test]
    fn test_compliance_validation_config_default() {
        let config = ComplianceValidationConfig::default();
        
        assert_eq!(config.enabled_validators.len(), 6);
        assert!(config.enabled_validators.contains(&"EDGS".to_string()));
        assert!(config.enabled_validators.contains(&"LAIO".to_string()));
        assert_eq!(config.timeout_seconds, Some(300));
        assert!(config.custom_rules.is_empty());
        assert!(config.exclude_patterns.is_empty());
    }

    #[test]
    fn test_custom_validation_rule() {
        let rule = CustomValidationRule {
            id: "CUSTOM-001".to_string(),
            name: "Custom Rule".to_string(),
            description: "Test custom rule".to_string(),
            pattern: r"TODO.*".to_string(),
            severity: ComplianceLevel::Medium,
            category: "Hygiene".to_string(),
            validation_logic: "Check for TODO comments".to_string(),
            enabled: true,
        };

        assert_eq!(rule.id, "CUSTOM-001");
        assert_eq!(rule.name, "Custom Rule");
        assert_eq!(rule.severity, ComplianceLevel::Medium);
        assert!(rule.enabled);
    }

    #[test]
    fn test_serialization() {
        let metrics = ComplianceMetrics::new();
        let json = serde_json::to_string(&metrics).unwrap();
        let deserialized: ComplianceMetrics = serde_json::from_str(&json).unwrap();
        
        assert_eq!(metrics.overall_score, deserialized.overall_score);
        assert_eq!(metrics.total_violations, deserialized.total_violations);
    }
}