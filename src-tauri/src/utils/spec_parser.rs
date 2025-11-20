/// Specification parser module for PACS
///
/// This module provides multi-format specification parsing capabilities,
/// supporting Markdown with frontmatter, YAML, JSON, TOML, and OpenSpec formats.

use crate::models::{
    SpecFormat, SpecType, SpecMetadata, ParsedSpec, ParsedContent,
    OpenSpecDocument, OpenSpecMetadata, OpenSpecStatus, SpecConversion
};
use crate::error::{ScannerError, ScannerResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use log::{debug, info, warn, error};

/// Detect specification format from file content and extension
pub fn detect_spec_format(content: &str, path: &Path) -> SpecFormat {
    // Check file extension first
    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        match extension.to_lowercase().as_str() {
            "md" => return SpecFormat::Markdown,
            "yaml" | "yml" => return SpecFormat::Yaml,
            "json" => return SpecFormat::Json,
            "toml" => return SpecFormat::Toml,
            _ => {}
        }
    }

    // Check content for OpenSpec indicators
    if content.contains("---") && content.contains("id:") && content.contains("title:") {
        return SpecFormat::OpenSpec;
    }

    // Check for YAML frontmatter in Markdown
    if content.trim_starts_with("---") && content.contains("---") {
        let lines: Vec<&str> = content.lines().collect();
        if lines.len() > 1 {
            let frontmatter_end = lines.iter()
                .skip(1)
                .position(|line| line.trim_starts_with("---"));
            
            if frontmatter_end.is_some() {
                return SpecFormat::Markdown;
            }
        }
    }

    // Try to parse as JSON
    if content.trim_starts_with("{") || content.trim_starts_with("[") {
        return SpecFormat::Json;
    }

    // Try to parse as YAML
    if !content.trim_starts_with("<") && (content.contains(":") || content.contains("-")) {
        return SpecFormat::Yaml;
    }

    // Try to parse as TOML
    if content.contains("=") && !content.contains("{") && !content.contains("[") {
        return SpecFormat::Toml;
    }

    SpecFormat::Unknown
}

/// Parse specification from Markdown file with YAML frontmatter
pub fn parse_spec_from_markdown(path: &Path) -> ScannerResult<ParsedSpec> {
    info!("Parsing Markdown specification: {}", path.display());
    
    let content = fs::read_to_string(path)
        .map_err(|e| ScannerError::FileAccess {
            path: path.display().to_string(),
            source: e,
        })?;

    let mut spec = ParsedSpec::new(path.to_path_buf(), SpecFormat::Markdown);
    
    // Split frontmatter and content
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 || !lines[0].trim_starts_with("---") {
        spec.add_warning("No YAML frontmatter found".to_string());
        spec.content = ParsedContent::PlainText(content);
        return Ok(spec);
    }

    // Find end of frontmatter
    let frontmatter_end = lines.iter()
        .skip(1)
        .position(|line| line.trim_starts_with("---"))
        .unwrap_or(lines.len());

    if frontmatter_end == 0 {
        spec.add_warning("Invalid YAML frontmatter format".to_string());
        spec.content = ParsedContent::PlainText(content);
        return Ok(spec);
    }

    // Parse frontmatter
    let frontmatter_lines = &lines[1..frontmatter_end];
    let frontmatter_text = frontmatter_lines.join("\n");
    
    let frontmatter: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&frontmatter_text)
        .map_err(|e| ScannerError::Serialization(e))?;

    // Extract metadata
    spec.metadata.title = frontmatter.get("title")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    
    spec.metadata.version = frontmatter.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    spec.metadata.status = frontmatter.get("status")
        .and_then(|v| v.as_str())
        .and_then(|s| parse_status_string(s))
        .unwrap_or(OpenSpecStatus::Draft);
    
    spec.metadata.author = frontmatter.get("author")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    if let Some(authors) = frontmatter.get("authors") {
        if let Some(authors_array) = authors.as_sequence() {
            spec.metadata.authors = authors_array.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
        }
    }

    // Extract tags
    if let Some(tags) = frontmatter.get("tags") {
        if let Some(tags_array) = tags.as_sequence() {
            spec.metadata.tags = tags_array.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
        }
    }

    // Extract creation/modification dates
    if let Some(date_str) = frontmatter.get("date").and_then(|v| v.as_str()) {
        spec.metadata.created = parse_date_string(date_str);
    }

    if let Some(modified_str) = frontmatter.get("modified").and_then(|v| v.as_str()) {
        spec.metadata.modified = parse_date_string(modified_str);
    }

    // Determine spec type from frontmatter or content
    spec.spec_type = frontmatter.get("type")
        .and_then(|v| v.as_str())
        .and_then(|s| parse_spec_type_string(s))
        .unwrap_or_else(|| infer_spec_type_from_content(&content));

    // Get content after frontmatter
    let content_lines = if frontmatter_end < lines.len() {
        &lines[frontmatter_end + 1..]
    } else {
        &[]
    };
    
    let content_text = content_lines.join("\n");
    spec.content = ParsedContent::Markdown {
        frontmatter,
        content: content_text,
    };

    // Set file metadata
    let metadata = fs::metadata(path)
        .map_err(|e| ScannerError::FileAccess {
            path: path.display().to_string(),
            source: e,
        })?;

    spec.metadata.size_bytes = metadata.len();
    spec.metadata.modified = Some(
        metadata.modified()
            .map_err(|e| ScannerError::FileAccess {
                path: path.display().to_string(),
                source: e,
            })?
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .and_then(|dt| DateTime::from_timestamp(dt.seconds() as i64))
            .unwrap_or_else(Utc::now)
    );

    Ok(spec)
}

/// Parse specification from JSON file
pub fn parse_spec_from_json(path: &Path) -> ScannerResult<ParsedSpec> {
    info!("Parsing JSON specification: {}", path.display());
    
    let content = fs::read_to_string(path)
        .map_err(|e| ScannerError::FileAccess {
            path: path.display().to_string(),
            source: e,
        })?;

    let json_value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| ScannerError::Serialization(e))?;

    let mut spec = ParsedSpec::new(path.to_path_buf(), SpecFormat::Json);
    
    // Extract metadata from JSON
    if let Some(obj) = json_value.as_object() {
        spec.metadata.title = obj.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        
        spec.metadata.version = obj.get("version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        spec.metadata.status = obj.get("status")
            .and_then(|v| v.as_str())
            .and_then(|s| parse_status_string(s))
            .unwrap_or(OpenSpecStatus::Draft);
        
        spec.metadata.author = obj.get("author")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        if let Some(authors) = obj.get("authors") {
            if let Some(authors_array) = authors.as_array() {
                spec.metadata.authors = authors_array.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        // Extract tags
        if let Some(tags) = obj.get("tags") {
            if let Some(tags_array) = tags.as_array() {
                spec.metadata.tags = tags_array.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        spec.spec_type = obj.get("type")
            .and_then(|v| v.as_str())
            .and_then(|s| parse_spec_type_string(s))
            .unwrap_or_else(|| infer_spec_type_from_content(&content));
    }

    spec.content = ParsedContent::Json(json_value);

    // Set file metadata
    let metadata = fs::metadata(path)
        .map_err(|e| ScannerError::FileAccess {
            path: path.display().to_string(),
            source: e,
        })?;

    spec.metadata.size_bytes = metadata.len();
    spec.metadata.modified = Some(
        metadata.modified()
            .map_err(|e| ScannerError::FileAccess {
                path: path.display().to_string(),
                source: e,
            })?
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .and_then(|dt| DateTime::from_timestamp(dt.seconds() as i64))
            .unwrap_or_else(Utc::now)
    );

    Ok(spec)
}

/// Parse specification from YAML file
pub fn parse_spec_from_yaml(path: &Path) -> ScannerResult<ParsedSpec> {
    info!("Parsing YAML specification: {}", path.display());
    
    let content = fs::read_to_string(path)
        .map_err(|e| ScannerError::FileAccess {
            path: path.display().to_string(),
            source: e,
        })?;

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
        .map_err(|e| ScannerError::Serialization(e))?;

    let mut spec = ParsedSpec::new(path.to_path_buf(), SpecFormat::Yaml);
    
    // Extract metadata from YAML
    if let Some(mapping) = yaml_value.as_mapping() {
        spec.metadata.title = mapping.get(&serde_yaml::Value::String("title".to_string()))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        
        spec.metadata.version = mapping.get(&serde_yaml::Value::String("version".to_string()))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        spec.metadata.status = mapping.get(&serde_yaml::Value::String("status".to_string()))
            .and_then(|v| v.as_str())
            .and_then(|s| parse_status_string(s))
            .unwrap_or(OpenSpecStatus::Draft);
        
        spec.metadata.author = mapping.get(&serde_yaml::Value::String("author".to_string()))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        if let Some(authors) = mapping.get(&serde_yaml::Value::String("authors".to_string())) {
            if let Some(authors_array) = authors.as_sequence() {
                spec.metadata.authors = authors_array.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        // Extract tags
        if let Some(tags) = mapping.get(&serde_yaml::Value::String("tags".to_string())) {
            if let Some(tags_array) = tags.as_sequence() {
                spec.metadata.tags = tags_array.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        spec.spec_type = mapping.get(&serde_yaml::Value::String("type".to_string()))
            .and_then(|v| v.as_str())
            .and_then(|s| parse_spec_type_string(s))
            .unwrap_or_else(|| infer_spec_type_from_content(&content));
    }

    spec.content = ParsedContent::Yaml(yaml_value);

    // Set file metadata
    let metadata = fs::metadata(path)
        .map_err(|e| ScannerError::FileAccess {
            path: path.display().to_string(),
            source: e,
        })?;

    spec.metadata.size_bytes = metadata.len();
    spec.metadata.modified = Some(
        metadata.modified()
            .map_err(|e| ScannerError::FileAccess {
                path: path.display().to_string(),
                source: e,
            })?
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .and_then(|dt| DateTime::from_timestamp(dt.seconds() as i64))
            .unwrap_or_else(Utc::now)
    );

    Ok(spec)
}

/// Parse specification from TOML file
pub fn parse_spec_from_toml(path: &Path) -> ScannerResult<ParsedSpec> {
    info!("Parsing TOML specification: {}", path.display());
    
    let content = fs::read_to_string(path)
        .map_err(|e| ScannerError::FileAccess {
            path: path.display().to_string(),
            source: e,
        })?;

    let toml_value: toml::Value = toml::from_str(&content)
        .map_err(|e| ScannerError::Serialization(e))?;

    let mut spec = ParsedSpec::new(path.to_path_buf(), SpecFormat::Toml);
    
    // Extract metadata from TOML
    if let Some(table) = toml_value.as_table() {
        spec.metadata.title = table.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        
        spec.metadata.version = table.get("version")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        spec.metadata.status = table.get("status")
            .and_then(|v| v.as_str())
            .and_then(|s| parse_status_string(s))
            .unwrap_or(OpenSpecStatus::Draft);
        
        spec.metadata.author = table.get("author")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // Handle authors array
        if let Some(authors) = table.get("authors") {
            if let Some(authors_array) = authors.as_array() {
                spec.metadata.authors = authors_array.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        // Extract tags
        if let Some(tags) = table.get("tags") {
            if let Some(tags_array) = tags.as_array() {
                spec.metadata.tags = tags_array.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        spec.spec_type = table.get("type")
            .and_then(|v| v.as_str())
            .and_then(|s| parse_spec_type_string(s))
            .unwrap_or_else(|| infer_spec_type_from_content(&content));
    }

    spec.content = ParsedContent::Toml(toml_value);

    // Set file metadata
    let metadata = fs::metadata(path)
        .map_err(|e| ScannerError::FileAccess {
            path: path.display().to_string(),
            source: e,
        })?;

    spec.metadata.size_bytes = metadata.len();
    spec.metadata.modified = Some(
        metadata.modified()
            .map_err(|e| ScannerError::FileAccess {
                path: path.display().to_string(),
                source: e,
            })?
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .and_then(|dt| DateTime::from_timestamp(dt.seconds() as i64))
            .unwrap_or_else(Utc::now)
    );

    Ok(spec)
}

/// Convert parsed specification to OpenSpec format
pub fn convert_to_openspec(spec: &ParsedSpec) -> SpecConversion {
    info!("Converting specification to OpenSpec format: {}", spec.metadata.path.display());
    
    let mut openspec_doc = OpenSpecDocument::new(
        spec.metadata.title.clone(),
        spec.spec_type.clone()
    );

    // Copy metadata
    openspec_doc.metadata.version = spec.metadata.version.clone().unwrap_or_else(|| "1.0.0".to_string());
    openspec_doc.metadata.status = spec.metadata.status.clone();
    openspec_doc.metadata.authors = spec.metadata.authors.clone();
    openspec_doc.metadata.tags = spec.metadata.tags.clone();
    openspec_doc.metadata.created = spec.metadata.created.unwrap_or_else(Utc::now);
    openspec_doc.metadata.modified = spec.metadata.modified.unwrap_or_else(Utc::now);

    // Convert content to OpenSpec sections
    match &spec.content {
        ParsedContent::Markdown { frontmatter, content } => {
            // Add title section if not present
            if openspec_doc.sections.iter().find(|s| s.id == "title").is_none() {
                openspec_doc.add_section(create_title_section(&spec.metadata.title));
            }
            
            // Add content sections
            let sections = parse_markdown_to_sections(content);
            for section in sections {
                openspec_doc.add_section(section);
            }
        },
        ParsedContent::Yaml(value) => {
            convert_yaml_to_openspec_sections(&mut openspec_doc, value);
        },
        ParsedContent::Json(value) => {
            convert_json_to_openspec_sections(&mut openspec_doc, value);
        },
        ParsedContent::Toml(value) => {
            convert_toml_to_openspec_sections(&mut openspec_doc, value);
        },
        ParsedContent::PlainText(text) => {
            // Add as a single content section
            openspec_doc.add_section(create_content_section("content", "Content", text));
        },
        ParsedContent::OpenSpec(doc) => {
            // Already OpenSpec, just copy
            openspec_doc = doc.clone();
        },
    }

    let mut conversion = SpecConversion {
        original: spec.clone(),
        converted: openspec_doc,
        quality_score: calculate_conversion_quality(spec),
        warnings: Vec::new(),
        errors: Vec::new(),
        manual_review_items: Vec::new(),
        converted_at: Utc::now(),
    };

    // Add warnings for missing metadata
    if spec.metadata.title.is_empty() {
        conversion.warnings.push("Missing title in original specification".to_string());
    }
    
    if spec.metadata.authors.is_empty() {
        conversion.warnings.push("Missing authors in original specification".to_string());
    }

    conversion
}

/// Extract metadata from documentation files in a project
pub fn extract_metadata_from_docs(project_path: &Path) -> ScannerResult<Vec<SpecMetadata>> {
    info!("Extracting metadata from documentation in: {}", project_path.display());
    
    let docs_path = project_path.join("docs");
    if !docs_path.exists() {
        return Ok(Vec::new());
    }

    let mut metadata_list = Vec::new();
    
    // Recursively scan for specification files
    for entry in walkdir::WalkDir::new(&docs_path) {
        .map_err(|e| ScannerError::Io(e))?
        .into_iter()
        .filter_map(|e| e.ok()) {
            let path = entry.path();
            
            // Only process markdown, yaml, json, toml files
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                match extension.to_lowercase().as_str() {
                    "md" | "yaml" | "yml" | "json" | "toml" => {
                        // Try to extract metadata quickly without full parsing
                        if let Ok(metadata) = quick_extract_metadata(&path) {
                            metadata_list.push(metadata);
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(metadata_list)
}

// Helper functions

fn parse_status_string(status: &str) -> Option<OpenSpecStatus> {
    match status.to_lowercase().as_str() {
        "draft" => Some(OpenSpecStatus::Draft),
        "review" => Some(OpenSpecStatus::Review),
        "approved" => Some(OpenSpecStatus::Approved),
        "deprecated" => Some(OpenSpecStatus::Deprecated),
        "archived" => Some(OpenSpecStatus::Archived),
        _ => None,
    }
}

fn parse_spec_type_string(spec_type: &str) -> Option<SpecType> {
    match spec_type.to_lowercase().as_str() {
        "design" => Some(SpecType::Design),
        "api" => Some(SpecType::Api),
        "architecture" => Some(SpecType::Architecture),
        "deployment" => Some(SpecType::Deployment),
        "testing" => Some(SpecType::Testing),
        "compliance" => Some(SpecType::Compliance),
        "schedule" => Some(SpecType::Schedule),
        "documentation" => Some(SpecType::Documentation),
        _ => None,
    }
}

fn infer_spec_type_from_content(content: &str) -> SpecType {
    let content_lower = content.to_lowercase();
    
    // Keywords for different spec types
    if content_lower.contains("api") || content_lower.contains("endpoint") || content_lower.contains("rest") {
        SpecType::Api
    } else if content_lower.contains("architecture") || content_lower.contains("system design") {
        SpecType::Architecture
    } else if content_lower.contains("deploy") || content_lower.contains("deployment") {
        SpecType::Deployment
    } else if content_lower.contains("test") || content_lower.contains("testing") {
        SpecType::Testing
    } else if content_lower.contains("compliance") || content_lower.contains("audit") {
        SpecType::Compliance
    } else if content_lower.contains("schedule") || content_lower.contains("timeline") {
        SpecType::Schedule
    } else {
        SpecType::Design // Default
    }
}

fn parse_date_string(date_str: &str) -> Option<DateTime<Utc>> {
    // Try ISO 8601 format first
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Some(dt);
    }
    
    // Try other common formats
    // This is a simplified implementation - in production, you'd want more robust parsing
    None
}

fn create_title_section(title: &str) -> crate::models::OpenSpecSection {
    crate::models::OpenSpecSection {
        id: "title".to_string(),
        title: "Title".to_string(),
        level: 1,
        content: format!("# {}", title),
        subsections: Vec::new(),
        metadata: HashMap::new(),
        required: true,
        review_status: None,
    }
}

fn create_content_section(id: &str, title: &str, content: &str) -> crate::models::OpenSpecSection {
    crate::models::OpenSpecSection {
        id: id.to_string(),
        title: title.to_string(),
        level: 2,
        content: content.to_string(),
        subsections: Vec::new(),
        metadata: HashMap::new(),
        required: false,
        review_status: None,
    }
}

fn parse_markdown_to_sections(content: &str) -> Vec<crate::models::OpenSpecSection> {
    let mut sections = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let mut current_section = String::new();
    let mut current_title = String::new();
    let mut section_level = 0;
    
    for line in lines {
        if line.starts_with('#') {
            // Save previous section if exists
            if !current_section.is_empty() {
                sections.push(create_content_section(
                    &format!("section_{}", sections.len()),
                    &current_title,
                    &current_section,
                ));
            }
            
            // Start new section
            section_level = line.chars().skip_while(|c| c == '#').count();
            current_title = line.trim_start_matches('#').trim().to_string();
            current_section.clear();
        } else {
            current_section.push_str(line);
            current_section.push('\n');
        }
    }
    
    // Add last section
    if !current_section.is_empty() {
        sections.push(create_content_section(
            &format!("section_{}", sections.len()),
            &current_title,
            &current_section,
        ));
    }
    
    sections
}

fn convert_yaml_to_openspec_sections(doc: &mut OpenSpecDocument, value: &serde_yaml::Value) {
    if let Some(mapping) = value.as_mapping() {
        for (key, value) in mapping {
            if let Some(key_str) = key.as_str() {
                let section_content = match value {
                    serde_yaml::Value::String(s) => s.clone(),
                    serde_yaml::Value::Sequence(seq) => {
                        seq.iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<_>>()
                            .join("\n")
                    },
                    _ => format!("{:?}", value),
                };
                
                doc.add_section(crate::models::OpenSpecSection {
                    id: key_str.to_string(),
                    title: key_str.to_string(),
                    level: 2,
                    content: section_content,
                    subsections: Vec::new(),
                    metadata: HashMap::new(),
                    required: false,
                    review_status: None,
                });
            }
        }
    }
}

fn convert_json_to_openspec_sections(doc: &mut OpenSpecDocument, value: &serde_json::Value) {
    if let Some(obj) = value.as_object() {
        for (key, value) in obj {
            let section_content = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Array(arr) => {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .collect::<Vec<_>>()
                        .join("\n")
                },
                serde_json::Value::Object(obj) => {
                    obj.iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join("\n")
                },
                _ => format!("{:?}", value),
            };
            
            doc.add_section(crate::models::OpenSpecSection {
                id: key.clone(),
                title: key.clone(),
                level: 2,
                content: section_content,
                subsections: Vec::new(),
                metadata: HashMap::new(),
                required: false,
                review_status: None,
            });
        }
    }
}

fn convert_toml_to_openspec_sections(doc: &mut OpenSpecDocument, value: &toml::Value) {
    if let Some(table) = value.as_table() {
        for (key, value) in table {
            let key_str = key.get();
            let section_content = match value {
                toml::Value::String(s) => s.clone(),
                toml::Value::Array(arr) => {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .collect::<Vec<_>>()
                        .join("\n")
                },
                toml::Value::Table(tbl) => {
                    tbl.iter()
                        .map(|(k, v)| format!("{} = {}", k, v))
                        .collect::<Vec<_>>()
                        .join("\n")
                },
                _ => format!("{:?}", value),
            };
            
            doc.add_section(crate::models::OpenSpecSection {
                id: key_str.to_string(),
                title: key_str.to_string(),
                level: 2,
                content: section_content,
                subsections: Vec::new(),
                metadata: HashMap::new(),
                required: false,
                review_status: None,
            });
        }
    }
}

fn calculate_conversion_quality(spec: &ParsedSpec) -> f32 {
    let mut score = 1.0;
    
    // Check for required metadata
    if spec.metadata.title.is_empty() {
        score -= 0.2;
    }
    
    if spec.metadata.authors.is_empty() {
        score -= 0.1;
    }
    
    if spec.metadata.version.is_none() {
        score -= 0.1;
    }
    
    // Check content quality
    match &spec.content {
        ParsedContent::Markdown { content, .. } => {
            if content.len() < 100 {
                score -= 0.1;
            }
        },
        ParsedContent::PlainText(text) => {
            if text.len() < 100 {
                score -= 0.2;
            }
        },
        _ => {}
    }
    
    score.max(0.0)
}

fn quick_extract_metadata(path: &Path) -> ScannerResult<SpecMetadata> {
    let mut metadata = SpecMetadata::new(path.to_path_buf());
    
    // Quick file metadata
    let file_metadata = fs::metadata(path)
        .map_err(|e| ScannerError::FileAccess {
            path: path.display().to_string(),
            source: e,
        })?;
    
    metadata.size_bytes = file_metadata.len();
    metadata.modified = Some(
        file_metadata.modified()
            .map_err(|e| ScannerError::FileAccess {
                path: path.display().to_string(),
                source: e,
            })?
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .and_then(|dt| DateTime::from_timestamp(dt.seconds() as i64))
            .unwrap_or_else(Utc::now)
    );

    // Try to extract title from filename
    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
        metadata.title = filename.to_string();
    }

    // Quick content scan for type detection
    if let Ok(content) = fs::read_to_string(path) {
        metadata.spec_type = infer_spec_type_from_content(&content);
    }

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_spec_format() {
        // Markdown with frontmatter
        let content = "---\ntitle: Test\n---\n# Content";
        let path = Path::new("test.md");
        assert_eq!(detect_spec_format(content, &path), SpecFormat::Markdown);

        // JSON
        let content = "{\"title\": \"Test\"}";
        let path = Path::new("test.json");
        assert_eq!(detect_spec_format(content, &path), SpecFormat::Json);

        // Unknown
        let content = "Just some text";
        let path = Path::new("test.txt");
        assert_eq!(detect_spec_format(content, &path), SpecFormat::Unknown);
    }

    #[test]
    fn test_parse_spec_from_markdown() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");
        let content = "---\ntitle: Test Spec\nversion: 1.0\nauthor: Test Author\ntags:\n  - test\n  - spec\n---\n\n# Introduction\n\nThis is a test.";
        
        fs::write(&file_path, content).unwrap();
        
        let result = parse_spec_from_markdown(&file_path).unwrap();
        assert_eq!(result.format, SpecFormat::Markdown);
        assert_eq!(result.metadata.title, "Test Spec");
        assert_eq!(result.metadata.version, Some("1.0".to_string()));
        assert_eq!(result.metadata.author, Some("Test Author".to_string()));
        assert_eq!(result.metadata.tags, vec!["test".to_string(), "spec".to_string()]);
        assert_eq!(result.spec_type, SpecType::Design); // Inferred from content
    }

    #[test]
    fn test_parse_spec_from_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let content = r#"{
            "title": "Test Spec",
            "version": "1.0",
            "type": "api",
            "authors": ["Test Author"]
        }"#;
        
        fs::write(&file_path, content).unwrap();
        
        let result = parse_spec_from_json(&file_path).unwrap();
        assert_eq!(result.format, SpecFormat::Json);
        assert_eq!(result.metadata.title, "Test Spec");
        assert_eq!(result.metadata.version, Some("1.0".to_string()));
        assert_eq!(result.metadata.authors, vec!["Test Author".to_string()]);
        assert_eq!(result.spec_type, SpecType::Api);
    }

    #[test]
    fn test_convert_to_openspec() {
        let mut spec = ParsedSpec::new(PathBuf::from("test.md"), SpecFormat::Markdown);
        spec.metadata.title = "Test".to_string();
        spec.metadata.authors = vec!["Author".to_string()];
        spec.content = ParsedContent::PlainText("Test content".to_string());
        
        let conversion = convert_to_openspec(&spec);
        assert_eq!(conversion.converted.metadata.title, "Test");
        assert_eq!(conversion.converted.metadata.authors, vec!["Author".to_string()]);
        assert!(!conversion.converted.sections.is_empty());
        assert!(conversion.quality_score > 0.5);
    }

    #[test]
    fn test_extract_metadata_from_docs() {
        let temp_dir = TempDir::new().unwrap();
        let docs_path = temp_dir.path().join("docs");
        fs::create_dir_all(&docs_path).unwrap();
        
        // Create test files
        let md_path = docs_path.join("spec.md");
        fs::write(&md_path, "# Test Spec\n\nThis is a test.").unwrap();
        
        let json_path = docs_path.join("api.json");
        fs::write(&json_path, "{\"title\": \"API Spec\"}").unwrap();
        
        let result = extract_metadata_from_docs(&temp_dir.path()).unwrap();
        assert_eq!(result.len(), 2);
        
        // Check that metadata was extracted
        let md_metadata = result.iter().find(|m| m.path == md_path).unwrap();
        assert_eq!(md_metadata.title, "spec");
        assert_eq!(md_metadata.spec_type, SpecType::Design);
        
        let json_metadata = result.iter().find(|m| m.path == json_path).unwrap();
        assert_eq!(json_metadata.title, "api.json");
        assert_eq!(json_metadata.spec_type, SpecType::Api);
    }

    #[test]
    fn test_infer_spec_type_from_content() {
        assert_eq!(infer_spec_type_from_content("REST API endpoints"), SpecType::Api);
        assert_eq!(infer_spec_type_from_content("System architecture"), SpecType::Architecture);
        assert_eq!(infer_spec_type_from_content("Deployment procedures"), SpecType::Deployment);
        assert_eq!(infer_spec_type_from_content("Test cases"), SpecType::Testing);
        assert_eq!(infer_spec_type_from_content("Compliance checklist"), SpecType::Compliance);
        assert_eq!(infer_spec_type_from_content("Project timeline"), SpecType::Schedule);
        assert_eq!(infer_spec_type_from_content("Random content"), SpecType::Design);
    }
}