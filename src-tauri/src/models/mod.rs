/// Models module for Disk Bloat Scanner and PACS
///
/// This module contains all data structures and models used throughout
/// the application for disk scanning, cleanup operations, project auditing,
/// compliance validation, and specification management.

pub mod core;
pub mod project_audit;
pub mod specification;
pub mod compliance;

// Re-export core types for disk scanning operations
pub use core::{
    DiskInfoResponse, SystemInfoResponse, ScanOpts, CleanupReq, CleanupResult,
    LargeFileEntry, BloatEntry, BloatCategory, DuplicateEntry, DuplicateSet,
    JunkFileEntry, JunkCategory, CacheEntry, CacheCategory, GitEntry,
    GitRepository, GitRepoStatus, BloatPattern, JunkPattern,
};

// Re-export PACS project audit types
pub use project_audit::{
    ComplianceStatus, ComplianceLevel, EdgsPhase, LaioClassification,
    SpecMetadata, SpecInventory, ComplianceViolation, AuditReport,
    GeneratedSpec, GeneratedBeadsIssue, AuditConfig, AuditSummary,
    SpecFormat, SpecType,
};

// Re-export PACS specification types
pub use specification::{
    OpenSpecDocument, OpenSpecMetadata, OpenSpecStatus, OpenSpecSection,
    OpenSpecRelationship, OpenSpecRelationshipType, OpenSpecApproval,
    OpenSpecApprovalStatus, OpenSpecApprovalRecord, OpenSpecApprovalDecision,
    OpenSpecApprovalRequirements, ParsedSpec, ParsedContent, SpecConversion,
    SpecValidation, ValidationError, ValidationErrorSeverity, ValidationWarning as SpecValidationWarning,
    SpecTemplate, TemplateSection, TemplateVariable, TemplateVariableType,
};

// Re-export PACS compliance validation types
pub use compliance::{
    ComplianceValidationResult, ValidationWarning as ComplianceWarning,
    EdgsComplianceResult, PhaseMilestone, LaioComplianceResult,
    TesComplianceResult, BloomComplianceResult, OpenspecComplianceResult,
    StructureComplianceResult, ComplianceMetrics, ComplianceValidationConfig,
    CustomValidationRule,
};