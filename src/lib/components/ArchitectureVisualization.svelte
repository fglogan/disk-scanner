<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { onMount } from 'svelte';
  import mermaid from 'mermaid';
  
  // Architecture Visualization types (matching Rust structs)
  interface ArchVizConfig {
    languages: string[];
    max_depth: number;
    include_tests: boolean;
    output_format: 'Mermaid' | 'SVG' | 'PNG' | 'PDF' | 'HTML';
    scope: 'Full' | { Directories: string[] } | { Files: string[] };
  }
  
  interface ModuleInfo {
    path: string;
    language: string;
    size_bytes: number;
    line_count: number;
    functions: FunctionInfo[];
    classes: ClassInfo[];
    imports: string[];
    exports: string[];
    complexity_score: number;
  }
  
  interface FunctionInfo {
    name: string;
    line_start: number;
    line_end: number;
    parameters: string[];
    return_type?: string;
    calls: string[];
    complexity: number;
    is_public: boolean;
  }
  
  interface ClassInfo {
    name: string;
    line_start: number;
    line_end: number;
    methods: FunctionInfo[];
    properties: string[];
    extends?: string;
    implements: string[];
    is_public: boolean;
  }
  
  interface ArchitectureAnalysis {
    project_path: string;
    analyzed_at: string;
    analyzer_version: string;
    file_count: number;
    language_breakdown: Record<string, number>;
    modules: ModuleInfo[];
    dependencies: any[];
    diagrams: Record<string, string>;
    metrics: ArchitectureMetrics;
  }
  
  interface ArchitectureMetrics {
    total_lines: number;
    total_functions: number;
    total_classes: number;
    average_complexity: number;
    coupling_score: number;
    cohesion_score: number;
    maintainability_index: number;
    technical_debt_ratio: number;
  }
  
  // Component state
  let isAnalyzing = $state(false);
  let analysis: ArchitectureAnalysis | null = $state(null);
  let error: string | null = $state(null);
  let config: ArchVizConfig | null = $state(null);
  let selectedProjectPath = $state('');
  let selectedView = $state<'overview' | 'modules' | 'diagram' | 'metrics'>('overview');
  let selectedModule: ModuleInfo | null = $state(null);
  let diagramFormat = $state<'Mermaid' | 'SVG' | 'PNG'>('Mermaid');
  let selectedDiagramType = $state<'overview' | 'dependency' | 'class' | 'files' | 'graphviz' | 'plantuml'>('overview');
  let generatedDiagrams = $state<Record<string, string>>({});
  let isGeneratingDiagrams = $state(false);
  let activeViewTabs = $state<Record<string, 'visual' | 'source'>>({});
  
  // Tab configuration
  const tabs = [
    { id: 'overview' as const, label: 'Overview', icon: '📊' },
    { id: 'modules' as const, label: 'Modules', icon: '📁' },
    { id: 'diagram' as const, label: 'Diagram', icon: '🔗' },
    { id: 'metrics' as const, label: 'Metrics', icon: '📈' }
  ];
  
  // Load configuration on mount
  onMount(async () => {
    try {
      config = await invoke<ArchVizConfig>('get_archviz_config');
      // Start with empty path - user must select
      selectedProjectPath = '';
      
      // Initialize Mermaid
      mermaid.initialize({
        startOnLoad: false,
        theme: 'default',
        securityLevel: 'loose',
        fontFamily: 'monospace'
      });
    } catch (e) {
      error = `Failed to load ArchViz config: ${e}`;
    }
  });

  // Render Mermaid diagram
  async function renderMermaidDiagram(diagramCode: string, elementId: string) {
    try {
      const element = document.getElementById(elementId);
      if (element) {
        element.innerHTML = '';
        const { svg } = await mermaid.render(`diagram-${Date.now()}`, diagramCode);
        element.innerHTML = svg;
      }
    } catch (e) {
      console.error('Failed to render Mermaid diagram:', e);
      const element = document.getElementById(elementId);
      if (element) {
        element.innerHTML = `<div class="text-red-600 p-4">Failed to render diagram: ${e}</div>`;
      }
    }
  }
  
  // Open directory picker
  async function selectProjectDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Project Directory to Analyze'
      });
      
      if (selected && typeof selected === 'string') {
        selectedProjectPath = selected;
        error = null; // Clear any previous errors
      }
    } catch (e) {
      error = `Failed to select directory: ${e}`;
    }
  }

  // Run architecture analysis
  async function runAnalysis() {
    if (!selectedProjectPath) {
      error = 'Please select a project path';
      return;
    }
    
    isAnalyzing = true;
    error = null;
    
    try {
      analysis = await invoke<ArchitectureAnalysis>('run_architecture_analysis', {
        projectPath: selectedProjectPath,
        config: config
      });
      selectedView = 'overview';
    } catch (e) {
      error = `Analysis failed: ${e}`;
    } finally {
      isAnalyzing = false;
    }
  }
  
  // Generate diagram in specific format
  async function generateDiagram(format: string) {
    if (!selectedProjectPath) return;
    
    try {
      const diagram = await invoke<string>('generate_diagram', {
        projectPath: selectedProjectPath,
        format: format
      });
      
      if (analysis) {
        analysis.diagrams[format] = diagram;
      }
    } catch (e) {
      error = `Failed to generate ${format} diagram: ${e}`;
    }
  }

  // Generate specific diagram type
  async function generateSpecificDiagram(diagramType: string) {
    if (!selectedProjectPath) return;
    
    try {
      isGeneratingDiagrams = true;
      const diagram = await invoke<string>('generate_diagram', {
        projectPath: selectedProjectPath,
        format: 'mermaid',
        diagramType
      });
      
      generatedDiagrams[diagramType] = diagram;
    } catch (e) {
      error = `Failed to generate ${diagramType} diagram: ${e}`;
    } finally {
      isGeneratingDiagrams = false;
    }
  }

  // Generate all diagram types automatically
  async function generateAllDiagrams() {
    if (!selectedProjectPath) return;
    
    try {
      isGeneratingDiagrams = true;
      const diagramTypes = ['overview', 'dependency', 'class', 'files', 'graphviz', 'plantuml'];
      
      for (const diagramType of diagramTypes) {
        try {
          const diagram = await invoke<string>('generate_diagram', {
            projectPath: selectedProjectPath,
            format: 'mermaid',
            diagramType
          });
          generatedDiagrams[diagramType] = diagram;
        } catch (e) {
          console.warn(`Failed to generate ${diagramType} diagram:`, e);
        }
      }
    } catch (e) {
      error = `Failed to generate diagrams: ${e}`;
    } finally {
      isGeneratingDiagrams = false;
    }
  }

  // Export all diagrams to files
  async function exportAllDiagrams() {
    if (!selectedProjectPath) return;
    
    try {
      const exportedFiles = await invoke<string[]>('export_all_diagrams', {
        projectPath: selectedProjectPath,
        outputDir: null // Use default
      });
      
      alert(`Exported ${exportedFiles.length} diagrams:\n${exportedFiles.join('\n')}`);
    } catch (e) {
      error = `Failed to export diagrams: ${e}`;
    }
  }

  // Auto-render Mermaid diagrams when they're generated
  $effect(() => {
    if (Object.keys(generatedDiagrams).length > 0) {
      // Small delay to ensure DOM elements are ready
      setTimeout(() => {
        Object.entries(generatedDiagrams).forEach(([diagramType, diagramContent]) => {
          if (['overview', 'dependency', 'class', 'files'].includes(diagramType)) {
            renderMermaidDiagram(diagramContent, `mermaid-render-${diagramType}`);
          }
        });
      }, 500);
    }
  });
  
  // Get language color for visualization
  function getLanguageColor(language: string): string {
    switch (language) {
      case 'rust': return '#dea584';
      case 'javascript': return '#f7df1e';
      case 'typescript': return '#3178c6';
      case 'python': return '#3776ab';
      case 'json': return '#292929';
      default: return '#gray';
    }
  }
  
  // Get complexity color
  function getComplexityColor(score: number): string {
    if (score <= 2) return 'text-green-600';
    if (score <= 5) return 'text-yellow-600';
    if (score <= 10) return 'text-orange-600';
    return 'text-red-600';
  }
  
  // Get maintainability color
  function getMaintainabilityColor(index: number): string {
    if (index >= 80) return 'text-green-600';
    if (index >= 60) return 'text-yellow-600';
    if (index >= 40) return 'text-orange-600';
    return 'text-red-600';
  }
  
  // Format file size
  function formatFileSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

<div class="p-6 bg-gradient-to-br from-slate-50 to-slate-100 rounded-lg shadow-lg">
  <div class="mb-6">
    <h2 class="text-2xl font-bold text-gray-900 mb-2">
      Architecture Visualization
    </h2>
    <p class="text-gray-600">
      Automated code analysis and interactive architecture diagrams using Tree-sitter AST parsing.
    </p>
  </div>
  
  <!-- Configuration Section -->
   {#if config}
     <div class="mb-6 p-6 bg-gradient-to-r from-blue-50 to-indigo-50 border-2 border-blue-200 rounded-lg shadow-sm">
      <h3 class="text-xl font-bold text-gray-900 mb-4">⚙️ Analysis Configuration</h3>
      <div class="grid grid-cols-2 gap-6 text-base">
        <div>
          <span class="font-bold text-gray-900">Languages:</span>
          <div class="flex flex-wrap gap-2 mt-2">
            {#each config.languages as language}
              <span 
                class="px-3 py-1 text-sm font-semibold rounded-full text-white border-2 border-white shadow-sm"
                style="background-color: {getLanguageColor(language)}"
              >
                {language}
              </span>
            {/each}
          </div>
        </div>
        <div class="space-y-3">
          <div>
            <span class="font-bold text-gray-900">Max Depth:</span>
            <span class="ml-2 text-gray-900 bg-gray-100 px-2 py-1 rounded font-mono">{config.max_depth}</span>
          </div>
          <div>
            <span class="font-bold text-gray-900">Include Tests:</span>
            <span class="ml-2 text-gray-900 bg-gray-100 px-2 py-1 rounded font-medium">{config.include_tests ? 'Yes' : 'No'}</span>
          </div>
          <div>
            <span class="font-bold text-gray-900">Output Format:</span>
            <span class="ml-2 text-gray-900 bg-gray-100 px-2 py-1 rounded font-medium">{config.output_format}</span>
          </div>
        </div>
      </div>
    </div>
  {/if}
  
   <!-- Project Selection and Analysis Controls -->
   <div class="mb-6 bg-white p-4 rounded-lg border-2 border-indigo-200 shadow-sm">
    <label for="project-path" class="block text-lg font-semibold text-gray-900 mb-3">
      📁 Select Project Directory
    </label>
    
    {#if selectedProjectPath}
      <div class="mb-4 p-3 bg-green-50 border border-green-200 rounded-lg">
        <div class="text-sm font-medium text-green-800 mb-1">Selected Directory:</div>
        <code class="text-green-900 bg-white px-2 py-1 rounded text-sm break-all">{selectedProjectPath}</code>
      </div>
    {:else}
      <div class="mb-4 p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
        <div class="text-yellow-800">⚠️ No directory selected. Please choose a project directory to analyze.</div>
      </div>
    {/if}
    
    <div class="flex gap-3">
      <button
        onclick={selectProjectDirectory}
        class="px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 flex items-center gap-2 font-medium"
      >
        📂 Browse Directories
      </button>
      
      <input
        id="project-path"
        type="text"
        bind:value={selectedProjectPath}
        placeholder="Or type path manually: /path/to/project"
        class="flex-1 px-4 py-3 border-2 border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 text-gray-900 font-mono"
      />
    </div>
    
    <div class="mt-4">
      <button
        onclick={runAnalysis}
        disabled={isAnalyzing || !selectedProjectPath}
        class="w-full px-6 py-4 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 font-semibold text-lg"
      >
        {#if isAnalyzing}
          <div class="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
          Analyzing Architecture...
        {:else if !selectedProjectPath}
          ⚠️ Select Directory First
        {:else}
          🔍 Analyze Project Architecture
        {/if}
      </button>
      
      {#if selectedProjectPath && !isAnalyzing}
        <div class="mt-2 text-sm text-gray-600 text-center">
          This will analyze code structure, dependencies, and generate architecture diagrams
      </div>
    {/if}
  </div>
</div>

<style>
  :global(.mermaid-container svg) {
    max-width: 100%;
    height: auto;
  }
  
  :global(.mermaid-container .node rect) {
    stroke-width: 2px;
  }
  
  :global(.mermaid-container .edgePath path) {
    stroke-width: 2px;
  }
  
  :global(.mermaid-container .cluster rect) {
    stroke-width: 2px;
    stroke-dasharray: 5,5;
  }
  
  :global(.mermaid-container text) {
    font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
    font-size: 12px;
  }
</style>
  
  <!-- Error Display -->
  {#if error}
    <div class="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg">
      <div class="flex items-center">
        <div class="text-red-600 font-medium">Error:</div>
        <div class="ml-2 text-red-700">{error}</div>
      </div>
    </div>
  {/if}
  
  <!-- Results Display -->
  {#if analysis}
    <!-- Navigation Tabs -->
    <div class="mb-6 border-b border-gray-200">
      <nav class="flex space-x-8">
        {#each tabs as tab}
          <button
            onclick={() => selectedView = tab.id}
            class="py-2 px-1 border-b-2 font-medium text-sm {selectedView === tab.id 
              ? 'border-indigo-500 text-indigo-600' 
              : 'border-transparent text-gray-700 hover:text-gray-900 hover:border-gray-300'}"
          >
            {tab.icon} {tab.label}
          </button>
        {/each}
      </nav>
    </div>
    
    <!-- Overview Tab -->
    {#if selectedView === 'overview'}
      <div class="space-y-6">
        <!-- Project Info Header -->
        <div class="bg-indigo-50 border border-indigo-200 rounded-lg p-4">
          <h3 class="text-xl font-bold text-indigo-900 mb-2">📊 Analysis Results</h3>
          <div class="text-indigo-800">
            <strong>Project:</strong> <code class="bg-white px-2 py-1 rounded text-indigo-900">{analysis.project_path}</code>
          </div>
          <div class="text-indigo-700 text-sm mt-1">
            Analyzed at: {new Date(analysis.analyzed_at).toLocaleString()}
          </div>
        </div>
        
         <!-- Quick Stats -->
         <div class="grid grid-cols-4 gap-4">
           <div class="bg-gradient-to-br from-slate-100 to-slate-50 p-6 rounded-lg border-2 border-slate-300 shadow-sm">
             <div class="text-3xl font-bold text-gray-900">{analysis.file_count}</div>
             <div class="text-base font-medium text-gray-700">Files Analyzed</div>
           </div>
           <div class="bg-gradient-to-br from-blue-100 to-blue-50 p-6 rounded-lg border-2 border-blue-300 shadow-sm">
             <div class="text-3xl font-bold text-blue-700">{analysis.metrics.total_functions}</div>
             <div class="text-base font-medium text-gray-700">Functions</div>
           </div>
           <div class="bg-gradient-to-br from-green-100 to-green-50 p-6 rounded-lg border-2 border-green-300 shadow-sm">
             <div class="text-3xl font-bold text-green-700">{analysis.metrics.total_classes}</div>
             <div class="text-base font-medium text-gray-700">Classes</div>
           </div>
           <div class="bg-gradient-to-br from-purple-100 to-purple-50 p-6 rounded-lg border-2 border-purple-300 shadow-sm">
             <div class="text-3xl font-bold text-purple-700">{analysis.metrics.total_lines.toLocaleString()}</div>
             <div class="text-base font-medium text-gray-700">Lines of Code</div>
           </div>
         </div>
        
         <!-- Language Breakdown -->
         <div class="bg-white p-6 rounded-lg border-2 border-indigo-200 shadow-sm">
          <h3 class="text-xl font-bold text-gray-900 mb-6">🔤 Language Breakdown</h3>
          <div class="space-y-4">
            {#each Object.entries(analysis.language_breakdown) as [language, count]}
              <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                <div class="flex items-center gap-4">
                  <div 
                    class="w-6 h-6 rounded-full border-2 border-white shadow-sm"
                    style="background-color: {getLanguageColor(language)}"
                  ></div>
                  <span class="text-lg font-semibold capitalize text-gray-900">{language}</span>
                </div>
                <div class="text-lg font-bold text-gray-800 bg-white px-3 py-1 rounded">{count} files</div>
              </div>
            {/each}
          </div>
        </div>
        
        <!-- Analysis Metadata -->
        <div class="text-xs text-gray-700 pt-4 border-t border-gray-200">
          <div>Analyzed: {new Date(analysis.analyzed_at).toLocaleString()}</div>
          <div>Analyzer Version: {analysis.analyzer_version}</div>
          <div>Project: {analysis.project_path}</div>
        </div>
      </div>
    {/if}
    
    <!-- Modules Tab -->
    {#if selectedView === 'modules'}
      <div class="space-y-6">
        <div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
          <h3 class="text-xl font-bold text-blue-900">📁 Project Modules</h3>
          <div class="text-blue-800 font-medium">{analysis.modules.length} modules found</div>
          <div class="text-blue-700 text-sm mt-1">Click any module to expand details</div>
        </div>
        
        <div class="grid gap-4">
          {#each analysis.modules as module}
            <div 
              class="p-4 border border-gray-200 rounded-lg hover:border-indigo-300 cursor-pointer transition-colors"
              role="button"
              tabindex="0"
              aria-expanded={selectedModule === module}
              onclick={() => selectedModule = selectedModule === module ? null : module}
              onkeydown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault();
                  selectedModule = selectedModule === module ? null : module;
                }
              }}
            >
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <div class="flex items-center gap-3 mb-2">
                    <div 
                      class="w-3 h-3 rounded-full"
                      style="background-color: {getLanguageColor(module.language)}"
                    ></div>
                    <h4 class="font-medium text-gray-900">{module.path.split('/').pop()}</h4>
                    <span class="text-xs text-gray-700 capitalize font-medium">{module.language}</span>
                  </div>
                  <div class="text-sm text-gray-600 mb-2">{module.path}</div>
                  <div class="flex gap-4 text-sm text-gray-700">
                    <span>{module.line_count} lines</span>
                    <span>{formatFileSize(module.size_bytes)}</span>
                    <span>{module.functions.length} functions</span>
                    <span>{module.classes.length} classes</span>
                  </div>
                </div>
                <div class="text-right">
                  <div class="text-sm {getComplexityColor(module.complexity_score)}">
                    Complexity: {module.complexity_score.toFixed(1)}
                  </div>
                </div>
              </div>
              
              <!-- Expanded Module Details -->
              {#if selectedModule === module}
                <div class="mt-4 pt-4 border-t border-gray-100">
                  <div class="grid grid-cols-2 gap-6">
                    <!-- Functions -->
                    {#if module.functions.length > 0}
                      <div>
                        <h5 class="font-medium text-gray-900 mb-2">Functions ({module.functions.length})</h5>
                        <div class="space-y-1 max-h-32 overflow-y-auto">
                          {#each module.functions as func}
                            <div class="text-sm p-2 bg-gray-50 rounded">
                              <div class="flex justify-between items-center">
                                <span class="font-mono">{func.name}()</span>
                                <span class="text-xs text-gray-500">
                                  {func.is_public ? 'public' : 'private'}
                                </span>
                              </div>
                              <div class="text-xs text-gray-500">
                                Lines {func.line_start}-{func.line_end}
                              </div>
                            </div>
                          {/each}
                        </div>
                      </div>
                    {/if}
                    
                    <!-- Classes -->
                    {#if module.classes.length > 0}
                      <div>
                        <h5 class="font-medium text-gray-900 mb-2">Classes ({module.classes.length})</h5>
                        <div class="space-y-1 max-h-32 overflow-y-auto">
                          {#each module.classes as cls}
                            <div class="text-sm p-2 bg-gray-50 rounded">
                              <div class="flex justify-between items-center">
                                <span class="font-mono">{cls.name}</span>
                                <span class="text-xs text-gray-500">
                                  {cls.is_public ? 'public' : 'private'}
                                </span>
                              </div>
                              <div class="text-xs text-gray-500">
                                Lines {cls.line_start}-{cls.line_end}, {cls.methods.length} methods
                              </div>
                            </div>
                          {/each}
                        </div>
                      </div>
                    {/if}
                  </div>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/if}
    
    <!-- Diagram Tab -->
    {#if selectedView === 'diagram'}
      <div class="space-y-6">
        <!-- Header with Controls -->
        <div class="bg-purple-50 border border-purple-200 rounded-lg p-4">
          <h3 class="text-xl font-bold text-purple-900 mb-4">🔗 Architecture Diagrams</h3>
          
          <div class="flex flex-wrap gap-3 mb-4">
            <select 
              bind:value={selectedDiagramType}
              class="px-4 py-2 bg-white text-gray-900 border-2 border-gray-400 rounded-md focus:ring-2 focus:ring-purple-500 focus:border-purple-500 font-medium shadow-sm"
            >
              <option value="overview">📊 Architecture Overview</option>
              <option value="dependency">🔗 Dependency Graph</option>
              <option value="class">🏗️ Class Hierarchy</option>
              <option value="files">📁 File Organization</option>
              <option value="graphviz">🎯 Graphviz DOT</option>
              <option value="plantuml">📐 PlantUML</option>
            </select>
            
            <button
              onclick={() => generateSpecificDiagram(selectedDiagramType)}
              disabled={isGeneratingDiagrams}
              class="px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 disabled:opacity-50 flex items-center gap-2"
            >
              {#if isGeneratingDiagrams}
                <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
              {/if}
              Generate Selected
            </button>
            
            <button
              onclick={generateAllDiagrams}
              disabled={isGeneratingDiagrams}
              class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 flex items-center gap-2"
            >
              🚀 Generate All Types
            </button>
            
            <button
              onclick={exportAllDiagrams}
              disabled={isGeneratingDiagrams || Object.keys(generatedDiagrams).length === 0}
              class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50 flex items-center gap-2"
            >
              💾 Export All
            </button>
          </div>
          
          <div class="text-purple-700 text-sm">
            Generate multiple diagram types automatically: overview, dependencies, classes, file structure, and more!
          </div>
        </div>

        <!-- Generated Diagrams Display -->
        {#if Object.keys(generatedDiagrams).length > 0}
          <div class="space-y-6">
             {#each Object.entries(generatedDiagrams) as [diagramType, diagramContent]}
               <div class="bg-white border-2 border-indigo-200 rounded-lg p-4 shadow-sm">
                <div class="flex justify-between items-center mb-3">
                  <h4 class="text-lg font-semibold text-gray-900">
                    {#if diagramType === 'overview'}📊 Architecture Overview
                    {:else if diagramType === 'dependency'}🔗 Dependency Graph
                    {:else if diagramType === 'class'}🏗️ Class Hierarchy
                    {:else if diagramType === 'files'}📁 File Organization
                    {:else if diagramType === 'graphviz'}🎯 Graphviz DOT
                    {:else if diagramType === 'plantuml'}📐 PlantUML
                    {:else}{diagramType.toUpperCase()}
                    {/if}
                  </h4>
                  <button
                    onclick={() => navigator.clipboard.writeText(diagramContent)}
                    class="px-3 py-1 bg-gray-100 text-gray-700 rounded text-sm hover:bg-gray-200"
                  >
                    📋 Copy
                  </button>
                </div>
                
                <!-- Diagram Display with Visual Rendering -->
                <div class="space-y-4">
                  {#if diagramType === 'overview' || diagramType === 'dependency' || diagramType === 'class' || diagramType === 'files'}
                    <!-- Mermaid Visual Rendering -->
                    <div class="bg-white rounded-lg border-2 border-gray-200 p-6">
                      <div class="flex justify-between items-center mb-4">
                        <h5 class="text-lg font-semibold text-gray-900">🎨 Visual Diagram</h5>
                        <button
                          onclick={() => renderMermaidDiagram(diagramContent, `mermaid-render-${diagramType}`)}
                          class="px-3 py-1 bg-indigo-100 text-indigo-700 rounded text-sm hover:bg-indigo-200"
                        >
                          🔄 Re-render
                        </button>
                      </div>
                      <div 
                        id="mermaid-render-{diagramType}" 
                        class="mermaid-container min-h-64 border border-gray-200 rounded-lg p-4 bg-gray-50 overflow-auto"
                      >
                        <div class="flex items-center justify-center h-64 text-gray-500">
                          <div class="text-center">
                            <div class="animate-spin w-8 h-8 border-4 border-indigo-500 border-t-transparent rounded-full mx-auto mb-2"></div>
                            <div>Click "Re-render" to display visual diagram</div>
                          </div>
                        </div>
                      </div>
                    </div>
                  {:else}
                    <!-- Non-Mermaid Diagram Instructions -->
                    <div class="bg-white rounded-lg border-2 border-gray-200 p-6 text-center">
                      {#if diagramType === 'graphviz'}
                        <div class="text-4xl mb-4">🎯</div>
                        <div class="text-lg font-semibold text-gray-900 mb-2">Graphviz DOT Diagram</div>
                        <div class="text-gray-600 mb-4">
                          Copy the source code below and paste it into Graphviz Online for visual rendering
                        </div>
                        <a 
                          href="https://dreampuf.github.io/GraphvizOnline/"
                          target="_blank"
                          class="inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
                        >
                          🌐 Open Graphviz Online
                        </a>
                      {:else if diagramType === 'plantuml'}
                        <div class="text-4xl mb-4">📐</div>
                        <div class="text-lg font-semibold text-gray-900 mb-2">PlantUML Diagram</div>
                        <div class="text-gray-600 mb-4">
                          Copy the source code below and paste it into PlantUML Online for visual rendering
                        </div>
                        <a 
                          href="https://www.plantuml.com/plantuml/uml/"
                          target="_blank"
                          class="inline-flex items-center px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700"
                        >
                          🌐 Open PlantUML Online
                        </a>
                      {/if}
                    </div>
                  {/if}

                  <!-- Source Code Section -->
                  <div class="bg-gray-50 rounded-lg p-4">
                    <div class="flex justify-between items-center mb-3">
                      <h5 class="text-lg font-semibold text-gray-900">📝 Source Code</h5>
                      <div class="flex gap-2">
                        <button
                          onclick={() => navigator.clipboard.writeText(diagramContent)}
                          class="px-3 py-1 bg-gray-100 text-gray-700 rounded text-sm hover:bg-gray-200"
                        >
                          📋 Copy Code
                        </button>
                        {#if diagramType === 'overview' || diagramType === 'dependency' || diagramType === 'class' || diagramType === 'files'}
                          <a
                            href="https://mermaid.live"
                            target="_blank"
                            class="px-3 py-1 bg-blue-100 text-blue-700 rounded text-sm hover:bg-blue-200"
                          >
                            🌐 Open in Mermaid Live
                          </a>
                        {/if}
                      </div>
                    </div>
                    <pre class="text-sm bg-white p-4 rounded border overflow-x-auto whitespace-pre-wrap"><code>{diagramContent}</code></pre>
                  </div>
                </div>
                
                {#if diagramType === 'overview' || diagramType === 'dependency' || diagramType === 'class' || diagramType === 'files'}
                  <div class="mt-2 text-sm text-gray-600">
                    💡 Copy this Mermaid code to <a href="https://mermaid.live" target="_blank" class="text-indigo-600 hover:underline">mermaid.live</a> for interactive viewing
                  </div>
                {:else if diagramType === 'graphviz'}
                  <div class="mt-2 text-sm text-gray-600">
                    💡 Use this DOT code with Graphviz tools or <a href="https://dreampuf.github.io/GraphvizOnline/" target="_blank" class="text-indigo-600 hover:underline">Graphviz Online</a>
                  </div>
                {:else if diagramType === 'plantuml'}
                  <div class="mt-2 text-sm text-gray-600">
                    💡 Use this PlantUML code with <a href="https://www.plantuml.com/plantuml/uml/" target="_blank" class="text-indigo-600 hover:underline">PlantUML Online</a>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {:else if analysis?.diagrams?.Mermaid}
          <!-- Fallback to original analysis diagram -->
          <div class="bg-gray-50 p-4 rounded-lg">
            <h4 class="font-medium text-gray-900 mb-3">Original Analysis Diagram</h4>
            <pre class="text-sm bg-white p-4 rounded border overflow-x-auto"><code>{analysis.diagrams.Mermaid}</code></pre>
          </div>
        {:else}
          <div class="text-center py-8 text-gray-500">
            <div class="text-4xl mb-2">🎨</div>
            <div>No diagrams generated yet. Click "Generate All Types" to create automatic diagrams!</div>
          </div>
        {/if}
      </div>
    {/if}
    
    <!-- Metrics Tab -->
    {#if selectedView === 'metrics'}
      <div class="space-y-6">
        <h3 class="text-lg font-semibold text-gray-900">Architecture Metrics</h3>
        
        <div class="grid grid-cols-2 gap-6">
          <!-- Code Quality Metrics -->
          <div class="bg-white p-6 rounded-lg border border-gray-200">
            <h4 class="font-medium text-gray-900 mb-4">Code Quality</h4>
            <div class="space-y-3">
              <div class="flex justify-between items-center">
                <span class="text-sm text-gray-600">Average Complexity</span>
                <span class="font-medium {getComplexityColor(analysis.metrics.average_complexity)}">
                  {analysis.metrics.average_complexity.toFixed(2)}
                </span>
              </div>
              <div class="flex justify-between items-center">
                <span class="text-sm text-gray-600">Maintainability Index</span>
                <span class="font-medium {getMaintainabilityColor(analysis.metrics.maintainability_index)}">
                  {analysis.metrics.maintainability_index.toFixed(1)}
                </span>
              </div>
              <div class="flex justify-between items-center">
                <span class="text-sm text-gray-600">Technical Debt Ratio</span>
                <span class="font-medium text-orange-600">
                  {(analysis.metrics.technical_debt_ratio * 100).toFixed(1)}%
                </span>
              </div>
            </div>
          </div>
          
          <!-- Architecture Metrics -->
          <div class="bg-white p-6 rounded-lg border border-gray-200">
            <h4 class="font-medium text-gray-900 mb-4">Architecture</h4>
            <div class="space-y-3">
              <div class="flex justify-between items-center">
                <span class="text-sm text-gray-600">Coupling Score</span>
                <span class="font-medium text-blue-600">
                  {analysis.metrics.coupling_score.toFixed(2)}
                </span>
              </div>
              <div class="flex justify-between items-center">
                <span class="text-sm text-gray-600">Cohesion Score</span>
                <span class="font-medium text-green-600">
                  {analysis.metrics.cohesion_score.toFixed(2)}
                </span>
              </div>
              <div class="flex justify-between items-center">
                <span class="text-sm text-gray-600">Dependencies</span>
                <span class="font-medium text-gray-900">
                  {analysis.dependencies.length}
                </span>
              </div>
            </div>
          </div>
        </div>
        
        <!-- Recommendations -->
        <div class="bg-blue-50 p-4 rounded-lg">
          <h4 class="font-medium text-blue-900 mb-3">Recommendations</h4>
          <ul class="space-y-2 text-sm text-blue-800">
            {#if analysis.metrics.average_complexity > 5}
              <li class="flex items-start gap-2">
                <span class="text-orange-500">⚠️</span>
                High average complexity detected. Consider refactoring complex functions.
              </li>
            {/if}
            {#if analysis.metrics.maintainability_index < 60}
              <li class="flex items-start gap-2">
                <span class="text-red-500">🚨</span>
                Low maintainability index. Review code structure and documentation.
              </li>
            {/if}
            {#if analysis.metrics.coupling_score > 3}
              <li class="flex items-start gap-2">
                <span class="text-yellow-500">💡</span>
                High coupling detected. Consider reducing dependencies between modules.
              </li>
            {/if}
            {#if analysis.metrics.technical_debt_ratio > 0.2}
              <li class="flex items-start gap-2">
                <span class="text-orange-500">📊</span>
                Significant technical debt. Plan refactoring sessions to improve code quality.
              </li>
            {/if}
          </ul>
        </div>
      </div>
    {/if}
  {/if}
</div>