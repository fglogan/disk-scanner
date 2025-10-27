# Tauri Tray Menu with Agent Integration
## OpenSpec Specification - Complete

**Status**: ✅ SPECIFICATION COMPLETE & READY FOR STAKEHOLDER REVIEW  
**Date**: October 27, 2025  
**Location**: `openspec/changes/tauri-tray-menu-agents/`  

---

## What is This Feature?

The **Tauri Tray Menu with Agent Integration** system adds an intelligent upper tray menu (menu bar on macOS, system tray on Windows/Linux) that provides:

### 1. **Status Dashboard in Menu Bar**
- See application status without opening main window
- Compliance score, EDGS phase, active issues visible at a glance
- Status icon that changes color (green/yellow/red) based on state
- Icon animates during active scans

### 2. **Quick Actions from Tray**
- **Scan Now**: Start a background scan
- **View Dashboard**: Open main window
- **Configure**: Quick access to settings
- **Background Jobs**: Enable/disable automated tasks
- **Project Switcher**: Jump between projects (PACS integration)

### 3. **Context-Aware Agent Activation**
- Application monitors state (compliance score, PACS drift, etc.)
- When issues detected, suggests relevant agents
- Example: "Compliance score dropped! Want me to analyze?"
- User can accept (run agent) or dismiss

### 4. **Agent Integration Framework**
- Support for headless agents (OpenCode HTTP API, local subprocesses, Tauri plugins)
- Agent broker manages instances, pooling, lifecycle
- Extensible registry for adding new agents
- Agents run without opening main window

### 5. **Background Job Scheduler**
- Configure tasks to run automatically
- Examples: Nightly compliance check, hourly PACS drift detection
- Monitor job status from tray menu
- View job results without opening app

---

## Why This Matters

### Current State Problems
- User must open main window to see any status
- No background task visibility or control
- Complex analysis tasks require opening UI
- No native OS integration (menu bar, system tray)
- No framework for delegating work to agents

### How This Feature Solves It

| Problem | Solution |
|---------|----------|
| Status invisible | Tray icon shows real-time status + menu |
| No background tasks | Configurable job scheduler in tray |
| UI context switching | State remains visible, accessible from menu |
| Complex tasks manual | Agents suggested automatically, runnable from menu |
| No OS integration | Native menu bar/system tray implementation |
| Limited extensibility | Agent registry framework for adding capabilities |

---

## Feature Breakdown

### Feature 1: Tray Menu UI (7 Issues, 24 Event Cycles)

**What it does**:
- Adds native menu bar icon (macOS) or system tray icon (Windows/Linux)
- Menu shows current status: compliance score, EDGS phase, recent issues
- Quick action buttons: Scan Now, View Dashboard, Configure
- Project dropdown (if using PACS)
- Background job toggles
- Status icon changes color and animates

**Key Components**:
- **Status Manager**: Tracks application state, selects icon
- **Menu Builder**: Constructs menu dynamically from state
- **Status Icons**: Green (healthy), Yellow (warning), Red (critical)
- **Tray Commands**: Route menu clicks to appropriate actions
- **Configuration**: Customizable via `tray-config.toml`

**Deliverables**:
- 1,200+ lines Rust code
- 4 new modules
- 40+ unit & integration tests
- Configuration system
- Cross-platform support (macOS, Windows, Linux)

### Feature 2: Agent Integration Framework (6 Issues, 30 Event Cycles)

**What it does**:
- Framework for invoking headless agents (without opening UI)
- Agent broker manages instance pooling and lifecycle
- Supports multiple agent types: OpenCode HTTP, local subprocess, Tauri plugins
- Automatic agent suggestions based on application state
- Extensible registry for adding new agents

**Key Components**:
- **Agent Broker**: Orchestrator for agent lifecycle
- **Agent Registry**: Configuration-driven agent definitions
- **OpenCode Client**: HTTP client for remote agents
- **Subprocess Launcher**: Run local agent binaries
- **Activation Engine**: Suggest agents based on state
- **Instance Pooling**: Manage agent instances efficiently

**Deliverables**:
- 1,500+ lines Rust code
- 6 new modules
- 50+ unit & integration tests
- Agent registry configuration
- Example agent stubs
- Integration with OpenCode

---

## Architecture Overview

```
┌─────────────────────────────────────────────────┐
│    Disk Bloat Scanner Main Application          │
│           (Tauri Tray + UI)                     │
└────────┬────────────────────────────────────────┘
         │
    ┌────┴─────────────────────────────────────────┐
    │                                              │
    ↓                                              ↓
[Tray Menu System]                        [Agent Integration Framework]
├─ Status Icon                            ├─ Agent Broker
├─ Menu Builder                           ├─ Agent Registry
├─ Status Manager                         ├─ OpenCode HTTP Client
├─ Background Jobs                        ├─ Subprocess Launcher
└─ Configuration                          ├─ Activation Engine
                                          └─ Plugin Wrapper Support

    ↓                                          ↓
[Native OS Tray]                          [External Agents]
├─ macOS Menu Bar                         ├─ OpenCode Headless API
├─ Windows System Tray                    ├─ Local Subprocess
└─ Linux System Tray                      └─ Tauri Plugin Wrapper
```

---

## Key Use Cases

### Use Case 1: Quick Status Check
User wants to see if disk bloat scanner is working without opening app:
1. Click tray icon → menu appears
2. See: Compliance: 78/100, Phase 2 (75%), Issues: 5
3. Close menu without opening app

### Use Case 2: Agent-Assisted Analysis
Compliance score drops suddenly:
1. Application detects drop and activates agent suggestion
2. Notification: "Compliance analysis needed! Run analyzer?"
3. User clicks → Agent runs in background
4. Results appear in notification
5. User can accept recommendations without opening UI

### Use Case 3: Project Switching (PACS)
User switching between projects:
1. Click "Projects" in tray menu
2. See list: Project A (85/100), Project B (62/100), Project C (91/100)
3. Click Project B
4. Tray updates to show Project B's status
5. Background jobs now run on Project B

### Use Case 4: Scheduled Compliance Checks
Organization wants nightly compliance report:
1. Configure via tray: "Compliance Check" → Daily at 6am
2. System runs check in background (no UI needed)
3. User receives notification with results
4. Results visible in job log

### Use Case 5: Drift Detection
PACS monitoring organization projects:
1. Configure via tray: "Drift Detection" → Every hour
2. Background job compares projects against baselines
3. If drift detected: notification with details
4. HIL can review and approve changes via agent

---

## Technical Highlights

### 1. Native Cross-Platform Menus
- Uses `tauri-plugin-system-tray` for native implementation
- Respects platform conventions (macOS menu bar, Windows system tray)
- No custom drawing (uses OS native UI)

### 2. Efficient Status Management
- Status icon cached
- Menu refreshes only when state changes
- Animation smooth (30 FPS)
- Low CPU/memory overhead

### 3. Agent Flexibility
- **OpenCode HTTP**: Remote agents via HTTP API
- **Local Subprocess**: Run agents as separate processes
- **Tauri Plugin Wrapper**: Integration with plugin system
- **Future-Proof**: Easy to add new agent types

### 4. Instance Pooling
- Agent broker maintains instance pool
- Respects max_instances per agent
- Lazy instantiation (create only when needed)
- Automatic cleanup of idle instances

### 5. Context-Aware Suggestions
- Monitor application state continuously
- Suggest agents when conditions met (e.g., compliance < 60)
- Cooldown prevents notification spam (default: 5 min)
- Track metrics: suggestion rate, acceptance rate

---

## Configuration

### Tray Menu Configuration (`tray-config.toml`)
```toml
[tray]
enabled = true
refresh_interval_ms = 5000
show_projects = true
show_agents = true
show_background_jobs = true

[tray.status_thresholds]
critical_score = 60.0      # Score below this = red icon
warning_score = 80.0       # Score below this = yellow icon
```

### Agent Registry Configuration (`agents-config.toml`)
```toml
[[agents.available]]
name = "compliance-analyzer"
type = "opencode_headless"
url = "http://opencode.local:8000/agents/compliance"
capabilities = ["analyze_compliance", "generate_report"]
max_instances = 2
timeout_seconds = 300
```

### Background Jobs Configuration (UI)
```
Configure via tray menu:
├─ Compliance Check
│  ├─ Enabled: [Toggle]
│  ├─ Schedule: Daily at 6:00 AM
│  ├─ Projects: [Select...]
│  └─ Save
├─ Drift Detection
│  ├─ Enabled: [Toggle]
│  ├─ Interval: Every 1 hour
│  └─ Save
```

---

## Integration Points

### With Disk Bloat Scanner
- Tray shows real-time scan status
- Latest scan results accessible without opening window
- "Scan Now" action triggers background scan

### With PACS (Project Auditor & Compliance Scanner)
- Project dropdown in tray menu
- Per-project compliance scores visible
- Quick project switching
- Background drift detection for all projects

### With EDGS (Event-Driven Gated Scheduling)
- EDGS phase shown in status
- Phase progression visible
- Gate approvals accessible via agent
- Compliance scoring reflects EDGS status

### With OpenCode
- Agents instantiated from OpenCode headless API
- Agent capabilities registry
- Authentication via API keys
- Future: Full OpenCode integration

---

## Specifications

All detailed specifications are in:

| Document | Size | Purpose |
|----------|------|---------|
| `proposal.md` | 44 KiB | Complete architecture & design |
| `tasks.md` | 55 KiB | 13 Beads issues with dependencies |
| This file | ~8 KiB | Quick reference |

---

## What's Needed for Implementation

### Development Requirements
- Tauri 2.x.x environment
- Rust 1.89.0+
- OpenCode local instance (for testing OpenCode agent integration)
- macOS/Windows/Linux for cross-platform testing

### Dependencies
- `tauri-plugin-system-tray` (tray menu)
- `reqwest` (HTTP client for OpenCode)
- `tokio` (async runtime)
- `serde` (JSON serialization)
- `chrono` (scheduling)

### Testing Infrastructure
- Mock HTTP server (for agent testing)
- Mock subprocess binaries
- Cross-platform CI/CD

---

## Success Criteria

### Feature 1 (Tray Menu): ✅ Success When...
1. Status icon visible in menu bar/system tray
2. Tray icon changes color based on compliance score
3. Menu dropdown shows current state (score, phase, issues)
4. "Scan Now" triggers background scan
5. "View Dashboard" opens main window
6. Works on macOS, Windows, Linux
7. All 40+ tests pass

### Feature 2 (Agent Framework): ✅ Success When...
1. Agent registry loads from configuration
2. OpenCode HTTP agent invocation succeeds
3. Local subprocess agent invocation succeeds
4. Instance pooling enforces max instances
5. Context-aware suggestions appear when appropriate
6. Agent results appear in UI/notifications
7. All 50+ tests pass

---

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Agent latency | Async with timeout, spinner shown, cancel button |
| OpenCode unavailable | Graceful degradation, fallback to local, retry logic |
| Agent suggestions spam | Cooldown (5 min default), frequency configurable, disable option |
| Background job resource usage | Low priority, resource limits, only one job at a time |
| Cross-platform issues | Use tauri-plugin-system-tray, extensive testing |

---

## Next Steps

1. **Stakeholder Review**: Present specification to team
2. **Design Approval**: Feedback and approval on architecture
3. **Environment Setup**: Configure development environment
4. **Sprint Planning**: Break into Beads issues, allocate to sprints
5. **Sprint 1**: Implement Feature 1 (Tray Menu UI)
6. **Sprint 2**: Implement Feature 2 (Agent Framework)
7. **Integration**: Connect to Disk Bloat Scanner
8. **Testing**: Cross-platform testing
9. **Deployment**: Release in next version

---

## Summary

The **Tauri Tray Menu with Agent Integration** feature brings:

✅ **Status visibility** - Know app state without opening window  
✅ **Background tasks** - Configure and monitor automated jobs  
✅ **Agent framework** - Run headless agents without UI  
✅ **Context awareness** - Agents suggested automatically  
✅ **Native OS integration** - Respects platform conventions  
✅ **Extensibility** - Easy to add new agents and capabilities  

**Total Implementation**:
- 2 Features
- 13 Beads Issues
- 54 Event Cycles
- 2,700+ lines Rust
- 90+ tests
- 20+ pages documentation

---

**Status**: 🟢 **READY FOR STAKEHOLDER REVIEW**  
**Next Action**: Present proposal, gather feedback, request Gate 0 approval  
**Last Updated**: October 27, 2025, 22:30 UTC  

