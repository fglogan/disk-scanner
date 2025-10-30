# Persistent Project Monitoring & Change Detection

## Why
Current project scanning is ephemeral - users lose all scan data when the app restarts. There's no way to track changes over time, detect project drift, or monitor storage trends. This limits the tool's value for ongoing project maintenance and prevents proactive storage management.

## What Changes
- **Project Persistence**: Save scanned projects to local database with full metadata
- **Change Detection**: Compare current scans with historical baselines to detect diffs
- **Dynamic Cards**: Real-time status cards showing trends, alerts, and change indicators
- **Timeline Tracking**: Historical view of project evolution over time
- **Smart Alerts**: Notifications for significant changes (size spikes, new issues, etc.)

## Impact
- Affected specs: [project-scanning, data-persistence, ui-components]
- Affected code: [ProjectScanner.svelte, new database layer, scan utilities]
- **BREAKING**: None - purely additive features
- Storage: Local SQLite database (~10-50MB per 1000 projects)
- Performance: Incremental scans 10x faster than full scans
- User value: Transforms from utility to monitoring platform