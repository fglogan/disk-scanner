<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  
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
  
  interface ProjectAuditReport {
    project_path: string;
    scanned_at: string;
    scanner_version: string;
    compliance_score: number;
    findings: ComplianceFinding[];
    baseline?: any;
    recommendations: string[];
    auto_fixes_available: number;
    standards_summary: Record<string, any>;
  }
  
  // Component state
  let isScanning = $state(false);
  let report: ProjectAuditReport | null = $state(null);
  let error: string | null = $state(null);
  let config: PACSConfig | null = $state(null);
  let selectedProjectPath = $state('');
  
  // Load PACS configuration on mount
  onMount(async () => {
    try {
      config = await invoke<PACSConfig>('get_pacs_config');
      // Default to current project directory
      selectedProjectPath = '/Users/tempext/Projects/disk-bloat-scanner';
    } catch (e) {
      error = `Failed to load PACS config: ${e}`;
    }
  });
  
  // Run PACS scan
  async function runScan() {
    if (!selectedProjectPath) {
      error = 'Please select a project path';
      return;
    }
    
    isScanning = true;
    error = null;
    
    try {
      report = await invoke<ProjectAuditReport>('run_pacs_scan', {
        projectPath: selectedProjectPath,
        config: config
      });
    } catch (e) {
      error = `Scan failed: ${e}`;
    } finally {
      isScanning = false;
    }
  }
  
  // Get severity color
  function getSeverityColor(severity: string): string {
    switch (severity) {
      case 'Critical': return 'text-red-600 bg-red-50';
      case 'High': return 'text-orange-600 bg-orange-50';
      case 'Medium': return 'text-yellow-600 bg-yellow-50';
      case 'Low': return 'text-blue-600 bg-blue-50';
      case 'Info': return 'text-gray-600 bg-gray-50';
      default: return 'text-gray-600 bg-gray-50';
    }
  }
  
  // Get compliance score color
  function getScoreColor(score: number): string {
    if (score >= 90) return 'text-green-600';
    if (score >= 70) return 'text-yellow-600';
    if (score >= 50) return 'text-orange-600';
    return 'text-red-600';
  }
</script>

<div class="p-6 bg-white rounded-lg shadow-lg">
  <div class="mb-6">
    <h2 class="text-2xl font-bold text-gray-900 mb-2">
      Project Auditor & Compliance Scanner (PACS)
    </h2>
    <p class="text-gray-600">
      Deep project analysis and compliance monitoring for TES-2025, EDGS, and OpenSpec standards.
    </p>
  </div>
  
  <!-- Configuration Section -->
  {#if config}
    <div class="mb-6 p-4 bg-gray-50 rounded-lg">
      <h3 class="text-lg font-semibold mb-3">Configuration</h3>
      <div class="grid grid-cols-2 gap-4 text-sm">
        <div>
          <span class="font-medium">Standards:</span>
          <span class="ml-2">{config.standards.join(', ')}</span>
        </div>
        <div>
          <span class="font-medium">Max Depth:</span>
          <span class="ml-2">{config.max_depth}</span>
        </div>
        <div>
          <span class="font-medium">Auto Generate Specs:</span>
          <span class="ml-2">{config.auto_generate_specs ? 'Yes' : 'No'}</span>
        </div>
        <div>
          <span class="font-medium">Auto Create Beads:</span>
          <span class="ml-2">{config.auto_create_beads ? 'Yes' : 'No'}</span>
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
        onclick={runScan}
        disabled={isScanning || !selectedProjectPath}
        class="px-6 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      >
        {#if isScanning}
          <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
          Scanning...
        {:else}
          Run PACS Scan
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
  {#if report}
    <div class="space-y-6">
      <!-- Compliance Score -->
      <div class="bg-gradient-to-r from-indigo-50 to-blue-50 p-6 rounded-lg">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-lg font-semibold text-gray-900">Compliance Score</h3>
            <p class="text-sm text-gray-600">Overall project compliance rating</p>
          </div>
          <div class="text-right">
            <div class="text-3xl font-bold {getScoreColor(report.compliance_score)}">
              {report.compliance_score.toFixed(1)}/100
            </div>
            <div class="text-sm text-gray-600">
              {report.auto_fixes_available} auto-fixes available
            </div>
          </div>
        </div>
      </div>
      
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
          <h3 class="text-lg font-semibold text-blue-900 mb-3">Recommendations</h3>
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
      
      <!-- Findings List -->
      {#if report.findings.length > 0}
        <div>
          <h3 class="text-lg font-semibold text-gray-900 mb-4">Compliance Findings</h3>
          <div class="space-y-3">
            {#each report.findings as finding}
              <div class="border border-gray-200 rounded-lg p-4">
                <div class="flex items-start justify-between mb-2">
                  <div class="flex items-center gap-3">
                    <span class="px-2 py-1 text-xs font-medium rounded-full {getSeverityColor(finding.severity)}">
                      {finding.severity}
                    </span>
                    <span class="text-sm text-gray-600">{finding.standard}</span>
                  </div>
                  {#if finding.auto_fixable}
                    <span class="px-2 py-1 text-xs font-medium text-green-700 bg-green-100 rounded-full">
                      Auto-fixable
                    </span>
                  {/if}
                </div>
                
                <h4 class="font-medium text-gray-900 mb-1">{finding.title}</h4>
                <p class="text-sm text-gray-600 mb-2">{finding.description}</p>
                
                {#if finding.file_path}
                  <div class="text-xs text-gray-500 mb-2">
                    📁 {finding.file_path}
                    {#if finding.line_number}
                      : line {finding.line_number}
                    {/if}
                  </div>
                {/if}
                
                <div class="text-sm text-indigo-700 bg-indigo-50 p-2 rounded">
                  <strong>Recommendation:</strong> {finding.recommendation}
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}
      
      <!-- Scan Metadata -->
      <div class="text-xs text-gray-500 pt-4 border-t border-gray-200">
        <div>Scanned: {new Date(report.scanned_at).toLocaleString()}</div>
        <div>Scanner Version: {report.scanner_version}</div>
        <div>Project: {report.project_path}</div>
      </div>
    </div>
  {/if}
</div>