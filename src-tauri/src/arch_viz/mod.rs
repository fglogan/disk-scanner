// Architecture Visualization Module
// Advanced code analysis and diagram generation using Tree-sitter

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tree_sitter::{Parser, Tree, Node};
use chrono::{DateTime, Utc};

/// Architecture visualization engine for code analysis and diagram generation
pub struct ArchVizEngine {
    project_path: PathBuf,
    config: ArchVizConfig,
    parsers: HashMap<String, Parser>,
    analysis_cache: HashMap<String, FileAnalysis>,
}

/// Configuration for architecture visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchVizConfig {
    /// Languages to analyze
    pub languages: Vec<String>,
    /// Maximum analysis depth
    pub max_depth: usize,
    /// Include test files
    pub include_tests: bool,
    /// Diagram output format
    pub output_format: DiagramFormat,
    /// Analysis scope
    pub scope: AnalysisScope,
}

/// Supported diagram output formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DiagramFormat {
    Mermaid,
    Graphviz,
    PlantUML,
    SVG,
    PNG,
    PDF,
    HTML,
}

/// Types of diagrams that can be generated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DiagramType {
    ArchitectureOverview,
    DependencyGraph,
    ClassHierarchy,
    FunctionCallGraph,
    FileOrganization,
    ModuleInteraction,
}

/// Analysis scope configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisScope {
    /// Analyze entire project
    Full,
    /// Analyze specific directories
    Directories(Vec<String>),
    /// Analyze specific files
    Files(Vec<String>),
}

/// Complete architecture analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureAnalysis {
    pub project_path: String,
    pub analyzed_at: DateTime<Utc>,
    pub analyzer_version: String,
    pub file_count: usize,
    pub language_breakdown: HashMap<String, usize>,
    pub modules: Vec<ModuleInfo>,
    pub dependencies: Vec<DependencyRelation>,
    pub diagrams: HashMap<DiagramFormat, String>,
    pub metrics: ArchitectureMetrics,
}

/// Information about a code module/file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub path: String,
    pub language: String,
    pub size_bytes: u64,
    pub line_count: usize,
    pub functions: Vec<FunctionInfo>,
    pub classes: Vec<ClassInfo>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub complexity_score: f64,
}

/// Function analysis information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub calls: Vec<String>,
    pub complexity: usize,
    pub is_public: bool,
}

/// Class analysis information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassInfo {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub methods: Vec<FunctionInfo>,
    pub properties: Vec<String>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub is_public: bool,
}

/// Dependency relationship between modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyRelation {
    pub from_module: String,
    pub to_module: String,
    pub dependency_type: DependencyType,
    pub strength: f64, // 0.0 to 1.0
}

/// Types of dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Import,
    FunctionCall,
    ClassInheritance,
    InterfaceImplementation,
    DataFlow,
}

/// Architecture metrics and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureMetrics {
    pub total_lines: usize,
    pub total_functions: usize,
    pub total_classes: usize,
    pub average_complexity: f64,
    pub coupling_score: f64,
    pub cohesion_score: f64,
    pub maintainability_index: f64,
    pub technical_debt_ratio: f64,
}

/// Individual file analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub path: String,
    pub language: String,
    pub ast_tree: String, // Serialized AST representation
    pub symbols: Vec<Symbol>,
    pub analyzed_at: DateTime<Utc>,
}

/// Code symbol (function, class, variable, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub line_start: usize,
    pub line_end: usize,
    pub scope: String,
    pub visibility: Visibility,
}

/// Types of code symbols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolType {
    Function,
    Class,
    Interface,
    Variable,
    Constant,
    Type,
    Module,
    Namespace,
}

/// Symbol visibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

impl ArchVizEngine {
    /// Create new architecture visualization engine
    pub fn new(project_path: impl AsRef<Path>, config: ArchVizConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut parsers = HashMap::new();
        
        // Initialize Tree-sitter parsers for supported languages
        for language in &config.languages {
            let mut parser = Parser::new();
            let tree_sitter_lang = match language.as_str() {
                "rust" => tree_sitter_rust::language(),
                "javascript" => tree_sitter_javascript::language(),
                "typescript" => tree_sitter_typescript::language_typescript(),
                "python" => tree_sitter_python::language(),
                "json" => tree_sitter_json::language(),
                _ => continue, // Skip unsupported languages
            };
            
            parser.set_language(tree_sitter_lang)?;
            parsers.insert(language.clone(), parser);
        }
        
        Ok(Self {
            project_path: project_path.as_ref().to_path_buf(),
            config,
            parsers,
            analysis_cache: HashMap::new(),
        })
    }
    
    /// Perform complete architecture analysis
    pub async fn analyze(&mut self) -> Result<ArchitectureAnalysis, Box<dyn std::error::Error>> {
        log::info!("Starting architecture analysis for: {}", self.project_path.display());
        
        // Step 1: Discover and categorize files
        let files = self.discover_files().await?;
        log::info!("Discovered {} files for analysis", files.len());
        
        // Step 2: Parse files and build ASTs
        let mut modules = Vec::new();
        let mut language_breakdown = HashMap::new();
        
        for file_path in files {
            if let Ok(analysis) = self.analyze_file(&file_path).await {
                let module_info = self.extract_module_info(&analysis)?;
                
                // Update language breakdown
                *language_breakdown.entry(analysis.language.clone()).or_insert(0) += 1;
                
                modules.push(module_info);
                self.analysis_cache.insert(file_path.to_string_lossy().to_string(), analysis);
            }
        }
        
        // Step 3: Analyze dependencies and relationships
        let dependencies = self.analyze_dependencies(&modules)?;
        
        // Step 4: Calculate architecture metrics
        let metrics = self.calculate_metrics(&modules, &dependencies)?;
        
        // Step 5: Generate diagrams automatically
        let mut diagrams = HashMap::new();
        
        // Generate all diagram types in Mermaid format
        diagrams.insert(DiagramFormat::Mermaid, self.generate_architecture_overview(&modules, &dependencies)?);
        
        // Generate additional diagram formats
        if modules.len() < 50 { // Only for smaller projects to avoid complexity
            diagrams.insert(DiagramFormat::Graphviz, self.generate_graphviz_diagram(&modules, &dependencies)?);
            diagrams.insert(DiagramFormat::PlantUML, self.generate_plantuml_diagram(&modules, &dependencies)?);
        }
        
        // Generate specialized diagrams
        let dependency_diagram = self.generate_dependency_graph(&modules, &dependencies)?;
        let class_diagram = self.generate_class_hierarchy(&modules)?;
        let file_org_diagram = self.generate_file_organization(&modules)?;
        
        // Store specialized diagrams with prefixed keys
        diagrams.insert(DiagramFormat::Mermaid, format!("# Architecture Overview\n{}\n\n# Dependency Graph\n{}\n\n# Class Hierarchy\n{}\n\n# File Organization\n{}", 
            diagrams.get(&DiagramFormat::Mermaid).unwrap_or(&String::new()),
            dependency_diagram,
            class_diagram,
            file_org_diagram
        ));
        
        let analysis = ArchitectureAnalysis {
            project_path: self.project_path.to_string_lossy().to_string(),
            analyzed_at: Utc::now(),
            analyzer_version: "1.0.0".to_string(),
            file_count: modules.len(),
            language_breakdown,
            modules,
            dependencies,
            diagrams,
            metrics,
        };
        
        log::info!("Architecture analysis completed. {} modules analyzed", analysis.file_count);
        
        Ok(analysis)
    }
    
    /// Discover files to analyze based on configuration
    async fn discover_files(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        use walkdir::WalkDir;
        
        let mut files = Vec::new();
        
        let walker = WalkDir::new(&self.project_path)
            .max_depth(self.config.max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| entry.file_type().is_file());
        
        for entry in walker {
            let path = entry.path();
            
            // Skip hidden files and directories
            if path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with('.'))
                .unwrap_or(false)
            {
                continue;
            }
            
            // Skip common build/dependency directories
            let path_str = path.to_string_lossy();
            if path_str.contains("/node_modules/") ||
               path_str.contains("/target/") ||
               path_str.contains("/dist/") ||
               path_str.contains("/build/") ||
               path_str.contains("/.next/") ||
               path_str.contains("/.nuxt/") ||
               path_str.contains("/vendor/") ||
               path_str.contains("/__pycache__/") ||
               path_str.contains("/.venv/") ||
               path_str.contains("/venv/") ||
               path_str.contains("/.cargo/") ||
               path_str.contains("/coverage/") ||
               path_str.contains("/tmp/") ||
               path_str.contains("/temp/") ||
               // Skip generated schema directories but keep root configs
               (path_str.contains("/gen/schemas/") && path_str.ends_with(".json"))
            {
                continue;
            }
            
            // Check if file extension matches supported languages
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                let language = match extension {
                    "rs" => "rust",
                    "js" | "jsx" => "javascript",
                    "ts" | "tsx" => "typescript",
                    "py" => "python",
                    "json" => "json",
                    _ => continue,
                };
                
                if self.config.languages.contains(&language.to_string()) {
                    // Skip test files if configured
                    if !self.config.include_tests && self.is_test_file(path) {
                        continue;
                    }
                    
                    files.push(path.to_path_buf());
                }
            }
        }
        
        Ok(files)
    }
    
    /// Check if a file is a test file
    fn is_test_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_lowercase();
        path_str.contains("test") || 
        path_str.contains("spec") ||
        path_str.contains("__tests__") ||
        path_str.ends_with("_test.rs") ||
        path_str.ends_with(".test.js") ||
        path_str.ends_with(".spec.js")
    }
    
    /// Analyze a single file
    async fn analyze_file(&mut self, file_path: &Path) -> Result<FileAnalysis, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        
        // Determine language from file extension
        let language = match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => "rust",
            Some("js") | Some("jsx") => "javascript",
            Some("ts") | Some("tsx") => "typescript",
            Some("py") => "python",
            Some("json") => "json",
            _ => return Err("Unsupported file type".into()),
        };
        
        // Get parser for this language
        let parser = self.parsers.get_mut(language)
            .ok_or("Parser not available for language")?;
        
        // Parse the file
        let tree = parser.parse(&content, None)
            .ok_or("Failed to parse file")?;
        
        // Extract symbols from AST
        let symbols = self.extract_symbols(&tree, &content, language)?;
        
        Ok(FileAnalysis {
            path: file_path.to_string_lossy().to_string(),
            language: language.to_string(),
            ast_tree: format!("{:?}", tree.root_node()), // Simplified AST representation
            symbols,
            analyzed_at: Utc::now(),
        })
    }
    
    /// Extract symbols from AST
    fn extract_symbols(&self, tree: &Tree, content: &str, language: &str) -> Result<Vec<Symbol>, Box<dyn std::error::Error>> {
        let mut symbols = Vec::new();
        let root_node = tree.root_node();
        
        self.traverse_node(root_node, content, language, &mut symbols, "global")?;
        
        Ok(symbols)
    }
    
    /// Recursively traverse AST nodes to extract symbols
    fn traverse_node(
        &self,
        node: Node,
        content: &str,
        language: &str,
        symbols: &mut Vec<Symbol>,
        scope: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match language {
            "rust" => self.extract_rust_symbols(node, content, symbols, scope)?,
            "javascript" | "typescript" => self.extract_js_symbols(node, content, symbols, scope)?,
            "python" => self.extract_python_symbols(node, content, symbols, scope)?,
            _ => {} // Skip unsupported languages
        }
        
        // Recursively process child nodes
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.traverse_node(child, content, language, symbols, scope)?;
        }
        
        Ok(())
    }
    
    /// Extract Rust-specific symbols
    fn extract_rust_symbols(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<Symbol>,
        scope: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match node.kind() {
            "function_item" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = &content[name_node.byte_range()];
                    symbols.push(Symbol {
                        name: name.to_string(),
                        symbol_type: SymbolType::Function,
                        line_start: node.start_position().row + 1,
                        line_end: node.end_position().row + 1,
                        scope: scope.to_string(),
                        visibility: if node.to_sexp().contains("pub") { Visibility::Public } else { Visibility::Private },
                    });
                }
            }
            "struct_item" | "enum_item" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = &content[name_node.byte_range()];
                    symbols.push(Symbol {
                        name: name.to_string(),
                        symbol_type: SymbolType::Type,
                        line_start: node.start_position().row + 1,
                        line_end: node.end_position().row + 1,
                        scope: scope.to_string(),
                        visibility: if node.to_sexp().contains("pub") { Visibility::Public } else { Visibility::Private },
                    });
                }
            }
            "impl_item" => {
                if let Some(type_node) = node.child_by_field_name("type") {
                    let type_name = &content[type_node.byte_range()];
                    symbols.push(Symbol {
                        name: format!("impl {}", type_name),
                        symbol_type: SymbolType::Class,
                        line_start: node.start_position().row + 1,
                        line_end: node.end_position().row + 1,
                        scope: scope.to_string(),
                        visibility: Visibility::Public,
                    });
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Extract JavaScript/TypeScript symbols
    fn extract_js_symbols(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<Symbol>,
        scope: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match node.kind() {
            "function_declaration" | "arrow_function" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = &content[name_node.byte_range()];
                    symbols.push(Symbol {
                        name: name.to_string(),
                        symbol_type: SymbolType::Function,
                        line_start: node.start_position().row + 1,
                        line_end: node.end_position().row + 1,
                        scope: scope.to_string(),
                        visibility: Visibility::Public, // JavaScript doesn't have private by default
                    });
                }
            }
            "class_declaration" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = &content[name_node.byte_range()];
                    symbols.push(Symbol {
                        name: name.to_string(),
                        symbol_type: SymbolType::Class,
                        line_start: node.start_position().row + 1,
                        line_end: node.end_position().row + 1,
                        scope: scope.to_string(),
                        visibility: Visibility::Public,
                    });
                }
            }
            "variable_declaration" => {
                // Extract variable names
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        if let Some(name_node) = child.child_by_field_name("name") {
                            let name = &content[name_node.byte_range()];
                            symbols.push(Symbol {
                                name: name.to_string(),
                                symbol_type: SymbolType::Variable,
                                line_start: child.start_position().row + 1,
                                line_end: child.end_position().row + 1,
                                scope: scope.to_string(),
                                visibility: Visibility::Public,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Extract Python symbols
    fn extract_python_symbols(
        &self,
        node: Node,
        content: &str,
        symbols: &mut Vec<Symbol>,
        scope: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match node.kind() {
            "function_definition" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = &content[name_node.byte_range()];
                    symbols.push(Symbol {
                        name: name.to_string(),
                        symbol_type: SymbolType::Function,
                        line_start: node.start_position().row + 1,
                        line_end: node.end_position().row + 1,
                        scope: scope.to_string(),
                        visibility: if name.starts_with('_') { Visibility::Private } else { Visibility::Public },
                    });
                }
            }
            "class_definition" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = &content[name_node.byte_range()];
                    symbols.push(Symbol {
                        name: name.to_string(),
                        symbol_type: SymbolType::Class,
                        line_start: node.start_position().row + 1,
                        line_end: node.end_position().row + 1,
                        scope: scope.to_string(),
                        visibility: if name.starts_with('_') { Visibility::Private } else { Visibility::Public },
                    });
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Extract module information from file analysis
    fn extract_module_info(&self, analysis: &FileAnalysis) -> Result<ModuleInfo, Box<dyn std::error::Error>> {
        let path = Path::new(&analysis.path);
        let metadata = std::fs::metadata(path)?;
        let content = std::fs::read_to_string(path)?;
        
        let functions: Vec<FunctionInfo> = analysis.symbols
            .iter()
            .filter(|s| matches!(s.symbol_type, SymbolType::Function))
            .map(|s| FunctionInfo {
                name: s.name.clone(),
                line_start: s.line_start,
                line_end: s.line_end,
                parameters: Vec::new(), // TODO: Extract parameters from AST
                return_type: None, // TODO: Extract return type
                calls: Vec::new(), // TODO: Extract function calls
                complexity: 1, // TODO: Calculate cyclomatic complexity
                is_public: matches!(s.visibility, Visibility::Public),
            })
            .collect();
        
        let classes: Vec<ClassInfo> = analysis.symbols
            .iter()
            .filter(|s| matches!(s.symbol_type, SymbolType::Class))
            .map(|s| ClassInfo {
                name: s.name.clone(),
                line_start: s.line_start,
                line_end: s.line_end,
                methods: Vec::new(), // TODO: Extract methods
                properties: Vec::new(), // TODO: Extract properties
                extends: None, // TODO: Extract inheritance
                implements: Vec::new(), // TODO: Extract interfaces
                is_public: matches!(s.visibility, Visibility::Public),
            })
            .collect();
        
        Ok(ModuleInfo {
            path: analysis.path.clone(),
            language: analysis.language.clone(),
            size_bytes: metadata.len(),
            line_count: content.lines().count(),
            functions,
            classes,
            imports: Vec::new(), // TODO: Extract imports
            exports: Vec::new(), // TODO: Extract exports
            complexity_score: 1.0, // TODO: Calculate module complexity
        })
    }
    
    /// Analyze dependencies between modules
    fn analyze_dependencies(&self, _modules: &[ModuleInfo]) -> Result<Vec<DependencyRelation>, Box<dyn std::error::Error>> {
        let dependencies = Vec::new();
        
        // TODO: Implement dependency analysis
        // This would involve:
        // 1. Analyzing import/export statements
        // 2. Tracking function calls between modules
        // 3. Identifying inheritance relationships
        // 4. Calculating dependency strength
        
        Ok(dependencies)
    }
    
    /// Calculate architecture metrics
    fn calculate_metrics(&self, modules: &[ModuleInfo], dependencies: &[DependencyRelation]) -> Result<ArchitectureMetrics, Box<dyn std::error::Error>> {
        let total_lines: usize = modules.iter().map(|m| m.line_count).sum();
        let total_functions: usize = modules.iter().map(|m| m.functions.len()).sum();
        let total_classes: usize = modules.iter().map(|m| m.classes.len()).sum();
        
        let average_complexity = if !modules.is_empty() {
            modules.iter().map(|m| m.complexity_score).sum::<f64>() / modules.len() as f64
        } else {
            0.0
        };
        
        // TODO: Implement more sophisticated metrics
        let coupling_score = dependencies.len() as f64 / modules.len().max(1) as f64;
        let cohesion_score = 0.8; // Placeholder
        let maintainability_index = 100.0 - (coupling_score * 10.0); // Simplified calculation
        let technical_debt_ratio = 0.1; // Placeholder
        
        Ok(ArchitectureMetrics {
            total_lines,
            total_functions,
            total_classes,
            average_complexity,
            coupling_score,
            cohesion_score,
            maintainability_index,
            technical_debt_ratio,
        })
    }
    
    /// Generate architecture overview diagram
    pub fn generate_architecture_overview(&self, modules: &[ModuleInfo], dependencies: &[DependencyRelation]) -> Result<String, Box<dyn std::error::Error>> {
        let mut diagram = String::new();
        
        // Helper function to sanitize names for Mermaid
        let sanitize_name = |name: &str| -> String {
            name.chars()
                .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                .collect::<String>()
                .trim_matches('_')
                .to_string()
        };
        
        // Start with graph definition
        diagram.push_str("graph TD\n");
        diagram.push_str("    %% Architecture Overview - High-level module relationships\n\n");
        
        // Limit modules to prevent overwhelming diagrams
        let limited_modules: Vec<_> = modules.iter().take(20).collect();
        
        // Add nodes for each module
        for (i, module) in limited_modules.iter().enumerate() {
            let module_name = std::path::Path::new(&module.path)
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");
            
            let clean_name = sanitize_name(module_name);
            let safe_name = if clean_name.is_empty() { format!("Module{}", i) } else { clean_name };
            
            diagram.push_str(&format!(
                "    M{}[\"{}\\n{} lines\"]\n",
                i, safe_name, module.line_count
            ));
        }
        
        diagram.push_str("\n");
        
        // Add edges for dependencies (limit to prevent clutter)
        let mut edge_count = 0;
        for dependency in dependencies.iter().take(30) {
            if edge_count >= 15 { break; } // Limit edges
            
            if let (Some(from_idx), Some(to_idx)) = (
                limited_modules.iter().position(|m| m.path == dependency.from_module),
                limited_modules.iter().position(|m| m.path == dependency.to_module),
            ) {
                if from_idx != to_idx { // Avoid self-references
                    diagram.push_str(&format!("    M{} --> M{}\n", from_idx, to_idx));
                    edge_count += 1;
                }
            }
        }
        
        // Add styling based on language
        diagram.push_str("\n    %% Styling by language\n");
        let mut used_languages = std::collections::HashSet::new();
        
        for (i, module) in limited_modules.iter().enumerate() {
            let lang = sanitize_name(&module.language);
            if !used_languages.contains(&lang) {
                let color = match module.language.as_str() {
                    "rust" => "#dea584",
                    "javascript" => "#f7df1e", 
                    "typescript" => "#3178c6",
                    "python" => "#3776ab",
                    "json" => "#292929",
                    _ => "#888888",
                };
                diagram.push_str(&format!(
                    "    classDef lang_{} fill:{},stroke:#333,stroke-width:2px\n",
                    lang, color
                ));
                used_languages.insert(lang.clone());
            }
            diagram.push_str(&format!("    class M{} lang_{}\n", i, lang));
        }
        
        Ok(diagram)
    }
    
    /// Generate detailed dependency graph
    pub fn generate_dependency_graph(&self, modules: &[ModuleInfo], dependencies: &[DependencyRelation]) -> Result<String, Box<dyn std::error::Error>> {
        let mut diagram = String::new();
        
        // Helper function to sanitize names for Mermaid
        let sanitize_name = |name: &str| -> String {
            name.chars()
                .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                .collect::<String>()
                .trim_matches('_')
                .to_string()
        };
        
        diagram.push_str("graph LR\n");
        diagram.push_str("    %% Dependency Graph - Shows import/export relationships\n\n");
        
        // Limit modules to prevent overwhelming diagrams
        let limited_modules: Vec<_> = modules.iter().take(15).collect();
        
        // Simple node approach instead of complex subgraphs
        for (i, module) in limited_modules.iter().enumerate() {
            let module_name = std::path::Path::new(&module.path)
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");
            
            let clean_name = sanitize_name(module_name);
            let safe_name = if clean_name.is_empty() { format!("Mod{}", i) } else { clean_name };
            
            diagram.push_str(&format!("    M{}[\"{}\"]\n", i, safe_name));
        }
        
        diagram.push_str("\n");
        
        // Add dependency arrows (limited)
        let mut edge_count = 0;
        for dependency in dependencies.iter().take(20) {
            if edge_count >= 10 { break; } // Limit edges
            
            if let (Some(from_idx), Some(to_idx)) = (
                limited_modules.iter().position(|m| m.path == dependency.from_module),
                limited_modules.iter().position(|m| m.path == dependency.to_module),
            ) {
                if from_idx != to_idx { // Avoid self-references
                    diagram.push_str(&format!("    M{} --> M{}\n", from_idx, to_idx));
                    edge_count += 1;
                }
            }
        }
        
        // If no dependencies found, add a note
        if edge_count == 0 {
            diagram.push_str("    NoDeps[\"No dependencies detected\\nThis might be a simple project\"]\n");
        }
        
        Ok(diagram)
    }
    
    /// Generate class hierarchy diagram
    pub fn generate_class_hierarchy(&self, modules: &[ModuleInfo]) -> Result<String, Box<dyn std::error::Error>> {
        let mut diagram = String::new();
        
        diagram.push_str("classDiagram\n");
        diagram.push_str("    %% Class Hierarchy - Shows inheritance and composition\n\n");
        
        // Helper function to sanitize names for Mermaid
        let sanitize_name = |name: &str| -> String {
            name.chars()
                .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                .collect::<String>()
                .trim_matches('_')
                .to_string()
        };
        
        let mut class_count = 0;
        
        for module in modules {
            for class in &module.classes {
                // Skip if no meaningful content
                if class.methods.is_empty() && class.properties.is_empty() {
                    continue;
                }
                
                class_count += 1;
                if class_count > 10 { // Limit to prevent overwhelming diagrams
                    break;
                }
                
                let class_name = sanitize_name(&class.name);
                if class_name.is_empty() {
                    continue;
                }
                
                // Add class definition
                diagram.push_str(&format!("    class {} {{\n", class_name));
                
                // Add properties (limit to first 5)
                for (i, property) in class.properties.iter().take(5).enumerate() {
                    let prop_name = sanitize_name(property);
                    if !prop_name.is_empty() {
                        diagram.push_str(&format!("        +{} : Property\n", prop_name));
                    }
                }
                
                // Add methods (limit to first 5)
                for (i, method) in class.methods.iter().take(5).enumerate() {
                    let method_name = sanitize_name(&method.name);
                    if !method_name.is_empty() {
                        let visibility = if method.is_public { "+" } else { "-" };
                        diagram.push_str(&format!("        {}{}()\n", visibility, method_name));
                    }
                }
                
                diagram.push_str("    }\n\n");
                
                // Add inheritance relationships (simplified)
                if let Some(parent) = &class.extends {
                    let parent_name = sanitize_name(parent);
                    if !parent_name.is_empty() && parent_name != class_name {
                        diagram.push_str(&format!("    {} <|-- {}\n", parent_name, class_name));
                    }
                }
                
                // Add interface implementations (limit to first 3)
                for interface in class.implements.iter().take(3) {
                    let interface_name = sanitize_name(interface);
                    if !interface_name.is_empty() && interface_name != class_name {
                        diagram.push_str(&format!("    {} <|.. {}\n", interface_name, class_name));
                    }
                }
            }
        }
        
        // If no classes found, create a simple message
        if class_count == 0 {
            diagram.push_str("    class NoClasses {\n");
            diagram.push_str("        +message : \"No classes found in this project\"\n");
            diagram.push_str("        +suggestion : \"This might be a functional or procedural codebase\"\n");
            diagram.push_str("    }\n");
        }
        
        Ok(diagram)
    }
    
    /// Generate file organization diagram
    pub fn generate_file_organization(&self, modules: &[ModuleInfo]) -> Result<String, Box<dyn std::error::Error>> {
        let mut diagram = String::new();
        
        diagram.push_str("graph TD\n");
        diagram.push_str("    %% File Organization - Shows directory structure\n");
        
        // Build directory tree
        let mut dirs: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        
        for module in modules {
            let path = std::path::Path::new(&module.path);
            let dir = path.parent()
                .and_then(|p| p.to_str())
                .unwrap_or("root")
                .to_string();
            let file = path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            dirs.entry(dir).or_default().push(file);
        }
        
        // Generate directory nodes
        for (dir, files) in dirs {
            let dir_id = dir.replace('/', "_").replace('.', "_");
            diagram.push_str(&format!("    {}[\"📁 {}\"]\n", dir_id, dir));
            
            for file in files {
                let file_id = format!("{}_{}", dir_id, file.replace('.', "_"));
                let icon = match file.split('.').last() {
                    Some("rs") => "🦀",
                    Some("js") | Some("jsx") => "📜",
                    Some("ts") | Some("tsx") => "📘",
                    Some("py") => "🐍",
                    Some("json") => "📋",
                    Some("md") => "📝",
                    _ => "📄",
                };
                diagram.push_str(&format!("    {}[\"{} {}\"]\n", file_id, icon, file));
                diagram.push_str(&format!("    {} --> {}\n", dir_id, file_id));
            }
        }
        
        Ok(diagram)
    }
    
    /// Generate Graphviz DOT format diagram
    pub fn generate_graphviz_diagram(&self, modules: &[ModuleInfo], dependencies: &[DependencyRelation]) -> Result<String, Box<dyn std::error::Error>> {
        let mut diagram = String::new();
        
        diagram.push_str("digraph Architecture {\n");
        diagram.push_str("    rankdir=TB;\n");
        diagram.push_str("    node [shape=box, style=rounded];\n");
        diagram.push_str("    edge [color=gray];\n\n");
        
        // Add nodes with language-based styling
        for (i, module) in modules.iter().enumerate() {
            let module_name = std::path::Path::new(&module.path)
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");
            
            let color = match module.language.as_str() {
                "rust" => "orange",
                "javascript" => "yellow",
                "typescript" => "blue",
                "python" => "green",
                _ => "gray",
            };
            
            diagram.push_str(&format!(
                "    M{} [label=\"{}\\n({} lines)\", fillcolor={}, style=\"filled,rounded\"];\n",
                i, module_name, module.line_count, color
            ));
        }
        
        // Add edges for dependencies
        for dependency in dependencies {
            if let (Some(from_idx), Some(to_idx)) = (
                modules.iter().position(|m| m.path == dependency.from_module),
                modules.iter().position(|m| m.path == dependency.to_module),
            ) {
                diagram.push_str(&format!("    M{} -> M{};\n", from_idx, to_idx));
            }
        }
        
        diagram.push_str("}\n");
        Ok(diagram)
    }
    
    /// Generate PlantUML diagram
    pub fn generate_plantuml_diagram(&self, modules: &[ModuleInfo], _dependencies: &[DependencyRelation]) -> Result<String, Box<dyn std::error::Error>> {
        let mut diagram = String::new();
        
        diagram.push_str("@startuml\n");
        diagram.push_str("!theme plain\n");
        diagram.push_str("title Architecture Overview\n\n");
        
        // Group by language
        let mut lang_groups: std::collections::HashMap<String, Vec<&ModuleInfo>> = std::collections::HashMap::new();
        for module in modules {
            lang_groups.entry(module.language.clone()).or_default().push(module);
        }
        
        // Create packages for each language
        for (language, lang_modules) in lang_groups {
            diagram.push_str(&format!("package \"{}\" {{\n", language.to_uppercase()));
            
            for module in lang_modules {
                let module_name = std::path::Path::new(&module.path)
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown");
                
                diagram.push_str(&format!("    [{}]\n", module_name));
            }
            
            diagram.push_str("}\n\n");
        }
        
        diagram.push_str("@enduml\n");
        Ok(diagram)
    }
}

impl Default for ArchVizConfig {
    fn default() -> Self {
        Self {
            languages: vec![
                "rust".to_string(),
                "javascript".to_string(),
                "typescript".to_string(),
                "python".to_string(),
                "json".to_string(),
            ],
            max_depth: 10,
            include_tests: false,
            output_format: DiagramFormat::Mermaid,
            scope: AnalysisScope::Full,
        }
    }
}