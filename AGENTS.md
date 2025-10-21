# AGENTS.md: Disk Bloat Scanner Project Agents

## Overview
This document defines the agents and protocols for the Disk Bloat Scanner project, including restart procedures for unknown states after crashes and unknown git commit states.

## Agents
- **Scanner Agent**: Handles disk scanning and bloat detection.
- **Cleanup Agent**: Manages file removal and organization.
- **Monitor Agent**: Provides real-time system monitoring.
- **Recovery Agent**: Handles crash recovery and state restoration.

## Restart Procedures

### After Application Crash
1. **Automatic Restart**:
   - The application attempts to restart automatically if crash detected.
   - Logs are written to `logs/crash.log` with timestamp and error details.

2. **Manual Restart**:
   - Close any running instances.
   - Run `cargo tauri dev` or `cargo tauri build` to restart.
   - Check system resources (RAM, CPU) for overload.

3. **State Recovery**:
   - On restart, the app checks for incomplete operations.
   - Resumes from last known good state if possible.
   - Alerts user if data loss detected.

### Unknown Git Commit State
1. **Check Current State**:
   - Run `git status` to see uncommitted changes, staged files, etc.
   - Run `git log --oneline -10` to view recent commits.

2. **Resolve Detached HEAD**:
   - If in detached HEAD: `git checkout <branch-name>` (e.g., main).
   - If unknown commit: `git reflog` to find last known state, then `git reset --hard <commit-hash>`.

3. **Handle Uncommitted Changes**:
   - Stash changes: `git stash`.
   - Or commit: `git add . && git commit -m "Emergency commit"`.
   - If corrupted, clone fresh repo and copy over changes.

4. **Reset to Known State**:
   - `git reset --hard HEAD` to discard uncommitted changes.
   - `git clean -fd` to remove untracked files.
   - Pull latest: `git pull origin main` (assuming main branch).

5. **Prevent Future Issues**:
   - Commit frequently.
   - Use branches for experimental changes.
   - Backup important files externally.

### Emergency Protocol (TES-2025-v6.9)
- If both crash and git issues occur, follow TES protocol:
  - Isolate project directory.
  - Restore from backup if available.
  - Refer to issue tracker.
  - Do NOT destructively delete any files until HIL confirmation and concurrence

## Best Practices
- Always commit before major changes.
- Use version control for all code.
- Monitor system health regularly.
- Keep backups of critical data.

## Contact
We track work and issues in Beads instead of Markdown. MANDATORY - Run `bd quickstart` to see how.
