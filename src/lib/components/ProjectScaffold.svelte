<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	interface ProjectConfig {
		name: string;
		template: 'tauri' | 'svelte' | 'vite-react' | 'full-stack';
		backend: 'rust' | 'node';
		db: 'sqlite' | 'postgres' | 'mongodb' | 'none';
		css: 'tailwind' | 'vanilla-css';
		pm: 'npm' | 'yarn' | 'pnpm';
	}

	interface HealthReport {
		score: number; // 0-100
		status: 'excellent' | 'good' | 'warning' | 'critical';
		issues: string[];
		recommendations: string[];
		lastChecked: Date;
	}

	interface FileEntry {
		path: string;
		status: 'present' | 'missing' | 'malformed';
		severity: 'critical' | 'warning' | 'info';
		description: string;
	}

	let activeTab: string = 'health';
	let projectConfig: ProjectConfig = {
		name: 'my-tauri-app',
		template: 'tauri',
		backend: 'rust',
		db: 'sqlite',
		css: 'tailwind',
		pm: 'npm'
	};

	let healthReport: HealthReport | null = null;
	let isScanning: boolean = false;
	let isCreating: boolean = false;
	let creationProgress: string = '';

	let fileChecks: FileEntry[] = [];
	let repairSuggestions: string[] = [];
	let isAutoRepairing: boolean = false;

	// Health check status indicators
	const criticalFiles = [
		{ path: 'Cargo.toml', category: 'Rust Backend' },
		{ path: 'package.json', category: 'Frontend' },
		{ path: 'tsconfig.json', category: 'TypeScript' },
		{ path: 'tauri.conf.json', category: 'Tauri Config' },
		{ path: 'src-tauri/src/main.rs', category: 'Rust Entry' },
		{ path: 'src/App.svelte', category: 'Frontend Entry' },
		{ path: '.opencode/agents.json', category: 'OpenCode' },
		{ path: 'docs/ARCHITECTURE.md', category: 'Documentation' },
		{ path: '.beads/config.json', category: 'Beads Tracking' }
	];

	const requiredDirs = [
		{ path: 'src', description: 'Frontend source code' },
		{ path: 'src-tauri/src', description: 'Rust backend code' },
		{ path: 'docs', description: 'Project documentation' },
		{ path: '.opencode', description: 'OpenCode configuration' },
		{ path: '.beads', description: 'Beads issue tracking' }
	];

	async function scanProjectHealth(): Promise<void> {
		isScanning = true;
		try {
			const report = await invoke<HealthReport>('check_project_health');
			healthReport = report;

			// Parse file structure
			await checkFileStructure();
		} catch (error) {
			console.error('Health check failed:', error);
			healthReport = {
				score: 0,
				status: 'critical',
				issues: ['Failed to scan project health'],
				recommendations: ['Check that you are in a valid Tauri project directory'],
				lastChecked: new Date()
			};
		} finally {
			isScanning = false;
		}
	}

	async function checkFileStructure(): Promise<void> {
		fileChecks = [];
		for (const file of criticalFiles) {
			try {
				const result = await invoke<{ exists: boolean; valid: boolean }>('check_file_structure', {
					filePath: file.path
				});

				fileChecks.push({
					path: file.path,
					status: result.exists ? (result.valid ? 'present' : 'malformed') : 'missing',
					severity: result.exists ? (result.valid ? 'info' : 'warning') : 'critical',
					description: file.category
				});
			} catch (error) {
				fileChecks.push({
					path: file.path,
					status: 'missing',
					severity: 'critical',
					description: file.category
				});
			}
		}
	}

	async function createNewProject(): Promise<void> {
		if (!projectConfig.name.trim()) {
			alert('Please enter a project name');
			return;
		}

		isCreating = true;
		creationProgress = 'Initializing project...';

		try {
			// Step 1: Create base Tauri project
			creationProgress = '📦 Creating Tauri 2.8.x project structure...';
			await invoke('create_tauri_project', {
				projectName: projectConfig.name,
				template: projectConfig.template
			});

			// Step 2: Setup Rust backend
			creationProgress = '🦀 Configuring Rust backend (Edition 2024)...';
			await invoke('setup_rust_backend', {
				projectName: projectConfig.name,
				dbType: projectConfig.db
			});

			// Step 3: Setup Frontend
			creationProgress = '🎨 Setting up frontend with Svelte + TypeScript...';
			await invoke('setup_frontend', {
				projectName: projectConfig.name,
				cssFramework: projectConfig.css
			});

			// Step 4: Install dependencies
			creationProgress = '📚 Installing dependencies...';
			await invoke('install_dependencies', {
				projectName: projectConfig.name,
				packageManager: projectConfig.pm
			});

			// Step 5: Setup Tailwind if selected
			if (projectConfig.css === 'tailwind') {
				creationProgress = '🎨 Configuring Tailwind CSS...';
				await invoke('setup_tailwind', {
					projectName: projectConfig.name
				});
			}

			// Step 6: Setup database
			if (projectConfig.db !== 'none') {
				creationProgress = `🗄️  Setting up ${projectConfig.db} database...`;
				await invoke('setup_database', {
					projectName: projectConfig.name,
					dbType: projectConfig.db
				});
			}

			// Step 7: Setup OpenCode
			creationProgress = '🔧 Initializing OpenCode environment...';
			await invoke('init_opencode', {
				projectName: projectConfig.name
			});

			// Step 8: Setup Beads
			creationProgress = '📋 Initializing Beads issue tracking...';
			await invoke('init_beads', {
				projectName: projectConfig.name
			});

			// Step 9: Setup OpenSpec
			creationProgress = '📊 Initializing OpenSpec workflow...';
			await invoke('init_openspec', {
				projectName: projectConfig.name
			});

			// Step 10: Generate documentation
			creationProgress = '📚 Generating compliance documentation...';
			await invoke('generate_docs', {
				projectName: projectConfig.name
			});

			// Step 11: Initial build
			creationProgress = '🔨 Building project...';
			await invoke('build_project', {
				projectName: projectConfig.name
			});

			creationProgress = '✅ Project created successfully!';
			setTimeout(() => {
				isCreating = false;
				creationProgress = '';
				alert(`Project "${projectConfig.name}" created successfully!\n\nNext steps:\n1. cd ${projectConfig.name}\n2. npm run tauri:dev`);
			}, 2000);
		} catch (error) {
			console.error('Project creation failed:', error);
			creationProgress = `❌ Error: ${error}`;
		}
	}

	async function detectAndRepairDrift(): Promise<void> {
		isAutoRepairing = true;
		repairSuggestions = [];

		try {
			const suggestions = await invoke<string[]>('detect_project_drift');
			repairSuggestions = suggestions;

			// Auto-repair known issues
			for (const suggestion of suggestions) {
				if (suggestion.includes('missing')) {
					await invoke('auto_repair_files', { suggestion });
				}
			}

			isAutoRepairing = false;
		} catch (error) {
			console.error('Drift detection failed:', error);
			repairSuggestions = ['Failed to detect drift issues'];
		}
	}

	onMount(() => {
		scanProjectHealth();
	});
</script>

<div class="scaffold-panel">
	<div class="header">
		<h2>🏗️ Project Scaffolding & Health</h2>
		<p class="subtitle">Initialize, validate, and repair your Tauri projects</p>
	</div>

	<div class="tabs">
		<button
			class="tab-button"
			class:active={activeTab === 'health'}
			on:click={() => (activeTab = 'health')}
		>
			🏥 Health Check
		</button>
		<button
			class="tab-button"
			class:active={activeTab === 'create'}
			on:click={() => (activeTab = 'create')}
		>
			➕ New Project
		</button>
		<button
			class="tab-button"
			class:active={activeTab === 'repair'}
			on:click={() => (activeTab = 'repair')}
		>
			🔧 Repair & Fix
		</button>
	</div>

	{#if activeTab === 'health'}
		<div class="tab-content health-content">
			<div class="health-section">
				<div class="section-header">
					<h3>Project Health Report</h3>
					<button on:click={scanProjectHealth} disabled={isScanning} class="scan-btn">
						{isScanning ? '🔄 Scanning...' : '🔍 Scan Now'}
					</button>
				</div>

				{#if healthReport}
					<div class="health-report">
						<div class="health-score" class:excellent={healthReport.score >= 90} class:good={healthReport.score >= 70}
							class:warning={healthReport.score >= 40} class:critical={healthReport.score < 40}>
							<div class="score-circle">{healthReport.score}</div>
							<div class="score-label">{healthReport.status.toUpperCase()}</div>
							<div class="score-date">Checked: {healthReport.lastChecked.toLocaleString()}</div>
						</div>

						{#if healthReport.issues.length > 0}
							<div class="issues-section">
								<h4>⚠️ Issues Found ({healthReport.issues.length})</h4>
								<ul class="issues-list">
									{#each healthReport.issues as issue}
										<li class="issue-item">{issue}</li>
									{/each}
								</ul>
							</div>
						{/if}

						{#if healthReport.recommendations.length > 0}
							<div class="recommendations-section">
								<h4>💡 Recommendations</h4>
								<ul class="recommendations-list">
									{#each healthReport.recommendations as rec}
										<li class="rec-item">✓ {rec}</li>
									{/each}
								</ul>
							</div>
						{/if}
					</div>
				{/if}

				<div class="file-structure">
					<h4>📁 File Structure Check</h4>
					<div class="files-grid">
						{#each fileChecks as file}
							<div class="file-item" class:present={file.status === 'present'} class:missing={file.status === 'missing'}
								class:malformed={file.status === 'malformed'}>
								<div class="file-status-icon">
									{#if file.status === 'present'}
										✅
									{:else if file.status === 'malformed'}
										⚠️
									{:else}
										❌
									{/if}
								</div>
								<div class="file-info">
									<div class="file-path">{file.path}</div>
									<div class="file-category">{file.description}</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			</div>
		</div>
	{/if}

	{#if activeTab === 'create'}
		<div class="tab-content create-content">
			<div class="create-section">
				<h3>Create New Tauri Project</h3>
				<p class="section-description">
					Initialize a production-ready Tauri 2.8.x project with all modern tooling
				</p>

				<form on:submit|preventDefault={createNewProject}>
					<div class="form-group">
						<label for="project-name">Project Name</label>
						<input
							id="project-name"
							type="text"
							bind:value={projectConfig.name}
							placeholder="my-awesome-app"
							disabled={isCreating}
						/>
						<small>Lowercase letters, numbers, hyphens only</small>
					</div>

					<div class="form-row">
						<div class="form-group">
							<label for="template">Template</label>
							<select id="template" bind:value={projectConfig.template} disabled={isCreating}>
								<option value="tauri">Tauri Desktop App</option>
								<option value="svelte">Svelte + Tauri</option>
								<option value="vite-react">Vite + React (alt)</option>
								<option value="full-stack">Full-Stack Starter</option>
							</select>
						</div>

						<div class="form-group">
							<label for="backend">Backend</label>
							<select id="backend" bind:value={projectConfig.backend} disabled={isCreating}>
								<option value="rust">Rust (Recommended)</option>
								<option value="node">Node.js</option>
							</select>
						</div>
					</div>

					<div class="form-row">
						<div class="form-group">
							<label for="database">Database</label>
							<select id="database" bind:value={projectConfig.db} disabled={isCreating}>
								<option value="none">None</option>
								<option value="sqlite">SQLite (Recommended)</option>
								<option value="postgres">PostgreSQL</option>
								<option value="mongodb">MongoDB</option>
							</select>
						</div>

						<div class="form-group">
							<label for="css">Styling</label>
							<select id="css" bind:value={projectConfig.css} disabled={isCreating}>
								<option value="tailwind">Tailwind CSS (Recommended)</option>
								<option value="vanilla-css">Vanilla CSS</option>
							</select>
						</div>
					</div>

					<div class="form-group">
						<label for="package-manager">Package Manager</label>
						<select id="package-manager" bind:value={projectConfig.pm} disabled={isCreating}>
							<option value="npm">npm</option>
							<option value="yarn">Yarn</option>
							<option value="pnpm">pnpm</option>
						</select>
					</div>

					<div class="includes-section">
						<h4>✅ Included in Your Project</h4>
						<ul class="includes-list">
							<li>✓ Tauri 2.8.5 with latest security features</li>
							<li>✓ Rust 2024 Edition with modern practices</li>
							<li>✓ Svelte 5 + TypeScript with strict mode</li>
							<li>✓ Vite for blazing-fast builds</li>
							<li>✓ {projectConfig.css === 'tailwind' ? 'Tailwind CSS' : 'Vanilla CSS'} for styling</li>
							<li>✓ {projectConfig.db !== 'none' ? projectConfig.db : 'No database'}</li>
							<li>✓ OpenCode integration with custom agents</li>
							<li>✓ Beads issue tracking initialized</li>
							<li>✓ OpenSpec workflow templates</li>
							<li>✓ Pre-populated /docs with compliance templates</li>
							<li>✓ GitHub Actions CI/CD starter</li>
							<li>✓ Pre-configured ESLint, Prettier, Cargo fmt</li>
						</ul>
					</div>

					{#if isCreating}
						<div class="creation-progress">
							<div class="progress-text">{creationProgress}</div>
							<div class="progress-bar">
								<div class="progress-fill"></div>
							</div>
						</div>
					{:else}
						<button type="submit" class="create-button">
							🚀 Create Project
						</button>
					{/if}
				</form>
			</div>
		</div>
	{/if}

	{#if activeTab === 'repair'}
		<div class="tab-content repair-content">
			<div class="repair-section">
				<h3>🔧 Auto-Repair Pathological Drift</h3>
				<p class="section-description">
					Detect and automatically fix configuration drift, missing files, and misconfiguration
				</p>

				<div class="repair-actions">
					<button on:click={detectAndRepairDrift} disabled={isAutoRepairing} class="repair-button">
						{isAutoRepairing ? '🔄 Analyzing...' : '🔍 Detect Issues'}
					</button>
				</div>

				{#if repairSuggestions.length > 0}
					<div class="repair-results">
						<h4>Issues Detected & Fixed</h4>
						<div class="suggestions-list">
							{#each repairSuggestions as suggestion}
								<div class="suggestion-item">
									<span class="suggestion-icon">✅</span>
									<span class="suggestion-text">{suggestion}</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				<div class="drift-categories">
					<h4>Common Issues Checked</h4>
					<div class="categories-grid">
						<div class="category-card">
							<div class="category-icon">📦</div>
							<div class="category-title">Dependencies</div>
							<div class="category-desc">Missing or outdated packages</div>
						</div>
						<div class="category-card">
							<div class="category-icon">⚙️</div>
							<div class="category-title">Configuration</div>
							<div class="category-desc">Misaligned config files</div>
						</div>
						<div class="category-card">
							<div class="category-icon">📁</div>
							<div class="category-title">File Structure</div>
							<div class="category-desc">Missing directories/files</div>
						</div>
						<div class="category-card">
							<div class="category-icon">🔐</div>
							<div class="category-title">Security</div>
							<div class="category-desc">Unsafe configurations</div>
						</div>
						<div class="category-card">
							<div class="category-icon">🎨</div>
							<div class="category-title">Code Quality</div>
							<div class="category-desc">Linting & formatting issues</div>
						</div>
						<div class="category-card">
							<div class="category-icon">📚</div>
							<div class="category-title">Documentation</div>
							<div class="category-desc">Missing or outdated docs</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.scaffold-panel {
		background: linear-gradient(135deg, #1e40af 0%, #7c3aed 100%);
		border-radius: 12px;
		padding: 24px;
		margin: 16px 0;
		box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
		color: #333;
	}

	.header {
		text-align: center;
		margin-bottom: 24px;
		color: white;
	}

	.header h2 {
		margin: 0 0 8px 0;
		font-size: 28px;
		font-weight: 700;
	}

	.subtitle {
		margin: 0;
		font-size: 14px;
		opacity: 0.95;
		font-weight: 500;
	}

	.tabs {
		display: flex;
		gap: 12px;
		margin-bottom: 24px;
		flex-wrap: wrap;
	}

	.tab-button {
		padding: 10px 20px;
		border: 2px solid white;
		border-radius: 8px;
		background: transparent;
		color: white;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s ease;
		font-size: 14px;
	}

	.tab-button:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.2);
		transform: translateY(-2px);
	}

	.tab-button.active {
		background: white;
		color: #1e40af;
	}

	.tab-content {
		background: white;
		border-radius: 8px;
		padding: 24px;
		animation: fadeIn 0.3s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Health Tab */
	.health-section {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
	}

	.section-header h3 {
		margin: 0;
		color: #1e40af;
		font-size: 20px;
	}

	.scan-btn {
		padding: 8px 16px;
		background: #1e40af;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 600;
		transition: all 0.3s ease;
	}

	.scan-btn:hover:not(:disabled) {
		background: #1e3a8a;
		transform: translateY(-2px);
	}

	.scan-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.health-report {
		background: #f8fafc;
		border-radius: 8px;
		padding: 20px;
		display: grid;
		grid-template-columns: 200px 1fr;
		gap: 24px;
	}

	.health-score {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 20px;
		border-radius: 8px;
		background: white;
		border: 3px solid #e2e8f0;
	}

	.health-score.excellent {
		border-color: #22c55e;
		background: #f0fdf4;
	}

	.health-score.good {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.health-score.warning {
		border-color: #f59e0b;
		background: #fffbeb;
	}

	.health-score.critical {
		border-color: #ef4444;
		background: #fef2f2;
	}

	.score-circle {
		font-size: 48px;
		font-weight: 700;
		margin-bottom: 8px;
	}

	.health-score.excellent .score-circle {
		color: #22c55e;
	}

	.health-score.good .score-circle {
		color: #3b82f6;
	}

	.health-score.warning .score-circle {
		color: #f59e0b;
	}

	.health-score.critical .score-circle {
		color: #ef4444;
	}

	.score-label {
		font-size: 14px;
		font-weight: 600;
		margin-bottom: 8px;
	}

	.score-date {
		font-size: 12px;
		color: #666;
	}

	.issues-section,
	.recommendations-section {
		grid-column: 2;
	}

	.issues-section h4,
	.recommendations-section h4 {
		margin: 0 0 12px 0;
		color: #333;
		font-size: 15px;
	}

	.issues-list,
	.recommendations-list {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.issue-item,
	.rec-item {
		padding: 8px 12px;
		background: white;
		border-radius: 6px;
		font-size: 13px;
		line-height: 1.5;
	}

	.issue-item {
		border-left: 4px solid #ef4444;
		color: #7f1d1d;
	}

	.rec-item {
		border-left: 4px solid #22c55e;
		color: #166534;
	}

	.file-structure {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.file-structure h4 {
		margin: 0;
		color: #333;
		font-size: 15px;
	}

	.files-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 12px;
	}

	.file-item {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px;
		background: #f8fafc;
		border-radius: 6px;
		border-left: 4px solid #e2e8f0;
		transition: all 0.3s ease;
	}

	.file-item.present {
		background: #f0fdf4;
		border-left-color: #22c55e;
	}

	.file-item.missing {
		background: #fef2f2;
		border-left-color: #ef4444;
	}

	.file-item.malformed {
		background: #fffbeb;
		border-left-color: #f59e0b;
	}

	.file-status-icon {
		font-size: 20px;
		min-width: 24px;
	}

	.file-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.file-path {
		font-size: 13px;
		font-weight: 600;
		color: #333;
		font-family: 'Courier New', monospace;
	}

	.file-category {
		font-size: 12px;
		color: #666;
	}

	/* Create Tab */
	.create-content {
		max-width: 800px;
		margin: 0 auto;
	}

	.create-section {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.create-section h3 {
		margin: 0 0 8px 0;
		color: #1e40af;
		font-size: 22px;
	}

	.section-description {
		margin: 0;
		color: #666;
		font-size: 14px;
		line-height: 1.6;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group label {
		font-weight: 600;
		color: #333;
		font-size: 14px;
	}

	.form-group input,
	.form-group select {
		padding: 10px 12px;
		border: 2px solid #e2e8f0;
		border-radius: 6px;
		font-size: 14px;
		transition: all 0.3s ease;
		font-family: inherit;
	}

	.form-group input:focus,
	.form-group select:focus {
		outline: none;
		border-color: #1e40af;
		box-shadow: 0 0 0 3px rgba(30, 64, 175, 0.1);
	}

	.form-group input:disabled,
	.form-group select:disabled {
		background: #f8fafc;
		color: #999;
		cursor: not-allowed;
	}

	.form-group small {
		font-size: 12px;
		color: #999;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.includes-section {
		background: #f8fafc;
		padding: 16px;
		border-radius: 8px;
		border-left: 4px solid #1e40af;
	}

	.includes-section h4 {
		margin: 0 0 12px 0;
		color: #1e40af;
		font-size: 14px;
	}

	.includes-list {
		list-style: none;
		padding: 0;
		margin: 0;
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 8px;
	}

	.includes-list li {
		font-size: 13px;
		color: #333;
		line-height: 1.6;
	}

	.creation-progress {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.progress-text {
		font-size: 14px;
		color: #1e40af;
		font-weight: 600;
		text-align: center;
	}

	.progress-bar {
		height: 6px;
		background: #e2e8f0;
		border-radius: 3px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #1e40af 0%, #7c3aed 100%);
		animation: progress 2s ease-in-out infinite;
	}

	@keyframes progress {
		0%, 100% {
			width: 0;
		}
		50% {
			width: 100%;
		}
	}

	.create-button {
		padding: 12px 24px;
		background: linear-gradient(135deg, #1e40af 0%, #7c3aed 100%);
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 15px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.create-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(30, 64, 175, 0.3);
	}

	.create-button:active {
		transform: translateY(0);
	}

	/* Repair Tab */
	.repair-section {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.repair-section h3 {
		margin: 0 0 8px 0;
		color: #1e40af;
		font-size: 22px;
	}

	.repair-actions {
		display: flex;
		gap: 12px;
	}

	.repair-button {
		padding: 12px 24px;
		background: #1e40af;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	.repair-button:hover:not(:disabled) {
		background: #1e3a8a;
		transform: translateY(-2px);
	}

	.repair-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.repair-results {
		background: #f0fdf4;
		border: 2px solid #22c55e;
		border-radius: 8px;
		padding: 16px;
	}

	.repair-results h4 {
		margin: 0 0 12px 0;
		color: #166534;
		font-size: 15px;
	}

	.suggestions-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.suggestion-item {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 10px;
		background: white;
		border-radius: 6px;
		border-left: 4px solid #22c55e;
		font-size: 13px;
	}

	.suggestion-icon {
		font-size: 16px;
		min-width: 20px;
	}

	.suggestion-text {
		color: #333;
		line-height: 1.5;
	}

	.drift-categories {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.drift-categories h4 {
		margin: 0;
		color: #333;
		font-size: 15px;
	}

	.categories-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
		gap: 12px;
	}

	.category-card {
		background: #f8fafc;
		border: 2px solid #e2e8f0;
		border-radius: 8px;
		padding: 16px;
		text-align: center;
		transition: all 0.3s ease;
		cursor: pointer;
	}

	.category-card:hover {
		border-color: #1e40af;
		background: #eff6ff;
		transform: translateY(-2px);
	}

	.category-icon {
		font-size: 32px;
		margin-bottom: 8px;
	}

	.category-title {
		font-weight: 600;
		color: #333;
		font-size: 14px;
		margin-bottom: 4px;
	}

	.category-desc {
		font-size: 12px;
		color: #666;
	}

	@media (max-width: 768px) {
		.scaffold-panel {
			padding: 16px;
		}

		.header h2 {
			font-size: 20px;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.includes-list {
			grid-template-columns: 1fr;
		}

		.health-report {
			grid-template-columns: 1fr;
		}

		.issues-section,
		.recommendations-section {
			grid-column: 1;
		}

		.categories-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
