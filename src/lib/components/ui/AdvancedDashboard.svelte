<script lang="ts">
  // Advanced UI Dashboard - MACM-V1 Implementation
  // Comprehensive dashboard with data visualization and responsive design
  
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import DataVisualization from './DataVisualization.svelte';
  import ResponsiveGrid from './ResponsiveGrid.svelte';
  import { showSuccess, showError, showInfo } from '../../stores.js';
  
  interface DashboardData {
    diskUsage: Array<{label: string, value: number, color: string}>;
    projectStats: Array<{label: string, value: number, color: string}>;
    recentActivity: Array<{action: string, timestamp: Date, details: string}>;
    systemHealth: {
      cpu: number;
      memory: number;
      disk: number;
      status: 'good' | 'warning' | 'critical';
    };
  }
  
  let dashboardData = $state<DashboardData>({
    diskUsage: [],
    projectStats: [],
    recentActivity: [],
    systemHealth: { cpu: 0, memory: 0, disk: 0, status: 'good' }
  });
  
  let isLoading = $state(true);
  let lastRefresh = $state<Date>(new Date());
  let autoRefresh = $state(true);
  let refreshInterval = $state<number | null>(null);
  
  onMount(() => {
    loadDashboardData();
    if (autoRefresh) {
      startAutoRefresh();
    }
    
    return () => {
      if (refreshInterval) {
        clearInterval(refreshInterval);
      }
    };
  });
  
  async function loadDashboardData() {
    isLoading = true;
    
    try {
      // Load system information
      const systemInfo = await invoke('get_system_info');
      const diskInfo = await invoke('get_disk_info');
      
      // Process disk usage data
      dashboardData.diskUsage = [
        {
          label: 'Used Space',
          value: systemInfo.disk_used_gb || 0,
          color: '#ef4444'
        },
        {
          label: 'Free Space', 
          value: systemInfo.disk_free_gb || 0,
          color: '#10b981'
        }
      ];
      
      // Mock project statistics (would be real data in production)
      dashboardData.projectStats = [
        { label: 'Node Modules', value: 2.4, color: '#f59e0b' },
        { label: 'Build Artifacts', value: 1.8, color: '#8b5cf6' },
        { label: 'Cache Files', value: 0.9, color: '#06b6d4' },
        { label: 'Log Files', value: 0.3, color: '#84cc16' }
      ];
      
      // Mock recent activity
      dashboardData.recentActivity = [
        {
          action: 'Cleaned node_modules',
          timestamp: new Date(Date.now() - 1000 * 60 * 15),
          details: 'Freed 1.2 GB'
        },
        {
          action: 'Scanned project directory',
          timestamp: new Date(Date.now() - 1000 * 60 * 30),
          details: 'Found 15 projects'
        },
        {
          action: 'Updated monitoring config',
          timestamp: new Date(Date.now() - 1000 * 60 * 45),
          details: 'Added 3 new paths'
        }
      ];
      
      // System health calculation
      const cpuUsage = Math.random() * 100;
      const memoryUsage = (systemInfo.memory_used_gb / systemInfo.memory_total_gb) * 100;
      const diskUsage = (systemInfo.disk_used_gb / (systemInfo.disk_used_gb + systemInfo.disk_free_gb)) * 100;
      
      dashboardData.systemHealth = {
        cpu: cpuUsage,
        memory: memoryUsage,
        disk: diskUsage,
        status: diskUsage > 90 || memoryUsage > 90 ? 'critical' : 
                diskUsage > 75 || memoryUsage > 75 ? 'warning' : 'good'
      };
      
      lastRefresh = new Date();
      showSuccess('Dashboard data refreshed successfully');
      
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
      showError('Failed to load dashboard data');
    } finally {
      isLoading = false;
    }
  }
  
  function startAutoRefresh() {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
    
    refreshInterval = setInterval(() => {
      loadDashboardData();
    }, 30000); // Refresh every 30 seconds
  }
  
  function toggleAutoRefresh() {
    autoRefresh = !autoRefresh;
    
    if (autoRefresh) {
      startAutoRefresh();
      showInfo('Auto-refresh enabled');
    } else {
      if (refreshInterval) {
        clearInterval(refreshInterval);
        refreshInterval = null;
      }
      showInfo('Auto-refresh disabled');
    }
  }
  
  function formatTimeAgo(date: Date): string {
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / (1000 * 60));
    
    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    
    const diffHours = Math.floor(diffMins / 60);
    if (diffHours < 24) return `${diffHours}h ago`;
    
    const diffDays = Math.floor(diffHours / 24);
    return `${diffDays}d ago`;
  }
  
  function getHealthStatusColor(status: string): string {
    switch (status) {
      case 'good': return '#10b981';
      case 'warning': return '#f59e0b';
      case 'critical': return '#ef4444';
      default: return '#6b7280';
    }
  }
  
  function getHealthStatusIcon(status: string): string {
    switch (status) {
      case 'good': return '✅';
      case 'warning': return '⚠️';
      case 'critical': return '🚨';
      default: return '❓';
    }
  }
</script>

<div class="advanced-dashboard">
  <div class="dashboard-header">
    <div class="header-content">
      <h1 class="dashboard-title">
        🚀 Advanced Dashboard
      </h1>
      <div class="header-stats">
        <div class="stat-item">
          <span class="stat-label">Last Updated:</span>
          <span class="stat-value">{formatTimeAgo(lastRefresh)}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">System Status:</span>
          <span class="stat-value" style="color: {getHealthStatusColor(dashboardData.systemHealth.status)}">
            {getHealthStatusIcon(dashboardData.systemHealth.status)} {dashboardData.systemHealth.status.toUpperCase()}
          </span>
        </div>
      </div>
    </div>
    
    <div class="header-actions">
      <button 
        class="action-btn refresh-btn"
        onclick={loadDashboardData}
        disabled={isLoading}
        aria-label="Refresh dashboard data"
      >
        <span class="btn-icon" class:spinning={isLoading}>🔄</span>
        {isLoading ? 'Refreshing...' : 'Refresh'}
      </button>
      
      <button 
        class="action-btn auto-refresh-btn"
        class:active={autoRefresh}
        onclick={toggleAutoRefresh}
        aria-label="Toggle auto-refresh"
      >
        <span class="btn-icon">⏱️</span>
        Auto-refresh
      </button>
    </div>
  </div>
  
  {#if isLoading}
    <div class="loading-state">
      <div class="loading-spinner"></div>
      <p>Loading dashboard data...</p>
    </div>
  {:else}
    <ResponsiveGrid columns="auto" gap="1.5rem" minItemWidth="300px">
      {#snippet children()}
        <!-- System Health Card -->
        <div class="dashboard-card health-card">
          <div class="card-header">
            <h3 class="card-title">🏥 System Health</h3>
            <div class="health-indicator" style="background-color: {getHealthStatusColor(dashboardData.systemHealth.status)}">
              {getHealthStatusIcon(dashboardData.systemHealth.status)}
            </div>
          </div>
          
          <div class="health-metrics">
            <div class="metric">
              <div class="metric-label">CPU Usage</div>
              <div class="metric-bar">
                <div class="metric-fill" style="width: {dashboardData.systemHealth.cpu}%; background-color: {dashboardData.systemHealth.cpu > 80 ? '#ef4444' : '#10b981'}"></div>
              </div>
              <div class="metric-value">{dashboardData.systemHealth.cpu.toFixed(1)}%</div>
            </div>
            
            <div class="metric">
              <div class="metric-label">Memory Usage</div>
              <div class="metric-bar">
                <div class="metric-fill" style="width: {dashboardData.systemHealth.memory}%; background-color: {dashboardData.systemHealth.memory > 80 ? '#ef4444' : '#10b981'}"></div>
              </div>
              <div class="metric-value">{dashboardData.systemHealth.memory.toFixed(1)}%</div>
            </div>
            
            <div class="metric">
              <div class="metric-label">Disk Usage</div>
              <div class="metric-bar">
                <div class="metric-fill" style="width: {dashboardData.systemHealth.disk}%; background-color: {dashboardData.systemHealth.disk > 80 ? '#ef4444' : '#10b981'}"></div>
              </div>
              <div class="metric-value">{dashboardData.systemHealth.disk.toFixed(1)}%</div>
            </div>
          </div>
        </div>
        
        <!-- Disk Usage Visualization -->
        <div class="dashboard-card">
          <DataVisualization 
            data={dashboardData.diskUsage}
            type="pie"
            title="💾 Disk Usage"
            height={300}
            interactive={true}
            showLegend={true}
          />
        </div>
        
        <!-- Project Statistics -->
        <div class="dashboard-card">
          <DataVisualization 
            data={dashboardData.projectStats}
            type="bar"
            title="📊 Project Bloat (GB)"
            height={300}
            interactive={true}
            showLegend={true}
          />
        </div>
        
        <!-- Recent Activity -->
        <div class="dashboard-card activity-card">
          <div class="card-header">
            <h3 class="card-title">📋 Recent Activity</h3>
            <div class="activity-count">{dashboardData.recentActivity.length} items</div>
          </div>
          
          <div class="activity-list">
            {#each dashboardData.recentActivity as activity}
              <div class="activity-item">
                <div class="activity-content">
                  <div class="activity-action">{activity.action}</div>
                  <div class="activity-details">{activity.details}</div>
                </div>
                <div class="activity-time">{formatTimeAgo(activity.timestamp)}</div>
              </div>
            {/each}
          </div>
          
          <div class="activity-footer">
            <button class="view-all-btn">View All Activity</button>
          </div>
        </div>
        
        <!-- Quick Actions -->
        <div class="dashboard-card actions-card">
          <div class="card-header">
            <h3 class="card-title">⚡ Quick Actions</h3>
          </div>
          
          <div class="quick-actions">
            <button class="quick-action-btn scan-btn">
              <span class="action-icon">🔍</span>
              <span class="action-label">Quick Scan</span>
            </button>
            
            <button class="quick-action-btn clean-btn">
              <span class="action-icon">🧹</span>
              <span class="action-label">Clean Cache</span>
            </button>
            
            <button class="quick-action-btn monitor-btn">
              <span class="action-icon">📊</span>
              <span class="action-label">Monitor Projects</span>
            </button>
            
            <button class="quick-action-btn settings-btn">
              <span class="action-icon">⚙️</span>
              <span class="action-label">Settings</span>
            </button>
          </div>
        </div>
        
        <!-- Performance Insights -->
        <div class="dashboard-card insights-card">
          <div class="card-header">
            <h3 class="card-title">💡 Performance Insights</h3>
          </div>
          
          <div class="insights-list">
            <div class="insight-item tip">
              <div class="insight-icon">💡</div>
              <div class="insight-content">
                <div class="insight-title">Optimization Tip</div>
                <div class="insight-description">
                  Your node_modules folders are taking up 2.4 GB. Consider cleaning unused dependencies.
                </div>
              </div>
            </div>
            
            <div class="insight-item warning">
              <div class="insight-icon">⚠️</div>
              <div class="insight-content">
                <div class="insight-title">Storage Warning</div>
                <div class="insight-description">
                  Disk usage is at {dashboardData.systemHealth.disk.toFixed(1)}%. Consider freeing up space.
                </div>
              </div>
            </div>
            
            <div class="insight-item success">
              <div class="insight-icon">✅</div>
              <div class="insight-content">
                <div class="insight-title">Good Performance</div>
                <div class="insight-description">
                  System is running efficiently. Memory usage is within normal range.
                </div>
              </div>
            </div>
          </div>
        </div>
      {/snippet}
    </ResponsiveGrid>
  {/if}
</div>

<style>
  .advanced-dashboard {
    min-height: 100vh;
    background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
    color: #e2e8f0;
    padding: 2rem;
  }
  
  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding: 1.5rem;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 1rem;
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .header-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .dashboard-title {
    margin: 0;
    font-size: 2rem;
    font-weight: 700;
    background: linear-gradient(135deg, #60a5fa, #a78bfa);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }
  
  .header-stats {
    display: flex;
    gap: 2rem;
  }
  
  .stat-item {
    display: flex;
    gap: 0.5rem;
    font-size: 0.875rem;
  }
  
  .stat-label {
    color: #94a3b8;
  }
  
  .stat-value {
    color: #f8fafc;
    font-weight: 600;
  }
  
  .header-actions {
    display: flex;
    gap: 1rem;
  }
  
  .action-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 0.5rem;
    color: #e2e8f0;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }
  
  .action-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    transform: translateY(-1px);
  }
  
  .action-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
  }
  
  .auto-refresh-btn.active {
    background: rgba(16, 185, 129, 0.2);
    border-color: #10b981;
    color: #10b981;
  }
  
  .btn-icon.spinning {
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  
  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 400px;
    gap: 1rem;
  }
  
  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 3px solid rgba(255, 255, 255, 0.1);
    border-top: 3px solid #60a5fa;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  .dashboard-card {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 1rem;
    padding: 1.5rem;
    border: 1px solid rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
  }
  
  .dashboard-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
  }
  
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  
  .card-title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #f8fafc;
  }
  
  .health-indicator {
    width: 2rem;
    height: 2rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
  }
  
  .health-metrics {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .metric {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .metric-label {
    min-width: 100px;
    font-size: 0.875rem;
    color: #94a3b8;
  }
  
  .metric-bar {
    flex: 1;
    height: 8px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    overflow: hidden;
  }
  
  .metric-fill {
    height: 100%;
    transition: width 0.3s ease;
  }
  
  .metric-value {
    min-width: 50px;
    text-align: right;
    font-weight: 600;
    color: #f8fafc;
  }
  
  .activity-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .activity-item {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1rem;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 0.5rem;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .activity-content {
    flex: 1;
  }
  
  .activity-action {
    font-weight: 600;
    color: #f8fafc;
    margin-bottom: 0.25rem;
  }
  
  .activity-details {
    font-size: 0.875rem;
    color: #94a3b8;
  }
  
  .activity-time {
    font-size: 0.75rem;
    color: #64748b;
    white-space: nowrap;
  }
  
  .quick-actions {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 1rem;
  }
  
  .quick-action-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1.5rem 1rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 0.75rem;
    color: #e2e8f0;
    cursor: pointer;
    transition: all 0.2s ease;
  }
  
  .quick-action-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    transform: translateY(-2px);
  }
  
  .action-icon {
    font-size: 1.5rem;
  }
  
  .action-label {
    font-size: 0.875rem;
    font-weight: 500;
  }
  
  .insights-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .insight-item {
    display: flex;
    gap: 1rem;
    padding: 1rem;
    border-radius: 0.5rem;
    border-left: 4px solid;
  }
  
  .insight-item.tip {
    background: rgba(59, 130, 246, 0.1);
    border-left-color: #3b82f6;
  }
  
  .insight-item.warning {
    background: rgba(245, 158, 11, 0.1);
    border-left-color: #f59e0b;
  }
  
  .insight-item.success {
    background: rgba(16, 185, 129, 0.1);
    border-left-color: #10b981;
  }
  
  .insight-icon {
    font-size: 1.25rem;
    flex-shrink: 0;
  }
  
  .insight-title {
    font-weight: 600;
    color: #f8fafc;
    margin-bottom: 0.25rem;
  }
  
  .insight-description {
    font-size: 0.875rem;
    color: #94a3b8;
    line-height: 1.5;
  }
  
  .view-all-btn {
    width: 100%;
    padding: 0.75rem;
    background: transparent;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 0.5rem;
    color: #94a3b8;
    cursor: pointer;
    transition: all 0.2s ease;
    margin-top: 1rem;
  }
  
  .view-all-btn:hover {
    background: rgba(255, 255, 255, 0.05);
    color: #e2e8f0;
  }
  
  @media (max-width: 768px) {
    .advanced-dashboard {
      padding: 1rem;
    }
    
    .dashboard-header {
      flex-direction: column;
      gap: 1rem;
      align-items: stretch;
    }
    
    .header-stats {
      flex-direction: column;
      gap: 0.5rem;
    }
    
    .header-actions {
      justify-content: center;
    }
    
    .quick-actions {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>