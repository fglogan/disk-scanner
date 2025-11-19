/// Specification data structures for PACS
///
/// This module defines models for handling different specification formats,
/// parsing results, and OpenSpec document structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Re-export from project_audit module for convenience
pub use crate::models::project_audit::{SpecFormat, SpecType, SpecMetadata};

/// OpenSpec document structure (canonical format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSpecDocument {
    /// Document metadata
    pub metadata: OpenSpecMetadata,
    /// Document content sections
    pub sections: Vec<OpenSpecSection>,
    /// Document relationships
    pub relationships: Vec<OpenSpecRelationship>,
    /// Document approval workflow
    pub approval: Option<OpenSpecApproval>,
}

/// OpenSpec metadata block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSpecMetadata {
    /// Document identifier (UUID)
    pub id: String,
    /// Document title
    pub title: String,
    /// Document version (semantic versioning)
    pub version: String,
    /// Document status
    pub status: OpenSpecStatus,
    /// Specification type
    pub spec_type: SpecType,
    /// Document author(s)
    pub authors: Vec<String>,
    /// Creation date
    pub created: DateTime<Utc>,
    /// Last modification date
    pub modified: DateTime<Utc>,
    /// Reviewers
    pub reviewers: Vec<String>,
    /// Approvers
    pub approvers: Vec<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Related Beads issues
    pub beads_issues: Vec<String>,
    /// Dependencies on other specifications
    pub dependencies: Vec<String>,
    /// EDGS phase information
    pub edgs_phase: Option<u8>,
    /// LAIO classification
    pub laio_domain: Option<u8>,
    /// LAIO maturity level
    pub laio_maturity: Option<String>,
    /// Compliance standards referenced
    pub compliance_standards: Vec<String>,
    /// Custom metadata fields
    pub custom: HashMap<String, serde_json::Value>,
}

/// OpenSpec document status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpenSpecStatus {
    /// Draft - work in progress
    Draft,
    /// Review - ready for review
    Review,
    /// Approved - formally approved
    Approved,
    /// Deprecated - superseded by newer version
    Deprecated,
    /// Archived - no longer active
    Archived,
}

impl OpenSpecStatus {
    /// Get status description
    pub fn description(&self) -> &'static str {
        match self {
            OpenSpecStatus::Draft => "Work in progress",
            OpenSpecStatus::Review => "Ready for review",
            OpenSpecStatus::Approved => "Formally approved",
            OpenSpecStatus::Deprecated => "Superseded by newer version",
            OpenSpecStatus::Archived => "No longer active",
        }
    }

    /// Check if status allows editing
    pub fn is_editable(&self) -> bool {
        matches!(self, OpenSpecStatus::Draft | OpenSpecStatus::Review)
    }

    /// Check if status is final
    pub fn is_final(&self) -> bool {
        matches!(self, OpenSpecStatus::Approved | OpenSpecStatus::Deprecated | OpenSpecStatus::Archived)
    }
}

/// OpenSpec content section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSpecSection {
    /// Section identifier
    pub id: String,
    /// Section title
    pub title: String,
    /// Section level (1-6 for headings)
    pub level: u8,
    /// Section content (Markdown)
    pub content: String,
    /// Subsections
    pub subsections: Vec<OpenSpecSection>,
    /// Section metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Required for compliance
    pub required: bool,
    /// Review status for this section
    pub review_status: Option<OpenSpecStatus>,
}

/// OpenSpec relationship to other documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSpecRelationship {
    /// Relationship type
    pub relationship_type: OpenSpecRelationshipType,
    /// Target document ID or path
    pub target: String,
    /// Relationship description
    pub description: String,
    /// Whether this relationship is required
    pub required: bool,
    /// Relationship strength (0.0-1.0)
    pub strength: f32,
}

/// OpenSpec relationship types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpenSpecRelationshipType {
    /// Depends on (prerequisite)
    DependsOn,
    /// Implements (fulfills requirement)
    Implements,
    /// References (cites or mentions)
    References,
    /// Supersedes (replaces older version)
    Supersedes,
    /// Conflicts with (contradicts)
    ConflictsWith,
    /// Complements (enhances)
    Complements,
    /// Validates (tests or verifies)
    Validates,
}

/// OpenSpec approval workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSpecApproval {
    /// Approval status
    pub status: OpenSpecApprovalStatus,
    /// Required approvers
    pub required_approvers: Vec<String>,
    /// Received approvals
    pub approvals: Vec<OpenSpecApprovalRecord>,
    /// Approval requirements
    pub requirements: OpenSpecApprovalRequirements,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

/// OpenSpec approval status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpenSpecApprovalStatus {
    /// Pending approval
    Pending,
    /// Partially approved
    Partial,
    /// Fully approved
    Approved,
    /// Rejected
    Rejected,
    /// Expired (approval window passed)
    Expired,
}

/// Individual approval record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSpecApprovalRecord {
    /// Approver name or ID
    pub approver: String,
    /// Approval timestamp
    pub timestamp: DateTime<Utc>,
    /// Approval decision
    pub decision: OpenSpecApprovalDecision,
    /// Comments or rationale
    pub comments: Option<String>,
    /// Approval scope (whole document or specific sections)
    pub scope: Option<Vec<String>>,
}

/// Approval decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpenSpecApprovalDecision {
    /// Approve
    Approve,
    /// Approve with comments
    ApproveWithComments,
    /// Request changes
    RequestChanges,
    /// Reject
    Reject,
}

/// Approval requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSpecApprovalRequirements {
    /// Minimum number of approvers required
    pub min_approvers: usize,
    /// Specific required approvers (by name/role)
    pub specific_approvers: Vec<String>,
    /// Approval deadline
    pub deadline: Option<DateTime<Utc>>,
    /// Quorum percentage (for group approvals)
    pub quorum_percentage: Option<f32>,
    /// Whether consecutive approvals required
    pub consecutive_required: bool,
}

/// Parsed specification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSpec {
    /// Original file path
    pub source_path: PathBuf,
    /// Detected format
    pub format: SpecFormat,
    /// Detected specification type
    pub spec_type: SpecType,
    /// Extracted metadata
    pub metadata: SpecMetadata,
    /// Parsed content (structured)
    pub content: ParsedContent,
    /// Parsing warnings
    pub warnings: Vec<String>,
    /// Parsing errors
    pub errors: Vec<String>,
    /// Whether conversion to OpenSpec is recommended
    pub recommend_conversion: bool,
}

/// Parsed content structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParsedContent {
    /// OpenSpec document (already in canonical format)
    OpenSpec(OpenSpecDocument),
    /// Markdown with frontmatter
    Markdown {
        /// Frontmatter metadata
        frontmatter: HashMap<String, serde_json::Value>,
        /// Markdown content
        content: String,
    },
    /// YAML specification
    Yaml(serde_yaml::Value),
    /// JSON specification
    Json(serde_json::Value),
    /// TOML specification
    Toml(toml::Value),
    /// Plain text (unstructured)
    PlainText(String),
}

/// Specification conversion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecConversion {
    /// Original specification
    pub original: ParsedSpec,
    /// Converted OpenSpec document
    pub converted: OpenSpecDocument,
    /// Conversion quality score (0.0-1.0)
    pub quality_score: f32,
    /// Conversion warnings
    pub warnings: Vec<String>,
    /// Conversion errors
    pub errors: Vec<String>,
    /// Manual review required items
    pub manual_review_items: Vec<String>,
    /// Conversion timestamp
    pub converted_at: DateTime<Utc>,
}

/// Specification validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecValidation {
    /// Specification being validated
    pub spec_path: PathBuf,
    /// Validation timestamp
    pub validated_at: DateTime<Utc>,
    /// Overall validation result
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
    /// Compliance score (0.0-1.0)
    pub compliance_score: f32,
    /// Required fixes
    pub required_fixes: Vec<String>,
    /// Recommended improvements
    pub recommended_improvements: Vec<String>,
}

/// Validation error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Error code
    pub code: String,
    /// Error severity
    pub severity: ValidationErrorSeverity,
    /// Error message
    pub message: String,
    /// Affected section or field
    pub location: Option<String>,
    /// Line number if applicable
    pub line_number: Option<u32>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
    /// Reference to standard or requirement
    pub reference: Option<String>,
}

/// Validation error severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationErrorSeverity {
    /// Error - must be fixed
    Error,
    /// Warning - should be fixed
    Warning,
    /// Info - consider fixing
    Info,
}

/// Validation warning details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Warning code
    pub code: String,
    /// Warning message
    pub message: String,
    /// Affected section or field
    pub location: Option<String>,
    /// Line number if applicable
    pub line_number: Option<u32>,
    /// Suggested improvement
    pub suggested_improvement: Option<String>,
}

/// Specification template for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecTemplate {
    /// Template identifier
    pub id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Specification type this template creates
    pub spec_type: SpecType,
    /// Template sections
    pub sections: Vec<TemplateSection>,
    /// Required metadata fields
    pub required_metadata: Vec<String>,
    /// Optional metadata fields
    pub optional_metadata: Vec<String>,
    /// Template variables
    pub variables: HashMap<String, TemplateVariable>,
}

/// Template section definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    /// Section identifier
    pub id: String,
    /// Section title template
    pub title_template: String,
    /// Section content template
    pub content_template: String,
    /// Section level
    pub level: u8,
    /// Whether section is required
    pub required: bool,
    /// Section conditions (when to include)
    pub condition: Option<String>,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: TemplateVariableType,
    /// Variable description
    pub description: String,
    /// Default value
    pub default_value: Option<serde_json::Value>,
    /// Whether variable is required
    pub required: bool,
    /// Validation rules
    pub validation: Option<String>,
}

/// Template variable types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateVariableType {
    /// String value
    String,
    /// Number value
    Number,
    /// Boolean value
    Boolean,
    /// Date value
    Date,
    /// List of values
    List,
    /// Object/structure
    Object,
}

impl OpenSpecDocument {
    /// Create new OpenSpec document
    pub fn new(title: String, spec_type: SpecType) -> Self {
        let now = Utc::now();
        Self {
            metadata: OpenSpecMetadata {
                id: uuid::Uuid::new_v4().to_string(),
                title,
                version: "1.0.0".to_string(),
                status: OpenSpecStatus::Draft,
                spec_type,
                authors: Vec::new(),
                created: now,
                modified: now,
                reviewers: Vec::new(),
                approvers: Vec::new(),
                tags: Vec::new(),
                beads_issues: Vec::new(),
                dependencies: Vec::new(),
                edgs_phase: None,
                laio_domain: None,
                laio_maturity: None,
                compliance_standards: Vec::new(),
                custom: HashMap::new(),
            },
            sections: Vec::new(),
            relationships: Vec::new(),
            approval: None,
        }
    }

    /// Add section to document
    pub fn add_section(&mut self, section: OpenSpecSection) {
        self.sections.push(section);
        self.metadata.modified = Utc::now();
    }

    /// Add relationship to document
    pub fn add_relationship(&mut self, relationship: OpenSpecRelationship) {
        self.relationships.push(relationship);
        self.metadata.modified = Utc::now();
    }

    /// Update document status
    pub fn update_status(&mut self, status: OpenSpecStatus) {
        self.metadata.status = status;
        self.metadata.modified = Utc::now();
    }

    /// Get section by ID
    pub fn get_section(&self, id: &str) -> Option<&OpenSpecSection> {
        self.sections.iter().find(|s| s.id == id)
    }

    /// Get section by ID (mutable)
    pub fn get_section_mut(&mut self, id: &str) -> Option<&mut OpenSpecSection> {
        self.sections.iter_mut().find(|s| s.id == id)
    }

    /// Validate document structure
    pub fn validate(&self) -> SpecValidation {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate metadata
        if self.metadata.title.is_empty() {
            errors.push(ValidationError {
                code: "SPEC-001".to_string(),
                severity: ValidationErrorSeverity::Error,
                message: "Document title is required".to_string(),
                location: Some("metadata.title".to_string()),
                line_number: None,
                suggested_fix: Some("Add a meaningful title to the document".to_string()),
                reference: Some("OpenSpec v1.0 Section 2.1".to_string()),
            });
        }

        if self.metadata.authors.is_empty() {
            warnings.push(ValidationWarning {
                code: "SPEC-002".to_string(),
                message: "No authors specified".to_string(),
                location: Some("metadata.authors".to_string()),
                line_number: None,
                suggested_improvement: Some("Add at least one author to the document".to_string()),
            });
        }

        // Validate sections
        if self.sections.is_empty() {
            errors.push(ValidationError {
                code: "SPEC-003".to_string(),
                severity: ValidationErrorSeverity::Error,
                message: "Document must have at least one section".to_string(),
                location: Some("sections".to_string()),
                line_number: None,
                suggested_fix: Some("Add content sections to the document".to_string()),
                reference: Some("OpenSpec v1.0 Section 3.0".to_string()),
            });
        }

        // Calculate compliance score
        let compliance_score = if errors.is_empty() {
            1.0 - (warnings.len() as f32 * 0.1) // Deduct 0.1 per warning
        } else {
            0.0
        }.max(0.0);

        SpecValidation {
            spec_path: PathBuf::new(), // Will be set by caller
            validated_at: Utc::now(),
            is_valid: errors.is_empty(),
            errors,
            warnings,
            compliance_score,
            required_fixes: Vec::new(),
            recommended_improvements: Vec::new(),
        }
    }

    /// Generate table of contents
    pub fn generate_toc(&self) -> String {
        let mut toc = String::new();
        toc.push_str("# Table of Contents\n\n");
        
        for section in &self.sections {
            let indent = "  ".repeat(section.level as usize - 1);
            toc.push_str(&format!("{}- [{}](#{})\n", indent, section.title, section.id));
            
            // Add subsections
            for subsection in &section.subsections {
                let sub_indent = "  ".repeat(subsection.level as usize - 1);
                toc.push_str(&format!("{}- [{}](#{})\n", sub_indent, subsection.title, subsection.id));
            }
        }
        
        toc
    }

    /// Export to Markdown format
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        
        // Frontmatter
        markdown.push_str("---\n");
        markdown.push_str(&format!("id: {}\n", self.metadata.id));
        markdown.push_str(&format!("title: {}\n", self.metadata.title));
        markdown.push_str(&format!("version: {}\n", self.metadata.version));
        markdown.push_str(&format!("status: {:?}\n", self.metadata.status));
        markdown.push_str(&format!("spec_type: {:?}\n", self.metadata.spec_type));
        markdown.push_str(&format!("created: {}\n", self.metadata.created.to_rfc3339()));
        markdown.push_str(&format!("modified: {}\n", self.metadata.modified.to_rfc3339()));
        
        if !self.metadata.authors.is_empty() {
            markdown.push_str(&format!("authors: {:?}\n", self.metadata.authors));
        }
        
        if !self.metadata.tags.is_empty() {
            markdown.push_str(&format!("tags: {:?}\n", self.metadata.tags));
        }
        
        markdown.push_str("---\n\n");
        
        // Table of contents
        markdown.push_str(&self.generate_toc());
        markdown.push('\n');
        
        // Content sections
        for section in &self.sections {
            markdown.push_str(&format!("{} {}\n\n", "#".repeat(section.level as usize), section.title));
            markdown.push_str(&section.content);
            markdown.push_str("\n\n");
            
            // Subsections
            for subsection in &section.subsections {
                markdown.push_str(&format!("{} {}\n\n", "#".repeat(subsection.level as usize), subsection.title));
                markdown.push_str(&subsection.content);
                markdown.push_str("\n\n");
            }
        }
        
        markdown
    }
}

impl ParsedSpec {
    /// Create new parsed specification
    pub fn new(source_path: PathBuf, format: SpecFormat) -> Self {
        Self {
            source_path,
            format,
            spec_type: SpecType::Unknown,
            metadata: SpecMetadata::new(PathBuf::new()),
            content: ParsedContent::PlainText(String::new()),
            warnings: Vec::new(),
            errors: Vec::new(),
            recommend_conversion: false,
        }
    }

    /// Add warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Add error
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    /// Check if parsing was successful
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get content as plain text
    pub fn get_text_content(&self) -> String {
        match &self.content {
            ParsedContent::OpenSpec(doc) => doc.to_markdown(),
            ParsedContent::Markdown { content, .. } => content.clone(),
            ParsedContent::Yaml(value) => serde_yaml::to_string(value).unwrap_or_default(),
            ParsedContent::Json(value) => serde_json::to_string_pretty(value).unwrap_or_default(),
            ParsedContent::Toml(value) => toml::to_string_pretty(value).unwrap_or_default(),
            ParsedContent::PlainText(text) => text.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openspec_document_creation() {
        let doc = OpenSpecDocument::new(
            "Test Specification".to_string(),
            SpecType::Design,
        );

        assert_eq!(doc.metadata.title, "Test Specification");
        assert_eq!(doc.metadata.spec_type, SpecType::Design);
        assert_eq!(doc.metadata.status, OpenSpecStatus::Draft);
        assert_eq!(doc.metadata.version, "1.0.0");
        assert!(doc.sections.is_empty());
        assert!(doc.relationships.is_empty());
    }

    #[test]
    fn test_openspec_status() {
        assert_eq!(OpenSpecStatus::Draft.description(), "Work in progress");
        assert!(OpenSpecStatus::Draft.is_editable());
        assert!(!OpenSpecStatus::Draft.is_final());
        
        assert_eq!(OpenSpecStatus::Approved.description(), "Formally approved");
        assert!(!OpenSpecStatus::Approved.is_editable());
        assert!(OpenSpecStatus::Approved.is_final());
    }

    #[test]
    fn test_openspec_document_validation() {
        let mut doc = OpenSpecDocument::new(
            "".to_string(), // Empty title - should cause error
            SpecType::Design,
        );

        let validation = doc.validate();
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
        assert_eq!(validation.errors[0].code, "SPEC-001");
    }

    #[test]
    fn test_openspec_document_add_section() {
        let mut doc = OpenSpecDocument::new(
            "Test Spec".to_string(),
            SpecType::Design,
        );

        let section = OpenSpecSection {
            id: "intro".to_string(),
            title: "Introduction".to_string(),
            level: 1,
            content: "This is the introduction".to_string(),
            subsections: Vec::new(),
            metadata: HashMap::new(),
            required: true,
            review_status: None,
        };

        doc.add_section(section);
        assert_eq!(doc.sections.len(), 1);
        assert_eq!(doc.sections[0].id, "intro");
    }

    #[test]
    fn test_openspec_document_toc_generation() {
        let mut doc = OpenSpecDocument::new(
            "Test Spec".to_string(),
            SpecType::Design,
        );

        let section = OpenSpecSection {
            id: "intro".to_string(),
            title: "Introduction".to_string(),
            level: 1,
            content: "Content".to_string(),
            subsections: vec![
                OpenSpecSection {
                    id: "overview".to_string(),
                    title: "Overview".to_string(),
                    level: 2,
                    content: "Overview content".to_string(),
                    subsections: Vec::new(),
                    metadata: HashMap::new(),
                    required: false,
                    review_status: None,
                },
            ],
            metadata: HashMap::new(),
            required: true,
            review_status: None,
        };

        doc.add_section(section);
        let toc = doc.generate_toc();
        
        assert!(toc.contains("Introduction"));
        assert!(toc.contains("Overview"));
        assert!(toc.contains("#intro"));
        assert!(toc.contains("#overview"));
    }

    #[test]
    fn test_parsed_spec_creation() {
        let spec = ParsedSpec::new(
            PathBuf::from("/test/spec.md"),
            SpecFormat::Markdown,
        );

        assert_eq!(spec.source_path, PathBuf::from("/test/spec.md"));
        assert_eq!(spec.format, SpecFormat::Markdown);
        assert!(spec.is_successful());
        assert!(spec.warnings.is_empty());
        assert!(spec.errors.is_empty());
    }

    #[test]
    fn test_parsed_spec_with_errors() {
        let mut spec = ParsedSpec::new(
            PathBuf::from("/test/spec.md"),
            SpecFormat::Markdown,
        );

        spec.add_error("Parse error".to_string());
        spec.add_warning("Missing metadata".to_string());

        assert!(!spec.is_successful());
        assert_eq!(spec.errors.len(), 1);
        assert_eq!(spec.warnings.len(), 1);
    }

    #[test]
    fn test_serialization() {
        let doc = OpenSpecDocument::new(
            "Test Spec".to_string(),
            SpecType::Design,
        );

        let json = serde_json::to_string(&doc).unwrap();
        let deserialized: OpenSpecDocument = serde_json::from_str(&json).unwrap();
        
        assert_eq!(doc.metadata.title, deserialized.metadata.title);
        assert_eq!(doc.metadata.spec_type, deserialized.metadata.spec_type);
    }

    #[test]
    fn test_template_variable() {
        let var = TemplateVariable {
            name: "title".to_string(),
            var_type: TemplateVariableType::String,
            description: "Document title".to_string(),
            default_value: Some(serde_json::Value::String("Default Title".to_string())),
            required: true,
            validation: None,
        };

        assert_eq!(var.name, "title");
        assert_eq!(var.var_type, TemplateVariableType::String);
        assert!(var.required);
        assert!(var.default_value.is_some());
    }
}