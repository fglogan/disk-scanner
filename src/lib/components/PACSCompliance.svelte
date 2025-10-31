<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { onMount } from 'svelte';
  import PACSBaselineManager from './PACSBaselineManager.svelte';
  
  // PACS types (matching Rust structs)
  interface PACSConfig {
    auto_generate_specs: boolean;
    auto_create_beads: boolean;
    standards: string[];
    output_dir: string;
    max_depth: number;
  }
  
  interface ComplianceFinding {
    id: string;
    severity: 'Critical' | 'High' | 'Medium' | 'Low' | 'Info';
    category: string;
    title: string;
    description: string;
    file_path?: string;
    line_number?: number;
    recommendation: string;
    auto_fixable: boolean;
    standard: string;
    detected_at: string;
  }
  
  interface ComplianceStatus {
    compliant: boolean;
    score: number;
    issues_count: number;
    critical_issues: number;
  }
  
  interface ProjectAuditReport {
    project_path: string;
    scanned_at: string;
    scanner_version: string;
    compliance_score: number;
    findings: ComplianceFinding[];
    baseline?: any;
    recommendations: string[];
    auto_fixes_available: number;
    standards_summary: Record<string, ComplianceStatus>;
  }
  
  // Component state
  let isScanning = $state(false);
  let scanProgress = $state('');
  let report: ProjectAuditReport | null = $state(null);
  let error: string | null = $state(null);
  let config: PACSConfig | null = $state(null);
  let selectedProjectPath = $state('');
  let showConfigEditor = $state(false);
  let selectedFindingCategory = $state('All');
  let selectedSeverity = $state('All');
  let isApplyingFixes = $state(false);
  
  // Helper functions for filtering
  function getFilteredFindings(): ComplianceFinding[] {
    if (!report?.findings) return [];
    
    return report.findings.filter(finding => {
      const categoryMatch = selectedFindingCategory === 'All' || finding.category === selectedFindingCategory;
      const severityMatch = selectedSeverity === 'All' || finding.severity === selectedSeverity;
      return categoryMatch && severityMatch;
    });
  }
  
  function getAvailableCategories(): string[] {
    if (!report?.findings) return ['All'];
    const categories = new Set(report.findings.map(f => f.category));
    return ['All', ...Array.from(categories).sort()];
  }
  
  function getAvailableSeverities(): string[] {
    return ['All', 'Critical', 'High', 'Medium', 'Low', 'Info'];
  }
  
  // Load PACS configuration on mount
  onMount(async () => {
    try {
      config = await invoke<PACSConfig>('get_pacs_config');
      selectedProjectPath = '';
    } catch (e) {
      error = `Failed to load PACS config: ${e}`;
    }
  });

  // Open directory picker
  async function selectProjectDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Project Directory for PACS Compliance Scan'
      });
      
      if (selected && typeof selected === 'string') {
        selectedProjectPath = selected;
        error = null;
        report = null; // Clear previous report
      }
    } catch (e) {
      error = `Failed to select directory: ${e}`;
    }
  }
  
  // Run PACS scan with progress updates
  async function runScan() {
    if (!selectedProjectPath) {
      error = 'Please select a project path';
      return;
    }
    
    isScanning = true;
    error = null;
    report = null; // Clear previous report
    scanProgress = 'Initializing scan...';
    
    let progressInterval: number | undefined;
    
    try {
      // Enhanced progress updates with more realistic timing
      const progressSteps = [
        { message: 'Initializing PACS scanner...', duration: 500 },
        { message: 'Inventorying project files...', duration: 1200 },
        { message: 'Analyzing documentation structure...', duration: 800 },
        { message: 'Validating TES-2025 compliance...', duration: 1000 },
        { message: 'Checking EDGS requirements...', duration: 600 },
        { message: 'Validating OpenSpec standards...', duration: 700 },
        { message: 'Detecting specification gaps...', duration: 900 },
        { message: 'Analyzing architectural patterns...', duration: 800 },
        { message: 'Generating compliance recommendations...', duration: 600 },
        { message: 'Calculating compliance score...', duration: 400 },
        { message: 'Finalizing audit report...', duration: 300 }
      ];
      
      let stepIndex = 0;
      
      const updateProgress = () => {
        if (stepIndex < progressSteps.length) {
          const step = progressSteps[stepIndex];
          scanProgress = step.message;
          stepIndex++;
          progressInterval = setTimeout(updateProgress, step.duration);
        }
      };
      
      updateProgress();
      
      // Start the actual scan
      const scanPromise = invoke<ProjectAuditReport>('run_pacs_scan', {
        projectPath: selectedProjectPath,
        config: config
      });
      
      report = await scanPromise;
      
      if (progressInterval) clearTimeout(progressInterval);
      scanProgress = `✅ Scan completed! Found ${report.findings.length} findings with ${report.compliance_score.toFixed(1)}/100 compliance score.`;
      
      // Show success message briefly
      setTimeout(() => { 
        if (!isScanning) scanProgress = ''; 
      }, 3000);
      
    } catch (e) {
      if (progressInterval) clearTimeout(progressInterval);
      error = `Scan failed: ${e}`;
      scanProgress = '';
      console.error('PACS scan error:', e);
    } finally {
      isScanning = false;
    }
  }
  
  // Update PACS configuration
  async function updateConfig() {
    if (!config) return;
    
    try {
      await invoke('update_pacs_config', { config });
      showConfigEditor = false;
      error = null;
    } catch (e) {
      error = `Failed to update config: ${e}`;
    }
  }
  
  // Apply automatic fixes
  async function applyAutoFixes() {
    if (!report || !selectedProjectPath) return;
    
    const autoFixableFindings = report.findings.filter(f => f.auto_fixable);
    if (autoFixableFindings.length === 0) {
      error = 'No auto-fixable issues found';
      return;
    }
    
    isApplyingFixes = true;
    error = null;
    
    try {
      // In a real implementation, this would call a backend command to apply fixes
      // For now, we'll simulate the process
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // Re-run scan after applying fixes
      await runScan();
      
    } catch (e) {
      error = `Failed to apply fixes: ${e}`;
    } finally {
      isApplyingFixes = false;
    }
  }
  
  // Export report
  async function exportReport() {
    if (!report) return;
    
    try {
      const reportJson = JSON.stringify(report, null, 2);
      const blob = new Blob([reportJson], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      
      const a = document.createElement('a');
      a.href = url;
      a.download = `pacs-report-${new Date().toISOString().split('T')[0]}.json`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      
    } catch (e) {
      error = `Failed to export report: ${e}`;
    }
  }
  
  // Get severity color
  function getSeverityColor(severity: string): string {
    switch (severity) {
      case 'Critical': return 'text-red-600 bg-red-50 border-red-200';
      case 'High': return 'text-orange-600 bg-orange-50 border-orange-200';
      case 'Medium': return 'text-yellow-600 bg-yellow-50 border-yellow-200';
      case 'Low': return 'text-blue-600 bg-blue-50 border-blue-200';
      case 'Info': return 'text-gray-800 bg-gray-100 border-gray-300';
      default: return 'text-gray-800 bg-gray-100 border-gray-300';
    }
  }
  
  // Get compliance score color
  function getScoreColor(score: number): string {
    if (score >= 90) return 'text-green-600';
    if (score >= 70) return 'text-yellow-600';
    if (score >= 50) return 'text-orange-600';
    return 'text-red-600';
  }
  
  // Get score background color for progress bars
  function getScoreBackground(score: number): string {
    if (score >= 90) return 'bg-green-500';
    if (score >= 70) return 'bg-yellow-500';
    if (score >= 50) return 'bg-orange-500';
    return 'bg-red-500';
  }
  
  // Format date for display
  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleString();
  }
  
  // Get finding icon
  function getFindingIcon(category: string): string {
    switch (category) {
      case 'MissingDocumentation': return '📝';
      case 'InvalidFormat': return '⚠️';
      case 'ComplianceViolation': return '🚫';
      case 'SecurityIssue': return '🔒';
      case 'ArchitecturalDrift': return '🏗️';
      case 'SpecificationGap': return '📋';
      case 'VersioningIssue': return '🔄';
      case 'MetadataIncomplete': return '📊';
      default: return '🔍';
    }
  }
</script>

<div class="p-6 bg-white rounded-lg shadow-lg max-w-7xl mx-auto">
  <div class="mb-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-gray-900 mb-2">
          Project Auditor & Compliance Scanner (PACS)
        </h2>
        <p class="text-gray-600">
          Deep project analysis and compliance monitoring for TES-2025, EDGS, and OpenSpec standards.
        </p>
      </div>
      <div class="flex gap-3">
        <button
          onclick={() => showConfigEditor = !showConfigEditor}
          class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 flex items-center gap-2"
        >
          ⚙️ Configure
        </button>
        {#if report}
          <button
            onclick={exportReport}
            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 flex items-center gap-2"
          >
            💾 Export Report
          </button>
        {/if}
      </div>
    </div>
  </div>
  
  <!-- Configuration Editor -->
  {#if showConfigEditor && config}
    <div class="mb-6 p-6 bg-gray-50 border-2 border-gray-200 rounded-lg">
      <h3 class="text-xl font-bold text-gray-900 mb-4">⚙️ PACS Configuration</h3>
      <div class="grid grid-cols-2 gap-6">
        <div class="space-y-4">
          <div>
            <label class="flex items-center gap-2">
              <input
                type="checkbox"
                bind:checked={config.auto_generate_specs}
                class="rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
              />
              <span class="font-medium text-gray-900">Auto Generate Specs</span>
            </label>
          </div>
          <div>
            <label class="flex items-center gap-2">
              <input
                type="checkbox"
                bind:checked={config.auto_create_beads}
                class="rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
              />
              <span class="font-medium text-gray-900">Auto Create Beads</span>
            </label>
          </div>
          <div>
            <label for="max-depth" class="block text-sm font-medium text-gray-700 mb-1">Max Scan Depth</label>
            <input
              id="max-depth"
              type="number"
              bind:value={config.max_depth}
              min="1"
              max="20"
              class="w-24 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
          </div>
        </div>
        <div>
          <label for="output-dir" class="block text-sm font-medium text-gray-700 mb-2">Output Directory</label>
          <input
            id="output-dir"
            type="text"
            bind:value={config.output_dir}
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
          />
        </div>
      </div>
      <div class="mt-4 flex gap-3">
        <button
          onclick={updateConfig}
          class="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
        >
          Save Configuration
        </button>
        <button
          onclick={() => showConfigEditor = false}
          class="px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400"
        >
          Cancel
        </button>
      </div>
    </div>
  {/if}
  
  <!-- Configuration Display (when not editing) -->
  {#if config && !showConfigEditor}
    <div class="mb-6 p-6 bg-white border-2 border-gray-200 rounded-lg shadow-sm">
      <h3 class="text-xl font-bold text-gray-900 mb-4">⚙️ Current Configuration</h3>
      <div class="grid grid-cols-2 gap-6 text-base">
        <div>
          <span class="font-bold text-gray-900">Standards:</span>
          <div class="ml-2 text-gray-900 bg-blue-50 px-3 py-2 rounded border font-medium mt-1">
            {config.standards.join(', ')}
          </div>
        </div>
        <div class="space-y-3">
          <div>
            <span class="font-bold text-gray-900">Max Depth:</span>
            <span class="ml-2 text-gray-900 bg-gray-100 px-2 py-1 rounded font-mono">{config.max_depth}</span>
          </div>
          <div>
            <span class="font-bold text-gray-900">Auto Generate Specs:</span>
            <span class="ml-2 text-gray-900 bg-gray-100 px-2 py-1 rounded font-medium">{config.auto_generate_specs ? 'Yes' : 'No'}</span>
          </div>
          <div>
            <span class="font-bold text-gray-900">Auto Create Beads:</span>
            <span class="ml-2 text-gray-900 bg-gray-100 px-2 py-1 rounded font-medium">{config.auto_create_beads ? 'Yes' : 'No'}</span>
          </div>
        </div>
      </div>
    </div>
  {/if}
  
  <!-- Project Selection and Scan Controls -->
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
        onclick={selectProjectDirectory}
        class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 flex items-center gap-2"
      >
        📁 Browse
      </button>
      <button
        onclick={runScan}
        disabled={isScanning || !selectedProjectPath}
        class="px-6 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      >
        {#if isScanning}
          <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
          Scanning...
        {:else}
          🚀 Run PACS Scan
        {/if}
      </button>
    </div>
    
    <!-- Progress Display -->
    {#if scanProgress}
      <div class="mt-3 p-3 bg-blue-50 border border-blue-200 rounded-lg">
        <div class="flex items-center gap-2">
          <div class="w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
          <span class="text-blue-800 font-medium">{scanProgress}</span>
        </div>
     </div>
   {/if}
 </div>
 
 <!-- Baseline Management Section -->
 {#if selectedProjectPath}
   <div class="mt-8">
     <PACSBaselineManager projectPath={selectedProjectPath} currentReport={report} />
   </div>
 {/if}
  
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
  {#if report}
    <div class="space-y-6">
      <!-- Compliance Score and Quick Actions -->
      <div class="bg-gradient-to-r from-indigo-50 to-blue-50 p-6 rounded-lg">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-lg font-semibold text-gray-900">Compliance Score</h3>
            <p class="text-sm text-gray-600">Overall project compliance rating</p>
            <div class="mt-2 w-64 bg-gray-200 rounded-full h-3">
              <div 
                class="h-3 rounded-full transition-all duration-500 {getScoreBackground(report.compliance_score)}"
                style="width: {report.compliance_score}%"
              ></div>
            </div>
          </div>
          <div class="text-right">
            <div class="text-3xl font-bold {getScoreColor(report.compliance_score)}">
              {report.compliance_score.toFixed(1)}/100
            </div>
            <div class="text-sm text-gray-600 mb-3">
              {report.auto_fixes_available} auto-fixes available
            </div>
            {#if report.auto_fixes_available > 0}
              <button
                onclick={applyAutoFixes}
                disabled={isApplyingFixes}
                class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50 flex items-center gap-2"
              >
                {#if isApplyingFixes}
                  <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                  Applying...
                {:else}
                  🔧 Apply Auto-Fixes
                {/if}
              </button>
            {/if}
          </div>
        </div>
      </div>
      
      <!-- Standards Summary -->
      {#if Object.keys(report.standards_summary).length > 0}
        <div class="bg-white p-6 rounded-lg border border-gray-200">
          <h3 class="text-lg font-semibold text-gray-900 mb-4">Standards Compliance</h3>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            {#each Object.entries(report.standards_summary) as [standard, status]}
              <div class="p-4 border rounded-lg {status.compliant ? 'border-green-200 bg-green-50' : 'border-red-200 bg-red-50'}">
                <div class="flex items-center justify-between mb-2">
                  <span class="font-medium text-gray-900">{standard}</span>
                  <span class="text-2xl">{status.compliant ? '✅' : '❌'}</span>
                </div>
                <div class="text-sm text-gray-600">
                  <div>Score: {status.score.toFixed(1)}/100</div>
                  <div>Issues: {status.issues_count}</div>
                  {#if status.critical_issues > 0}
                    <div class="text-red-600 font-medium">Critical: {status.critical_issues}</div>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}
      
      <!-- Quick Stats -->
      <div class="grid grid-cols-4 gap-4">
        <div class="bg-white p-4 rounded-lg border border-gray-200">
          <div class="text-2xl font-bold text-gray-900">{report.findings.length}</div>
          <div class="text-sm text-gray-600">Total Findings</div>
        </div>
        <div class="bg-white p-4 rounded-lg border border-gray-200">
          <div class="text-2xl font-bold text-red-600">
            {report.findings.filter(f => f.severity === 'Critical').length}
          </div>
          <div class="text-sm text-gray-600">Critical Issues</div>
        </div>
        <div class="bg-white p-4 rounded-lg border border-gray-200">
          <div class="text-2xl font-bold text-orange-600">
            {report.findings.filter(f => f.severity === 'High').length}
          </div>
          <div class="text-sm text-gray-600">High Priority</div>
        </div>
        <div class="bg-white p-4 rounded-lg border border-gray-200">
          <div class="text-2xl font-bold text-green-600">{report.auto_fixes_available}</div>
          <div class="text-sm text-gray-600">Auto-Fixable</div>
        </div>
      </div>
      
      <!-- Recommendations -->
      {#if report.recommendations.length > 0}
        <div class="bg-blue-50 p-4 rounded-lg">
          <h3 class="text-lg font-semibold text-blue-900 mb-3">📋 Recommendations</h3>
          <ul class="space-y-2">
            {#each report.recommendations as recommendation}
              <li class="flex items-start gap-2">
                <div class="w-2 h-2 bg-blue-500 rounded-full mt-2 flex-shrink-0"></div>
                <span class="text-blue-800">{recommendation}</span>
              </li>
            {/each}
          </ul>
        </div>
      {/if}
      
      <!-- Findings List with Filters -->
      {#if report.findings.length > 0}
        <div>
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-semibold text-gray-900">Compliance Findings</h3>
            <div class="flex gap-3">
              <select
                bind:value={selectedFindingCategory}
                class="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
              >
                {#each getAvailableCategories() as category}
                  <option value={category}>{category}</option>
                {/each}
              </select>
              <select
                bind:value={selectedSeverity}
                class="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
              >
                {#each getAvailableSeverities() as severity}
                  <option value={severity}>{severity}</option>
                {/each}
              </select>
            </div>
          </div>
          
          <div class="space-y-3">
            {#each getFilteredFindings() as finding}
              <div class="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
                <div class="flex items-start justify-between mb-2">
                  <div class="flex items-center gap-3">
                    <span class="text-2xl">{getFindingIcon(finding.category)}</span>
                    <span class="px-2 py-1 text-xs font-medium rounded-full border {getSeverityColor(finding.severity)}">
                      {finding.severity}
                    </span>
                    <span class="text-sm text-gray-600">{finding.standard}</span>
                  </div>
                  {#if finding.auto_fixable}
                    <span class="px-2 py-1 text-xs font-medium text-green-700 bg-green-100 rounded-full">
                      🔧 Auto-fixable
                    </span>
                  {/if}
                </div>
                
                <h4 class="font-medium text-gray-900 mb-1">{finding.title}</h4>
                <p class="text-sm text-gray-600 mb-2">{finding.description}</p>
                
                {#if finding.file_path}
                  <div class="text-xs text-gray-700 mb-2 font-mono bg-gray-100 px-2 py-1 rounded">
                    📁 {finding.file_path}
                    {#if finding.line_number}
                      : line {finding.line_number}
                    {/if}
                  </div>
                {/if}
                
                <div class="text-sm text-indigo-700 bg-indigo-50 p-2 rounded border border-indigo-200">
                  <strong>💡 Recommendation:</strong> {finding.recommendation}
                </div>
                
                <div class="text-xs text-gray-500 mt-2">
                  Detected: {formatDate(finding.detected_at)}
                </div>
              </div>
            {/each}
          </div>
          
          {#if getFilteredFindings().length === 0 && report.findings.length > 0}
            <div class="text-center py-8 text-gray-500">
              No findings match the selected filters.
            </div>
          {/if}
        </div>
      {:else}
        <div class="text-center py-8 bg-green-50 rounded-lg border border-green-200">
          <div class="text-4xl mb-2">🎉</div>
          <div class="text-lg font-semibold text-green-800">Perfect Compliance!</div>
          <div class="text-green-600">No compliance issues found in this project.</div>
        </div>
      {/if}
      
      <!-- Scan Metadata -->
      <div class="text-xs text-gray-700 pt-4 border-t border-gray-200 bg-gray-50 p-4 rounded">
        <div class="grid grid-cols-3 gap-4">
          <div>
            <strong>Scanned:</strong> {formatDate(report.scanned_at)}
          </div>
          <div>
            <strong>Scanner Version:</strong> {report.scanner_version}
          </div>
          <div>
            <strong>Project:</strong> {report.project_path}
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>