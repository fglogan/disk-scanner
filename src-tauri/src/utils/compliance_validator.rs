/// Compliance validator module for PACS
///
/// This module provides validation for EDGS, LAIO, TES-2025, Bloom Playbook,
/// OpenSpec format, and project structure compliance.

use crate::models::{
    ComplianceViolation, ComplianceLevel, EdgsPhase, LaioMaturity,
    EdgsComplianceResult, LaioComplianceResult, TesComplianceResult,
    BloomComplianceResult, OpenspecComplianceResult, StructureComplianceResult
};
use crate::error::{ScannerError, ScannerResult};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use log::{debug, info, warn, error};
use serde_yaml;
use walkdir::WalkDir;

/// Validate EDGS compliance
pub fn validate_edgs(project_path: &Path) -> ScannerResult<Vec<ComplianceViolation>> {
    let mut violations = Vec::new();
    info!("Validating EDGS compliance for: {}", project_path.display());
    
    // Check for EDGS schedule document
    let edgs_schedule_path = project_path.join("docs").join("schedules").join("EDGS_SCHEDULE.md");
    if !edgs_schedule_path.exists() {
        violations.push(ComplianceViolation {
            violation_type: "EDGS_MISSING_SCHEDULE".to_string(),
            severity: ComplianceLevel::High,
            file_path: Some(edgs_schedule_path.to_string_lossy().to_string()),
            message: "Missing EDGS schedule document".to_string(),
            remediation: "Create docs/schedules/EDGS_SCHEDULE.md with event-driven scheduling".to_string(),
            code: "EDGS-001".to_string(),
        });
    }
    
    // Check for phase gates definition
    let gates_path = project_path.join("docs").join("PHASE_GATES.md");
    if !gates_path.exists() {
        violations.push(ComplianceViolation {
            violation_type: "EDGS_MISSING_GATES".to_string(),
            severity: ComplianceLevel::Medium,
            file_path: Some(gates_path.to_string_lossy().to_string()),
            message: "Missing phase gates definition".to_string(),
            remediation: "Define phase gates in docs/PHASE_GATES.md".to_string(),
            code: "EDGS-002".to_string(),
        });
    }
    
    // Check for proof of execution bundles
    let poe_dir = project_path.join(".edgs").join("poe");
    if !poe_dir.exists() {
        violations.push(ComplianceViolation {
            violation_type: "EDGS_MISSING_POE".to_string(),
            severity: ComplianceLevel::Medium,
            file_path: Some(poe_dir.to_string_lossy().to_string()),
            message: "Missing proof of execution directory".to_string(),
            remediation: "Create .edgs/poe/ directory for evidence bundles".to_string(),
            code: "EDGS-003".to_string(),
        });
    }
    
    // Check for event-driven scheduling (no time estimates)
    if let Ok(content) = fs::read_to_string(&edgs_schedule_path) {
        if content.contains("hour") || content.contains("day") || content.contains("week") || content.contains("month") {
            violations.push(ComplianceViolation {
                violation_type: "EDGS_TIME_ESTIMATES".to_string(),
                severity: ComplianceLevel::Critical,
                file_path: Some(edgs_schedule_path.to_string_lossy().to_string()),
                message: "Found time-based estimates in EDGS schedule".to_string(),
                remediation: "Replace all time estimates with event-driven cycles".to_string(),
                code: "EDGS-004".to_string(),
            });
        }
    }
    
    debug!("EDGS validation found {} violations", violations.len());
    Ok(violations)
}

/// Validate LAIO compliance
pub fn validate_laio(project_path: &Path) -> ScannerResult<Vec<ComplianceViolation>> {
    let mut violations = Vec::new();
    info!("Validating LAIO compliance for: {}", project_path.display());
    
    // Check for LAIO constitution
    let constitution_path = project_path.join(".laio").join("constitution.yaml");
    if !constitution_path.exists() {
        violations.push(ComplianceViolation {
            violation_type: "LAIO_MISSING_CONSTITUTION".to_string(),
            severity: ComplianceLevel::High,
            file_path: Some(constitution_path.to_string_lossy().to_string()),
            message: "Missing LAIO constitution file".to_string(),
            remediation: "Create .laio/constitution.yaml with domain and maturity definitions".to_string(),
            code: "LAIO-001".to_string(),
        });
    } else {
        // Parse and validate constitution
        if let Ok(content) = fs::read_to_string(&constitution_path) {
            match serde_yaml::from_str::<HashMap<String, serde_yaml::Value>>(&content) {
                Ok(constitution) => {
                    // Check for required fields
                    if !constitution.contains_key("domain") {
                        violations.push(ComplianceViolation {
                            violation_type: "LAIO_MISSING_DOMAIN".to_string(),
                            severity: ComplianceLevel::High,
                            file_path: Some(constitution_path.to_string_lossy().to_string()),
                            message: "LAIO constitution missing domain classification".to_string(),
                            remediation: "Add 'domain' field with value 1-9 to constitution.yaml".to_string(),
                            code: "LAIO-002".to_string(),
                        });
                    }
                    
                    if !constitution.contains_key("maturity") {
                        violations.push(ComplianceViolation {
                            violation_type: "LAIO_MISSING_MATURITY".to_string(),
                            severity: ComplianceLevel::High,
                            file_path: Some(constitution_path.to_string_lossy().to_string()),
                            message: "LAIO constitution missing maturity level".to_string(),
                            remediation: "Add 'maturity' field (L0-L5) to constitution.yaml".to_string(),
                            code: "LAIO-003".to_string(),
                        });
                    }
                }
                Err(e) => {
                    violations.push(ComplianceViolation {
                        violation_type: "LAIO_INVALID_CONSTITUTION".to_string(),
                        severity: ComplianceLevel::Critical,
                        file_path: Some(constitution_path.to_string_lossy().to_string()),
                        message: format!("Invalid LAIO constitution YAML: {}", e),
                        remediation: "Fix YAML syntax errors in constitution.yaml".to_string(),
                        code: "LAIO-004".to_string(),
                    });
                }
            }
        }
    }
    
    // Check for LAIO object models
    let models_dir = project_path.join(".laio").join("models");
    if !models_dir.exists() {
        violations.push(ComplianceViolation {
            violation_type: "LAIO_MISSING_MODELS".to_string(),
            severity: ComplianceLevel::Medium,
            file_path: Some(models_dir.to_string_lossy().to_string()),
            message: "Missing LAIO models directory".to_string(),
            remediation: "Create .laio/models/ with LAIO_Project, LAIO_Resource, LAIO_Event definitions".to_string(),
            code: "LAIO-005".to_string(),
        });
    }
    
    debug!("LAIO validation found {} violations", violations.len());
    Ok(violations)
}

/// Validate TES-2025 compliance
pub fn validate_tes2025(project_path: &Path) -> ScannerResult<Vec<ComplianceViolation>> {
    let mut violations = Vec::new();
    info!("Validating TES-2025 compliance for: {}", project_path.display());
    
    // Check for no time estimates in tasks
    for entry in WalkDir::new(project_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            
            // Check markdown files for time estimates
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(content) = fs::read_to_string(path) {
                    let lowercase = content.to_lowercase();
                    
                    // Look for time estimate patterns
                    if lowercase.contains("estimated time:") || 
                       lowercase.contains("duration:") ||
                       lowercase.contains("eta:") ||
                       (lowercase.contains("hours") && lowercase.contains("task")) ||
                       (lowercase.contains("days") && lowercase.contains("complete")) {
                        
                        violations.push(ComplianceViolation {
                            violation_type: "TES_TIME_ESTIMATES".to_string(),
                            severity: ComplianceLevel::Critical,
                            file_path: Some(path.to_string_lossy().to_string()),
                            message: "Found time-based estimates violating TES-2025".to_string(),
                            remediation: "Replace time estimates with event-driven cycles".to_string(),
                            code: "TES-001".to_string(),
                        });
                    }
                }
            }
        }
    }
    
    // Check for audit trail
    let audit_dir = project_path.join(".audit");
    if !audit_dir.exists() {
        violations.push(ComplianceViolation {
            violation_type: "TES_MISSING_AUDIT".to_string(),
            severity: ComplianceLevel::Medium,
            file_path: Some(audit_dir.to_string_lossy().to_string()),
            message: "Missing audit trail directory".to_string(),
            remediation: "Create .audit/ directory for audit trail records".to_string(),
            code: "TES-002".to_string(),
        });
    }
    
    // Check for event-driven task definitions
    let tasks_path = project_path.join("docs").join("tasks.md");
    if tasks_path.exists() {
        if let Ok(content) = fs::read_to_string(&tasks_path) {
            if !content.contains("event") && !content.contains("cycle") {
                violations.push(ComplianceViolation {
                    violation_type: "TES_MISSING_EVENTS".to_string(),
                    severity: ComplianceLevel::High,
                    file_path: Some(tasks_path.to_string_lossy().to_string()),
                    message: "Tasks not defined in event-driven terms".to_string(),
                    remediation: "Define tasks using event-driven cycles, not time".to_string(),
                    code: "TES-003".to_string(),
                });
            }
        }
    }
    
    debug!("TES-2025 validation found {} violations", violations.len());
    Ok(violations)
}

/// Validate Bloom Playbook compliance
pub fn validate_bloom(project_path: &Path) -> ScannerResult<Vec<ComplianceViolation>> {
    let mut violations = Vec::new();
    info!("Validating Bloom Playbook compliance for: {}", project_path.display());
    
    // Check for Knowledge phase documentation
    let knowledge_path = project_path.join("docs").join("knowledge");
    if !knowledge_path.exists() {
        violations.push(ComplianceViolation {
            violation_type: "BLOOM_MISSING_KNOWLEDGE".to_string(),
            severity: ComplianceLevel::Medium,
            file_path: Some(knowledge_path.to_string_lossy().to_string()),
            message: "Missing Bloom Knowledge phase documentation".to_string(),
            remediation: "Create docs/knowledge/ with foundational concepts and theory".to_string(),
            code: "BLOOM-001".to_string(),
        });
    }
    
    // Check for Application phase documentation
    let application_path = project_path.join("docs").join("application");
    if !application_path.exists() {
        violations.push(ComplianceViolation {
            violation_type: "BLOOM_MISSING_APPLICATION".to_string(),
            severity: ComplianceLevel::Medium,
            file_path: Some(application_path.to_string_lossy().to_string()),
            message: "Missing Bloom Application phase documentation".to_string(),
            remediation: "Create docs/application/ with practical implementation guides".to_string(),
            code: "BLOOM-002".to_string(),
        });
    }
    
    // Check for Evaluation phase documentation
    let evaluation_path = project_path.join("docs").join("evaluation");
    if !evaluation_path.exists() {
        violations.push(ComplianceViolation {
            violation_type: "BLOOM_MISSING_EVALUATION".to_string(),
            severity: ComplianceLevel::Medium,
            file_path: Some(evaluation_path.to_string_lossy().to_string()),
            message: "Missing Bloom Evaluation phase documentation".to_string(),
            remediation: "Create docs/evaluation/ with assessment criteria and results".to_string(),
            code: "BLOOM-003".to_string(),
        });
    }
    
    debug!("Bloom validation found {} violations", violations.len());
    Ok(violations)
}

/// Validate OpenSpec format compliance
pub fn validate_openspec(project_path: &Path) -> ScannerResult<Vec<ComplianceViolation>> {
    let mut violations = Vec::new();
    info!("Validating OpenSpec format compliance for: {}", project_path.display());
    
    // Check for OpenSpec directory
    let openspec_dir = project_path.join("openspec");
    if !openspec_dir.exists() {
        violations.push(ComplianceViolation {
            violation_type: "OPENSPEC_MISSING_DIR".to_string(),
            severity: ComplianceLevel::High,
            file_path: Some(openspec_dir.to_string_lossy().to_string()),
            message: "Missing OpenSpec directory".to_string(),
            remediation: "Create openspec/ directory with project.md and changes/".to_string(),
            code: "SPEC-001".to_string(),
        });
        return Ok(violations); // Can't check further without directory
    }
    
    // Check for project.md
    let project_spec = openspec_dir.join("project.md");
    if !project_spec.exists() {
        violations.push(ComplianceViolation {
            violation_type: "OPENSPEC_MISSING_PROJECT".to_string(),
            severity: ComplianceLevel::High,
            file_path: Some(project_spec.to_string_lossy().to_string()),
            message: "Missing OpenSpec project definition".to_string(),
            remediation: "Create openspec/project.md with project metadata".to_string(),
            code: "SPEC-002".to_string(),
        });
    }
    
    // Check for changes directory
    let changes_dir = openspec_dir.join("changes");
    if !changes_dir.exists() {
        violations.push(ComplianceViolation {
            violation_type: "OPENSPEC_MISSING_CHANGES".to_string(),
            severity: ComplianceLevel::Medium,
            file_path: Some(changes_dir.to_string_lossy().to_string()),
            message: "Missing OpenSpec changes directory".to_string(),
            remediation: "Create openspec/changes/ for tracking specifications".to_string(),
            code: "SPEC-003".to_string(),
        });
    }
    
    // Validate OpenSpec format in existing specs
    if changes_dir.exists() {
        for entry in WalkDir::new(&changes_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() && entry.path().file_name() == Some(std::ffi::OsStr::new("proposal.md")) {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    // Check for required OpenSpec fields
                    if !content.contains("## Problem") && !content.contains("## Solution") {
                        violations.push(ComplianceViolation {
                            violation_type: "OPENSPEC_INVALID_FORMAT".to_string(),
                            severity: ComplianceLevel::Medium,
                            file_path: Some(entry.path().to_string_lossy().to_string()),
                            message: "OpenSpec proposal missing required sections".to_string(),
                            remediation: "Add '## Problem' and '## Solution' sections".to_string(),
                            code: "SPEC-004".to_string(),
                        });
                    }
                }
            }
        }
    }
    
    debug!("OpenSpec validation found {} violations", violations.len());
    Ok(violations)
}

/// Validate project structure compliance
pub fn validate_structure(project_path: &Path) -> ScannerResult<Vec<ComplianceViolation>> {
    let mut violations = Vec::new();
    info!("Validating project structure compliance for: {}", project_path.display());
    
    // Check for proper docs organization
    let docs_dir = project_path.join("docs");
    if !docs_dir.exists() {
        violations.push(ComplianceViolation {
            violation_type: "STRUCTURE_MISSING_DOCS".to_string(),
            severity: ComplianceLevel::High,
            file_path: Some(docs_dir.to_string_lossy().to_string()),
            message: "Missing docs/ directory".to_string(),
            remediation: "Create docs/ directory for documentation".to_string(),
            code: "STRUCT-001".to_string(),
        });
    }
    
    // Check for rogue markdown files in root
    let mut rogue_count = 0;
    for entry in fs::read_dir(project_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Allow README, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT
                let allowed = ["README.md", "LICENSE.md", "CONTRIBUTING.md", "CODE_OF_CONDUCT.md", "CHANGELOG.md"];
                
                if name.ends_with(".md") && !allowed.contains(&name) {
                    rogue_count += 1;
                    violations.push(ComplianceViolation {
                        violation_type: "STRUCTURE_ROGUE_MARKDOWN".to_string(),
                        severity: ComplianceLevel::Low,
                        file_path: Some(path.to_string_lossy().to_string()),
                        message: format!("Markdown file '{}' should be in docs/ directory", name),
                        remediation: "Move documentation files to appropriate docs/ subdirectory".to_string(),
                        code: "STRUCT-002".to_string(),
                    });
                    
                    // Only report first 5 rogue files to avoid spam
                    if rogue_count >= 5 {
                        violations.push(ComplianceViolation {
                            violation_type: "STRUCTURE_MANY_ROGUE_FILES".to_string(),
                            severity: ComplianceLevel::Medium,
                            file_path: None,
                            message: format!("Found {} total rogue markdown files in root", rogue_count),
                            remediation: "Move all documentation to docs/ subdirectories".to_string(),
                            code: "STRUCT-003".to_string(),
                        });
                        break;
                    }
                }
            }
        }
    }
    
    // Check for required subdirectories
    let required_dirs = ["design", "architecture", "api", "guides"];
    for dir_name in &required_dirs {
        let dir_path = docs_dir.join(dir_name);
        if docs_dir.exists() && !dir_path.exists() {
            violations.push(ComplianceViolation {
                violation_type: format!("STRUCTURE_MISSING_{}", dir_name.to_uppercase()),
                severity: ComplianceLevel::Low,
                file_path: Some(dir_path.to_string_lossy().to_string()),
                message: format!("Missing recommended docs/{} directory", dir_name),
                remediation: format!("Create docs/{} for {} documentation", dir_name, dir_name),
                code: format!("STRUCT-{:03}", 10 + required_dirs.iter().position(|&d| d == *dir_name).unwrap()),
            });
        }
    }
    
    debug!("Structure validation found {} violations", violations.len());
    Ok(violations)
}

/// Calculate overall compliance score based on violations
pub fn calculate_compliance_score(violations: &[ComplianceViolation]) -> f32 {
    if violations.is_empty() {
        return 100.0;
    }
    
    // Weight violations by severity
    let mut penalty = 0.0;
    
    for violation in violations {
        match violation.severity {
            ComplianceLevel::Critical => penalty += 20.0,
            ComplianceLevel::High => penalty += 10.0,
            ComplianceLevel::Medium => penalty += 5.0,
            ComplianceLevel::Low => penalty += 2.0,
        }
    }
    
    // Cap penalty at 100
    if penalty > 100.0 {
        penalty = 100.0;
    }
    
    100.0 - penalty
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    
    #[test]
    fn test_validate_edgs_missing_schedule() {
        let temp_dir = TempDir::new().unwrap();
        let violations = validate_edgs(temp_dir.path()).unwrap();
        
        assert!(violations.iter().any(|v| v.code == "EDGS-001"));
    }
    
    #[test]
    fn test_validate_laio_missing_constitution() {
        let temp_dir = TempDir::new().unwrap();
        let violations = validate_laio(temp_dir.path()).unwrap();
        
        assert!(violations.iter().any(|v| v.code == "LAIO-001"));
    }
    
    #[test]
    fn test_validate_tes2025_clean_project() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create compliant structure
        fs::create_dir_all(temp_dir.path().join(".audit")).unwrap();
        
        let violations = validate_tes2025(temp_dir.path()).unwrap();
        
        // Should only have missing audit trail violation
        assert_eq!(violations.len(), 0);
    }
    
    #[test]
    fn test_calculate_compliance_score_perfect() {
        let violations = vec![];
        let score = calculate_compliance_score(&violations);
        
        assert_eq!(score, 100.0);
    }
    
    #[test]
    fn test_calculate_compliance_score_with_violations() {
        let violations = vec![
            ComplianceViolation {
                violation_type: "TEST".to_string(),
                severity: ComplianceLevel::Critical,
                file_path: None,
                message: "Test".to_string(),
                remediation: "Fix it".to_string(),
                code: "TEST-001".to_string(),
            },
            ComplianceViolation {
                violation_type: "TEST".to_string(),
                severity: ComplianceLevel::Medium,
                file_path: None,
                message: "Test".to_string(),
                remediation: "Fix it".to_string(),
                code: "TEST-002".to_string(),
            },
        ];
        
        let score = calculate_compliance_score(&violations);
        
        // Critical (20) + Medium (5) = 25 penalty
        assert_eq!(score, 75.0);
    }
}