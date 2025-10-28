#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const BEADS_FILE = path.join(__dirname, '../docs/BEADS_ISSUE_TRACKER.md');

function parsePriority(title) {
  if (title.includes('🚨')) return 'critical';
  if (title.includes('⚠️')) return 'high';
  if (title.includes('📝')) return 'medium';
  if (title.includes('✅')) return 'low';
  return 'medium';
}

function parseStatus(statusLine) {
  if (!statusLine) return 'pending';
  if (statusLine.includes('✅ COMPLETED')) return 'completed';
  if (statusLine.includes('IN PROGRESS')) return 'in_progress';
  if (statusLine.includes('PENDING')) return 'pending';
  return 'pending';
}

function parseBeadsIssues() {
  const content = fs.readFileSync(BEADS_FILE, 'utf-8');
  const lines = content.split('\n');
  
  const issues = [];
  let currentIssue = null;
  let inCodeBlock = false;
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    if (line.startsWith('```')) {
      inCodeBlock = !inCodeBlock;
    }
    
    const headerMatch = line.match(/^### (BEAD-[A-Z0-9-]+): (.+?)( [🚨⚠️📝✅])?$/);
    if (headerMatch) {
      if (currentIssue) {
        issues.push(currentIssue);
      }
      
      const [, id, title] = headerMatch;
      currentIssue = {
        id,
        title: title.trim(),
        priority: parsePriority(line),
        status: 'pending',
        effort: '',
        impact: '',
        assignee: '',
        dependencies: [],
        description: '',
        files: [],
        githubIssue: null,
        lineNumber: i + 1
      };
      continue;
    }
    
    if (!currentIssue || inCodeBlock) continue;
    
    if (line.startsWith('**Status:**')) {
      currentIssue.status = parseStatus(line);
    } else if (line.startsWith('**Priority:**')) {
      const priority = line.split(':')[1]?.trim().toLowerCase().replace(/\*\*/g, '').trim();
      if (priority) currentIssue.priority = priority;
    } else if (line.startsWith('**Effort:**')) {
      currentIssue.effort = line.split(':')[1]?.trim().replace(/\*\*/g, '').trim() || '';
    } else if (line.startsWith('**Impact:**')) {
      currentIssue.impact = line.split(':')[1]?.trim().replace(/\*\*/g, '').trim() || '';
    } else if (line.startsWith('**Assignee:**')) {
      currentIssue.assignee = line.split(':')[1]?.trim().replace(/\*\*/g, '').trim() || '';
    } else if (line.startsWith('**Dependencies:**')) {
      const deps = line.split(':')[1]?.trim();
      if (deps && deps !== 'None') {
        currentIssue.dependencies = deps.split(',').map(d => d.trim());
      }
    } else if (line.startsWith('**Files:**')) {
      const files = line.substring(line.indexOf(':') + 1).trim();
      if (files) {
        currentIssue.files.push(files);
      }
    } else if (line.startsWith('**GitHub:**') || line.startsWith('**GitHub Issue:**')) {
      const match = line.match(/#(\d+)/);
      if (match) {
        currentIssue.githubIssue = parseInt(match[1]);
      }
    } else if (line.startsWith('- ') && !line.startsWith('- `')) {
      currentIssue.description += line.substring(2) + '\n';
    } else if (line.trim() && !line.startsWith('**') && !line.startsWith('---')) {
      if (!currentIssue.description || currentIssue.description.endsWith('\n\n')) {
        currentIssue.description += line + '\n';
      }
    }
    
    if (line.startsWith('---') && currentIssue.description) {
      currentIssue.description = currentIssue.description.trim();
    }
  }
  
  if (currentIssue) {
    issues.push(currentIssue);
  }
  
  return issues;
}

function generateSummary(issues) {
  const stats = {
    total: issues.length,
    byStatus: {},
    byPriority: {},
  };
  
  issues.forEach(issue => {
    stats.byStatus[issue.status] = (stats.byStatus[issue.status] || 0) + 1;
    stats.byPriority[issue.priority] = (stats.byPriority[issue.priority] || 0) + 1;
  });
  
  return stats;
}

function exportToJson(issues) {
  const output = {
    generated: new Date().toISOString(),
    summary: generateSummary(issues),
    issues
  };
  
  const outputPath = path.join(__dirname, '../docs/beads-export.json');
  fs.writeFileSync(outputPath, JSON.stringify(output, null, 2));
  console.log(`✅ Exported ${issues.length} issues to ${outputPath}`);
  
  return output;
}

console.log('🔍 Parsing BEADS Issue Tracker...\n');

const issues = parseBeadsIssues();
const output = exportToJson(issues);

console.log('\n📊 Summary:');
console.log(`   Total Issues: ${output.summary.total}`);
console.log('\n   By Status:');
Object.entries(output.summary.byStatus).forEach(([status, count]) => {
  console.log(`     ${status}: ${count}`);
});
console.log('\n   By Priority:');
Object.entries(output.summary.byPriority).forEach(([priority, count]) => {
  console.log(`     ${priority}: ${count}`);
});

console.log('\n✨ First 5 issues:');
issues.slice(0, 5).forEach(issue => {
  console.log(`   ${issue.id}: ${issue.title} [${issue.status}]`);
});

export { parseBeadsIssues, generateSummary, exportToJson };
