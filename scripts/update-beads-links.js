#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const BEADS_FILE = path.join(__dirname, '../docs/BEADS_ISSUE_TRACKER.md');
const EXPORT_FILE = path.join(__dirname, '../docs/beads-export.json');

console.log('🔗 Updating BEADS_ISSUE_TRACKER.md with GitHub issue links...\n');

const exportData = JSON.parse(fs.readFileSync(EXPORT_FILE, 'utf-8'));
const beadsContent = fs.readFileSync(BEADS_FILE, 'utf-8');
const lines = beadsContent.split('\n');

const issuesWithGithub = exportData.issues.filter(i => i.githubIssue !== null);
console.log(`📋 Found ${issuesWithGithub.length} issues with GitHub links\n`);

let updatedLines = [...lines];
let updatedCount = 0;

issuesWithGithub.forEach(issue => {
  const headerId = issue.id;
  const githubNum = issue.githubIssue;
  
  for (let i = 0; i < updatedLines.length; i++) {
    const line = updatedLines[i];
    
    if (line.includes(`### ${headerId}:`)) {
      let nextLineIdx = i + 1;
      
      while (nextLineIdx < updatedLines.length && 
             !updatedLines[nextLineIdx].startsWith('**Status:**')) {
        nextLineIdx++;
      }
      
      if (nextLineIdx < updatedLines.length) {
        const statusLine = updatedLines[nextLineIdx];
        
        if (!statusLine.includes('**GitHub:**')) {
          let insertIdx = nextLineIdx + 1;
          
          while (insertIdx < updatedLines.length && 
                 updatedLines[insertIdx].startsWith('**')) {
            if (updatedLines[insertIdx].startsWith('**GitHub:**')) {
              updatedLines[insertIdx] = `**GitHub:** #${githubNum}`;
              updatedCount++;
              console.log(`   ✅ Updated ${headerId} → #${githubNum}`);
              break;
            }
            insertIdx++;
          }
          
          if (insertIdx >= updatedLines.length || 
              !updatedLines[insertIdx].startsWith('**GitHub:**')) {
            updatedLines.splice(nextLineIdx + 1, 0, `**GitHub:** #${githubNum}`);
            updatedCount++;
            console.log(`   ➕ Added ${headerId} → #${githubNum}`);
          }
        }
      }
      
      break;
    }
  }
});

fs.writeFileSync(BEADS_FILE, updatedLines.join('\n'));

console.log(`\n✨ Updated ${updatedCount} BEADS entries with GitHub links`);
console.log(`📄 File: ${BEADS_FILE}`);
