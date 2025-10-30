<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  
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
  
  // Load configuration on mount
  onMount(async () => {
    try {
      config = await invoke<ArchVizConfig>('get_archviz_config');
      // Default to current project directory
      selectedProjectPath = '/Users/tempext/Projects/disk-bloat-scanner';
    } catch (e) {
      error = `Failed to load ArchViz config: ${e}`;
    }
  });
  
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

<div class="p-6 bg-white rounded-lg shadow-lg">
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
    <div class="mb-6 p-4 bg-gray-50 rounded-lg">
      <h3 class="text-lg font-semibold mb-3">Configuration</h3>
      <div class="grid grid-cols-2 gap-4 text-sm">
        <div>
          <span class="font-medium">Languages:</span>
          <div class="flex flex-wrap gap-1 mt-1">
            {#each config.languages as language}
              <span 
                class="px-2 py-1 text-xs rounded-full text-white"
                style="background-color: {getLanguageColor(language)}"
              >
                {language}
              </span>
            {/each}
          </div>
        </div>
        <div>
          <span class="font-medium">Max Depth:</span>
          <span class="ml-2">{config.max_depth}</span>
        </div>
        <div>
          <span class="font-medium">Include Tests:</span>
          <span class="ml-2">{config.include_tests ? 'Yes' : 'No'}</span>
        </div>
        <div>
          <span class="font-medium">Output Format:</span>
          <span class="ml-2">{config.output_format}</span>
        </div>
      </div>
    </div>
  {/if}
  
  <!-- Project Selection and Analysis Controls -->
  <div class="mb-6">
    <label for="project-path" class="block text-sm font-medium text-gray-700 mb-2">
      Project Path
    </label>
    <div class="flex gap-3">
      <input
        id="project-path"
        type="text"
        bind:value={selectedProjectPath}
        placeholder="/path/to/project"
        class="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
      />
      <button
        onclick={runAnalysis}
        disabled={isAnalyzing || !selectedProjectPath}
        class="px-6 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      >
        {#if isAnalyzing}
          <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
          Analyzing...
        {:else}
          Run Analysis
        {/if}
      </button>
    </div>
  </div>
  
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
        {#each [
          { id: 'overview', label: 'Overview', icon: '📊' },
          { id: 'modules', label: 'Modules', icon: '📁' },
          { id: 'diagram', label: 'Diagram', icon: '🔗' },
          { id: 'metrics', label: 'Metrics', icon: '📈' }
        ] as tab}
          <button
            onclick={() => selectedView = tab.id}
            class="py-2 px-1 border-b-2 font-medium text-sm {selectedView === tab.id 
              ? 'border-indigo-500 text-indigo-600' 
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
          >
            {tab.icon} {tab.label}
          </button>
        {/each}
      </nav>
    </div>
    
    <!-- Overview Tab -->
    {#if selectedView === 'overview'}
      <div class="space-y-6">
        <!-- Quick Stats -->
        <div class="grid grid-cols-4 gap-4">
          <div class="bg-white p-4 rounded-lg border border-gray-200">
            <div class="text-2xl font-bold text-gray-900">{analysis.file_count}</div>
            <div class="text-sm text-gray-600">Files Analyzed</div>
          </div>
          <div class="bg-white p-4 rounded-lg border border-gray-200">
            <div class="text-2xl font-bold text-blue-600">{analysis.metrics.total_functions}</div>
            <div class="text-sm text-gray-600">Functions</div>
          </div>
          <div class="bg-white p-4 rounded-lg border border-gray-200">
            <div class="text-2xl font-bold text-green-600">{analysis.metrics.total_classes}</div>
            <div class="text-sm text-gray-600">Classes</div>
          </div>
          <div class="bg-white p-4 rounded-lg border border-gray-200">
            <div class="text-2xl font-bold text-purple-600">{analysis.metrics.total_lines.toLocaleString()}</div>
            <div class="text-sm text-gray-600">Lines of Code</div>
          </div>
        </div>
        
        <!-- Language Breakdown -->
        <div class="bg-white p-6 rounded-lg border border-gray-200">
          <h3 class="text-lg font-semibold text-gray-900 mb-4">Language Breakdown</h3>
          <div class="space-y-3">
            {#each Object.entries(analysis.language_breakdown) as [language, count]}
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <div 
                    class="w-4 h-4 rounded-full"
                    style="background-color: {getLanguageColor(language)}"
                  ></div>
                  <span class="font-medium capitalize">{language}</span>
                </div>
                <div class="text-gray-600">{count} files</div>
              </div>
            {/each}
          </div>
        </div>
        
        <!-- Analysis Metadata -->
        <div class="text-xs text-gray-500 pt-4 border-t border-gray-200">
          <div>Analyzed: {new Date(analysis.analyzed_at).toLocaleString()}</div>
          <div>Analyzer Version: {analysis.analyzer_version}</div>
          <div>Project: {analysis.project_path}</div>
        </div>
      </div>
    {/if}
    
    <!-- Modules Tab -->
    {#if selectedView === 'modules'}
      <div class="space-y-4">
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-900">Project Modules</h3>
          <div class="text-sm text-gray-600">{analysis.modules.length} modules</div>
        </div>
        
        <div class="grid gap-4">
          {#each analysis.modules as module}
            <div 
              class="p-4 border border-gray-200 rounded-lg hover:border-indigo-300 cursor-pointer transition-colors"
              onclick={() => selectedModule = selectedModule === module ? null : module}
            >
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <div class="flex items-center gap-3 mb-2">
                    <div 
                      class="w-3 h-3 rounded-full"
                      style="background-color: {getLanguageColor(module.language)}"
                    ></div>
                    <h4 class="font-medium text-gray-900">{module.path.split('/').pop()}</h4>
                    <span class="text-xs text-gray-500 capitalize">{module.language}</span>
                  </div>
                  <div class="text-sm text-gray-600 mb-2">{module.path}</div>
                  <div class="flex gap-4 text-sm text-gray-500">
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
        <div class="flex justify-between items-center">
          <h3 class="text-lg font-semibold text-gray-900">Architecture Diagram</h3>
          <div class="flex gap-2">
            <select 
              bind:value={diagramFormat}
              class="px-3 py-1 border border-gray-300 rounded text-sm"
            >
              <option value="Mermaid">Mermaid</option>
              <option value="SVG">SVG</option>
              <option value="PNG">PNG</option>
            </select>
            <button
              onclick={() => generateDiagram(diagramFormat)}
              class="px-3 py-1 bg-indigo-600 text-white rounded text-sm hover:bg-indigo-700"
            >
              Generate
            </button>
          </div>
        </div>
        
        {#if analysis.diagrams.Mermaid}
          <div class="bg-gray-50 p-4 rounded-lg">
            <h4 class="font-medium text-gray-900 mb-3">Mermaid Diagram</h4>
            <pre class="text-sm bg-white p-4 rounded border overflow-x-auto"><code>{analysis.diagrams.Mermaid}</code></pre>
          </div>
        {/if}
        
        <div class="text-sm text-gray-600">
          💡 Tip: Copy the Mermaid code above and paste it into 
          <a href="https://mermaid.live" target="_blank" class="text-indigo-600 hover:underline">mermaid.live</a> 
          to see the interactive diagram.
        </div>
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