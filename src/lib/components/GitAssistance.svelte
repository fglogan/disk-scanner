<script lang="ts">
	import { onMount } from 'svelte';

	let activeTab: string = 'learn';
	let expandedSection: string | null = null;
	let gitStatus: any = {
		branch: 'main',
		commits_ahead: 0,
		commits_behind: 0,
		uncommitted: 0,
		untracked: 0,
		has_pr: false,
		ci_status: 'pending'
	};

	let workflows = [
		{
			id: 'new-feature',
			title: '✨ Start a New Feature',
			description: 'Create a new feature branch and get started',
			steps: [
				{
					title: 'Create Feature Branch',
					command: 'git checkout -b feature/my-feature',
					explanation: 'Creates and switches to a new branch for your feature'
				},
				{
					title: 'Make Your Changes',
					command: 'Edit files and save',
					explanation: 'Work on your feature naturally'
				},
				{
					title: 'Stage Changes',
					command: 'git add .',
					explanation: 'Prepare your changes for commit'
				},
				{
					title: 'Commit Changes',
					command: 'git commit -m "feat: add my feature"',
					explanation: 'Save your work with a descriptive message'
				},
				{
					title: 'Push to Remote',
					command: 'git push origin feature/my-feature',
					explanation: 'Upload your branch to GitHub'
				}
			]
		},
		{
			id: 'pr-workflow',
			title: '🔄 Create a Pull Request',
			description: 'Submit your changes for review',
			steps: [
				{
					title: 'Push Your Branch',
					command: 'git push origin feature/my-feature',
					explanation: 'Ensure your branch is on GitHub'
				},
				{
					title: 'Open GitHub',
					command: 'Visit github.com/yourrepo',
					explanation: 'Navigate to your repository'
				},
				{
					title: 'Create PR',
					command: 'Click "Create Pull Request" button',
					explanation: 'GitHub will suggest creating a PR from your branch'
				},
				{
					title: 'Add Description',
					command: 'Describe your changes',
					explanation: 'Explain what you changed and why'
				},
				{
					title: 'Request Review',
					command: 'Add reviewers and submit',
					explanation: 'Ask team members to review your code'
				}
			]
		},
		{
			id: 'sync-main',
			title: '⬇️ Sync with Main',
			description: 'Update your work with latest changes',
			steps: [
				{
					title: 'Switch to Main',
					command: 'git checkout main',
					explanation: 'Go to the main branch'
				},
				{
					title: 'Fetch Latest',
					command: 'git fetch origin',
					explanation: 'Download latest changes from remote'
				},
				{
					title: 'Pull Changes',
					command: 'git pull origin main',
					explanation: 'Update main branch with latest commits'
				},
				{
					title: 'Switch Back',
					command: 'git checkout feature/my-feature',
					explanation: 'Return to your feature branch'
				},
				{
					title: 'Merge Main In',
					command: 'git merge main',
					explanation: 'Bring main\'s changes into your branch'
				}
			]
		},
		{
			id: 'resolve-conflict',
			title: '🔧 Fix Merge Conflict',
			description: 'Handle conflicts when merging',
			steps: [
				{
					title: 'Identify Conflict',
					command: 'git status',
					explanation: 'Files with conflicts will be marked'
				},
				{
					title: 'Open Files',
					command: 'Open conflicted files in editor',
					explanation: 'Look for <<<<<<, ======, and >>>>>> markers'
				},
				{
					title: 'Resolve Manually',
					command: 'Choose which changes to keep',
					explanation: 'Edit the file to remove conflict markers and keep desired code'
				},
				{
					title: 'Mark Resolved',
					command: 'git add <resolved-files>',
					explanation: 'Tell Git the conflicts are fixed'
				},
				{
					title: 'Complete Merge',
					command: 'git commit -m "Merge main: resolve conflicts"',
					explanation: 'Finalize the merge with a commit'
				}
			]
		},
		{
			id: 'catch-up',
			title: '🚀 Catch Up Commits',
			description: 'Update your branch with missed commits',
			steps: [
				{
					title: 'Check Status',
					command: 'git status',
					explanation: 'See how many commits you\'re behind'
				},
				{
					title: 'Rebase (Recommended)',
					command: 'git rebase origin/main',
					explanation: 'Replay your commits on top of latest main (cleaner history)'
				},
				{
					title: 'Or Merge',
					command: 'git merge origin/main',
					explanation: 'Alternative: merge latest main into your branch'
				},
				{
					title: 'Resolve Any Conflicts',
					command: 'Fix conflicted files (see conflict workflow)',
					explanation: 'If there are conflicts, follow the conflict resolution process'
				},
				{
					title: 'Force Push if Rebased',
					command: 'git push --force-with-lease origin feature/my-feature',
					explanation: 'Only needed if you rebased (safer than --force)'
				}
			]
		},
		{
			id: 'ci-failure',
			title: '❌ Fix CI/CD Failure',
			description: 'Handle failed automated tests',
			steps: [
				{
					title: 'Check CI Status',
					command: 'View GitHub Actions tab',
					explanation: 'See which tests failed and why'
				},
				{
					title: 'Review Logs',
					command: 'Click on failed job for details',
					explanation: 'Understand what went wrong'
				},
				{
					title: 'Make Fix Locally',
					command: 'Edit code and test locally',
					explanation: 'Reproduce and fix the issue'
				},
				{
					title: 'Commit & Push',
					command: 'git add . && git commit -m "fix: resolve CI failure" && git push',
					explanation: 'Push your fix to trigger CI again'
				},
				{
					title: 'Monitor Re-run',
					command: 'Watch CI pipeline',
					explanation: 'Verify tests pass on retry'
				}
			]
		},
		{
			id: 'cleanup-branch',
			title: '🧹 Clean Up Local Branches',
			description: 'Remove old and merged branches',
			steps: [
				{
					title: 'List Branches',
					command: 'git branch -a',
					explanation: 'See all local and remote branches'
				},
				{
					title: 'Prune Remote',
					command: 'git fetch --prune',
					explanation: 'Remove remote branches that were deleted'
				},
				{
					title: 'Delete Local Branch',
					command: 'git branch -d feature/old-feature',
					explanation: 'Delete a branch locally (safe delete, requires merge)'
				},
				{
					title: 'Force Delete if Needed',
					command: 'git branch -D feature/abandoned',
					explanation: 'Force delete without merge check (use carefully)'
				},
				{
					title: 'Clean Up Remotes',
					command: 'git remote prune origin',
					explanation: 'Clean up stale remote references'
				}
			]
		}
	];

	let tips = [
		{
			title: '💡 Commit Early, Commit Often',
			tip: 'Make small, logical commits with clear messages. This makes history easier to read and changes easier to revert if needed.'
		},
		{
			title: '📝 Write Good Commit Messages',
			tip: 'Start with a verb (feat:, fix:, docs:, etc). Keep it under 50 characters for the title, add detail in the body if needed.'
		},
		{
			title: '🌿 Branch Naming Matters',
			tip: 'Use descriptive names: feature/user-auth, bugfix/login-crash, docs/api-readme. Avoid generic names like "fix" or "changes".'
		},
		{
			title: '🔄 Sync Regularly',
			tip: 'Keep your branch up-to-date with main. Merge or rebase regularly to avoid large, difficult merge conflicts.'
		},
		{
			title: '✅ Test Before Pushing',
			tip: 'Run tests locally before pushing. This catches issues early and keeps the CI pipeline green.'
		},
		{
			title: '🔍 Review Your Own PR First',
			tip: 'Before requesting review, look at your own PR changes. Catch obvious issues and typos before others review.'
		},
		{
			title: '⚡ Use Shortcuts',
			tip: 'git add . (all files), git commit --amend (fix last commit), git stash (save work temporarily)'
		},
		{
			title: '🛑 Never Force Push to Main',
			tip: 'Always use --force-with-lease, and only on your own branches. Never rewrite main branch history.'
		}
	];

	let glossary = [
		{
			term: 'Branch',
			definition: 'A parallel version of your code. Default branch is usually "main" or "master".'
		},
		{
			term: 'Commit',
			definition: 'A snapshot of your code at a point in time. Every commit has a unique ID and message.'
		},
		{
			term: 'Push',
			definition: 'Upload your local commits to the remote repository (usually GitHub).'
		},
		{
			term: 'Pull',
			definition: 'Download and merge remote changes into your local branch.'
		},
		{
			term: 'Merge',
			definition: 'Combine changes from two branches into one. Can create merge commits.'
		},
		{
			term: 'Rebase',
			definition: 'Replay your commits on top of another branch. Creates cleaner history than merge.'
		},
		{
			term: 'Pull Request (PR)',
			definition: 'Request to merge your branch into main. Allows code review before merging.'
		},
		{
			term: 'Conflict',
			definition: 'Occurs when two branches modify the same lines differently. Must be resolved manually.'
		},
		{
			term: 'Fork',
			definition: 'Create your own copy of a repository. Common in open source for contributing.'
		},
		{
			term: 'Clone',
			definition: 'Download a repository to your computer for the first time.'
		},
		{
			term: 'Fetch',
			definition: 'Download remote changes without modifying local files. Safe way to see what changed.'
		},
		{
			term: 'Stash',
			definition: 'Temporarily save work without committing. Useful when you need to switch branches quickly.'
		}
	];

	function toggleSection(id: string): void {
		expandedSection = expandedSection === id ? null : id;
	}

	function copyCommand(command: string): void {
		navigator.clipboard.writeText(command);
		// Could add a toast notification here
	}

	function fetchGitStatus(): void {
		// This would call a Tauri command to get real git status
		// For now, showing placeholder
		console.log('Fetching git status...');
	}

	onMount(() => {
		fetchGitStatus();
	});
</script>

<div class="git-assistance-panel">
	<div class="header">
		<h2>🚀 Git Assistance Hub</h2>
		<p class="subtitle">Learn Git, manage branches, and understand workflows</p>
	</div>

	<div class="tabs">
		<button
			class="tab-button"
			class:active={activeTab === 'learn'}
			on:click={() => (activeTab = 'learn')}
		>
			📚 Learn
		</button>
		<button
			class="tab-button"
			class:active={activeTab === 'workflows'}
			on:click={() => (activeTab = 'workflows')}
		>
			🔄 Workflows
		</button>
		<button
			class="tab-button"
			class:active={activeTab === 'glossary'}
			on:click={() => (activeTab = 'glossary')}
		>
			📖 Glossary
		</button>
		<button
			class="tab-button"
			class:active={activeTab === 'tips'}
			on:click={() => (activeTab = 'tips')}
		>
			💡 Tips
		</button>
	</div>

	{#if activeTab === 'learn'}
		<div class="tab-content learn-content">
			<div class="learn-section">
				<h3>🎯 Git Workflow Visualization</h3>
				<div class="workflow-diagram">
					<div class="flow-box">
						<div class="box-icon">1️⃣</div>
						<div class="box-label">Create Branch</div>
						<code>git checkout -b feature</code>
					</div>
					<div class="arrow">→</div>
					<div class="flow-box">
						<div class="box-icon">2️⃣</div>
						<div class="box-label">Make Changes</div>
						<code>Edit files</code>
					</div>
					<div class="arrow">→</div>
					<div class="flow-box">
						<div class="box-icon">3️⃣</div>
						<div class="box-label">Commit</div>
						<code>git commit</code>
					</div>
					<div class="arrow">→</div>
					<div class="flow-box">
						<div class="box-icon">4️⃣</div>
						<div class="box-label">Push</div>
						<code>git push</code>
					</div>
					<div class="arrow">→</div>
					<div class="flow-box">
						<div class="box-icon">5️⃣</div>
						<div class="box-label">Create PR</div>
						<code>Review & Merge</code>
					</div>
				</div>
			</div>

			<div class="learn-section">
				<h3>📊 Your Repository Status</h3>
				<div class="status-grid">
					<div class="status-card">
						<div class="status-value current-branch">
							{gitStatus.branch}
						</div>
						<div class="status-label">Current Branch</div>
					</div>
					<div class="status-card">
						<div class="status-value ahead">{gitStatus.commits_ahead}</div>
						<div class="status-label">Commits Ahead</div>
					</div>
					<div class="status-card">
						<div class="status-value behind">{gitStatus.commits_behind}</div>
						<div class="status-label">Commits Behind</div>
					</div>
					<div class="status-card">
						<div class="status-value uncommitted">{gitStatus.uncommitted}</div>
						<div class="status-label">Uncommitted Changes</div>
					</div>
					<div class="status-card">
						<div class="status-value untracked">{gitStatus.untracked}</div>
						<div class="status-label">Untracked Files</div>
					</div>
					<div class="status-card">
						<div class="status-value ci-status">{gitStatus.ci_status}</div>
						<div class="status-label">CI/CD Status</div>
					</div>
				</div>
			</div>
		</div>
	{/if}

	{#if activeTab === 'workflows'}
		<div class="tab-content workflows-content">
			<div class="workflows-list">
				{#each workflows as workflow (workflow.id)}
					<div class="workflow-item">
						<button
							class="workflow-header"
							on:click={() => toggleSection(workflow.id)}
							class:expanded={expandedSection === workflow.id}
						>
							<span class="workflow-title">{workflow.title}</span>
							<span class="workflow-description">{workflow.description}</span>
							<span class="expand-icon">{expandedSection === workflow.id ? '▼' : '▶'}</span>
						</button>

						{#if expandedSection === workflow.id}
							<div class="workflow-steps">
								{#each workflow.steps as step, idx}
									<div class="step">
										<div class="step-number">{idx + 1}</div>
										<div class="step-content">
											<h4>{step.title}</h4>
											<div class="step-command">
												<code>{step.command}</code>
												<button
													class="copy-btn"
													on:click={() => copyCommand(step.command)}
													title="Copy to clipboard"
												>
													📋
												</button>
											</div>
											<p class="step-explanation">{step.explanation}</p>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	{/if}

	{#if activeTab === 'glossary'}
		<div class="tab-content glossary-content">
			<div class="glossary-grid">
				{#each glossary as item (item.term)}
					<div class="glossary-item">
						<h4>{item.term}</h4>
						<p>{item.definition}</p>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	{#if activeTab === 'tips'}
		<div class="tab-content tips-content">
			<div class="tips-grid">
				{#each tips as tip (tip.title)}
					<div class="tip-card">
						<h4>{tip.title}</h4>
						<p>{tip.tip}</p>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style>
	.git-assistance-panel {
		background: white;
		border-radius: 12px;
		padding: 20px;
		margin: 10px 0;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		color: #333;
		border: 1px solid #e5e7eb;
	}

	.header {
		text-align: center;
		margin-bottom: 20px;
		color: #667eea;
		background: linear-gradient(135deg, #f5f7ff 0%, #faf9ff 100%);
		padding: 20px;
		border-radius: 8px;
		margin: -20px -20px 20px -20px;
	}

	.header h2 {
		margin: 0 0 5px 0;
		font-size: 24px;
		font-weight: 700;
	}

	.subtitle {
		margin: 0;
		font-size: 14px;
		opacity: 0.9;
		font-weight: 500;
	}

	.tabs {
		display: flex;
		gap: 8px;
		margin-bottom: 20px;
		flex-wrap: wrap;
	}

	.tab-button {
		padding: 8px 16px;
		border: none;
		border-radius: 6px;
		background: white;
		color: #667eea;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s ease;
		font-size: 13px;
	}

	.tab-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.tab-button.active {
		background: #333;
		color: white;
	}

	.tab-content {
		background: #f8f9fb;
		border-radius: 8px;
		padding: 20px;
		animation: fadeIn 0.3s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(5px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Learn Tab */
	.learn-section {
		margin-bottom: 30px;
	}

	.learn-section h3 {
		color: #667eea;
		margin-bottom: 15px;
		font-size: 18px;
	}

	.workflow-diagram {
		display: flex;
		align-items: center;
		gap: 8px;
		overflow-x: auto;
		padding: 15px;
		background: #f5f7ff;
		border-radius: 8px;
		margin-bottom: 15px;
	}

	.flow-box {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 6px;
		padding: 12px;
		background: white;
		border-radius: 6px;
		border-left: 4px solid #667eea;
		min-width: 130px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.box-icon {
		font-size: 20px;
	}

	.box-label {
		font-size: 12px;
		font-weight: 600;
		color: #333;
	}

	.flow-box code {
		background: #f0f0f0;
		padding: 4px 8px;
		border-radius: 3px;
		font-size: 11px;
		color: #667eea;
	}

	.arrow {
		font-size: 20px;
		color: #667eea;
		font-weight: bold;
	}

	.status-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
		gap: 12px;
	}

	.status-card {
		background: #f5f7ff;
		border-radius: 8px;
		padding: 15px;
		text-align: center;
		border-top: 3px solid #667eea;
	}

	.status-value {
		font-size: 24px;
		font-weight: 700;
		margin-bottom: 5px;
		font-family: 'Courier New', monospace;
	}

	.status-value.current-branch {
		color: #667eea;
	}

	.status-value.ahead {
		color: #48bb78;
	}

	.status-value.behind {
		color: #ed8936;
	}

	.status-value.uncommitted {
		color: #f56565;
	}

	.status-value.untracked {
		color: #ecc94b;
	}

	.status-value.ci-status {
		color: #4299e1;
	}

	.status-label {
		font-size: 12px;
		color: #666;
		font-weight: 500;
	}

	/* Workflows Tab */
	.workflows-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.workflow-item {
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		overflow: hidden;
	}

	.workflow-header {
		width: 100%;
		padding: 16px;
		background: #f7fafc;
		border: none;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 12px;
		transition: background 0.3s ease;
		font-size: 15px;
	}

	.workflow-header:hover {
		background: #edf2f7;
	}

	.workflow-header.expanded {
		background: #e6fffa;
		border-bottom: 2px solid #667eea;
	}

	.workflow-title {
		font-weight: 700;
		color: #333;
		flex: 1;
		text-align: left;
	}

	.workflow-description {
		font-size: 13px;
		color: #666;
		flex: 1;
	}

	.expand-icon {
		font-size: 12px;
		color: #667eea;
	}

	.workflow-steps {
		padding: 20px;
		background: #f8f9fb;
		border-top: 1px solid #e2e8f0;
	}

	.step {
		display: flex;
		gap: 16px;
		margin-bottom: 20px;
		padding-bottom: 20px;
		border-bottom: 1px solid #e2e8f0;
	}

	.step:last-child {
		border-bottom: none;
		margin-bottom: 0;
		padding-bottom: 0;
	}

	.step-number {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
		width: 32px;
		height: 32px;
		background: #667eea;
		color: white;
		border-radius: 50%;
		font-weight: 700;
		font-size: 14px;
	}

	.step-content h4 {
		margin: 0 0 8px 0;
		color: #333;
		font-size: 14px;
		font-weight: 600;
	}

	.step-command {
		display: flex;
		align-items: center;
		gap: 8px;
		background: #f5f7ff;
		padding: 10px 12px;
		border-radius: 6px;
		margin-bottom: 8px;
	}

	.step-command code {
		font-family: 'Courier New', monospace;
		font-size: 12px;
		color: #667eea;
		flex: 1;
	}

	.copy-btn {
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		padding: 4px 8px;
		cursor: pointer;
		font-size: 12px;
		transition: all 0.2s ease;
	}

	.copy-btn:hover {
		background: #f7fafc;
		border-color: #cbd5e0;
	}

	.step-explanation {
		margin: 0;
		font-size: 13px;
		color: #666;
		line-height: 1.4;
	}

	/* Glossary Tab */
	.glossary-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 16px;
	}

	.glossary-item {
		background: #f7fafc;
		padding: 16px;
		border-radius: 8px;
		border-left: 4px solid #667eea;
	}

	.glossary-item h4 {
		margin: 0 0 8px 0;
		color: #333;
		font-size: 15px;
		font-weight: 600;
	}

	.glossary-item p {
		margin: 0;
		font-size: 13px;
		color: #666;
		line-height: 1.5;
	}

	/* Tips Tab */
	.tips-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 16px;
	}

	.tip-card {
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		color: white;
		padding: 16px;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.tip-card h4 {
		margin: 0 0 10px 0;
		font-size: 14px;
		font-weight: 600;
	}

	.tip-card p {
		margin: 0;
		font-size: 13px;
		line-height: 1.5;
		opacity: 0.95;
	}

	@media (max-width: 768px) {
		.workflow-diagram {
			flex-direction: column;
			align-items: stretch;
		}

		.arrow {
			transform: rotate(90deg);
		}

		.flow-box {
			width: 100%;
		}

		.status-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.glossary-grid,
		.tips-grid {
			grid-template-columns: 1fr;
		}

		.workflow-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.workflow-description {
			font-size: 12px;
		}
	}
</style>
