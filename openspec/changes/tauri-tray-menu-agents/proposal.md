# Tauri Tray Menu with Agent Integration
## OpenSpec Proposal Document

**Document ID:** OS-TRAY-2025-10-27-v1  
**Status:** PROPOSAL (Awaiting Stakeholder Review)  
**Version:** 1.0  
**Date:** October 27, 2025  
**Author:** VOS-Coder (AI Assistant)  
**Related Project**: Disk Bloat Scanner v0.1.1  
**Baseline Scope**: Add intelligent tray menu system with agent integration framework

---

## Executive Summary

The **Tauri Tray Menu with Agent Integration** system provides:

1. **Upper Tray Menu System**: Native macOS/Windows/Linux menu bar UI with status icons, quick actions, and contextual choosers
2. **Status Dashboard**: Real-time system and application state visible without opening main window
3. **Agent Integration Framework**: Headless agents instantiated via OpenCode API or Tauri plugin wrapper to assist with complex tasks
4. **Context-Aware Assistance**: Agents appear dynamically based on detected issues, user workflow, and available actions
5. **Future-Ready Architecture**: Extensible design for plugging in new agents and capabilities

### Primary Use Cases

1. **Quick Status Check** - See disk health, compliance score, recent issues from tray without opening app
2. **Context Assistance** - Agent appears suggesting "Large files found, want me to scan?" or "Phase regressed, need approval workflow?"
3. **Smart Actions** - Tray menu suggests next actions based on last scan results, EDGS phase, compliance status
4. **Background Scanning** - Configure background tasks (hourly compliance checks, PACS drift detection) via tray menu
5. **Multi-Project Management** - Switch between projects in tray dropdown, see status for each
6. **Agent Invocation** - Call headless agents for complex tasks without opening UI (e.g., "Generate compliance report")

---

## Problem Statement

### Current State

- Main window must be opened to see any application state
- No background task visibility or control
- User context switches lose continuity (scanning results not visible while working elsewhere)
- Complex tasks (compliance analysis, drift detection) require opening UI
- No native OS integration (menu bar, notification center)
- No agent framework for delegating complex work

### Target State

- ✅ Status visible at all times via tray icon and menu
- ✅ Background tasks configurable and monitorable from tray
- ✅ Context switches maintain state (jump back to scanner, see last results)
- ✅ Complex tasks delegated to headless agents
- ✅ Native OS integration (respects platform UI conventions)
- ✅ Extensible agent framework (add new agents without code changes)

---

## Solution Architecture

### Component 1: Tray Menu System (TMS)

**Purpose**: Native menu bar UI with status, quick actions, and settings

**Features**:
1. **Status Icon**
   - Green: All systems operational
   - Yellow: Warnings (phase needs completion, compliance score low)
   - Red: Critical issues (phase regressed, compliance violations)
   - Animated during scanning
   - Clickable: Expand to full menu

2. **Status Menu** (Primary tray dropdown)
   - **Current Status** section
     - Compliance Score: `78/100`
     - Last Scan: `2 minutes ago`
     - EDGS Phase: `Phase 2 (75% complete)`
     - Active Issues: `5` (link to details)
   
   - **Quick Actions** section
     - Scan Now (quick scan of default directory)
     - View Dashboard (open main window)
     - Run Background Job (if configured)
     - Configure...
   
   - **Projects** submenu (PACS feature)
     - Project A (Compliance: 85/100)
     - Project B (Compliance: 62/100) ← Red icon
     - Project C (Compliance: 91/100)
     - [Manage...]
   
   - **Agents** section (future)
     - Available Agents: [Agent 1], [Agent 2], ...
     - Run Agent > [submenu of agent actions]

3. **Context Choosers**
   - Project selector (for PACS multi-project support)
   - Scan profile selector (quick, full, compliance-check)
   - Agent selector (for manual invocation)
   - Export format selector (report generation)

**Tray Menu Structure**:
```
🟢 Disk Bloat Scanner
├─ Status Dashboard (opens main window)
│  └─ Compliance: 78/100 | Phase 2: 75% | Issues: 5
├─ ─────────────────────────
├─ Scan Now (default directory)
├─ Background Jobs
│  ├─ Compliance Check (Daily, 6am) [Toggle]
│  ├─ PACS Drift Detection (Hourly) [Toggle]
│  └─ Configure Schedule...
├─ ─────────────────────────
├─ Projects (PACS)
│  ├─ Project A (85/100) ✅
│  ├─ Project B (62/100) ⚠️
│  └─ Project C (91/100) ✅
├─ ─────────────────────────
├─ Agents
│  ├─ Available: 2 agents
│  ├─ Run Analysis > [submenu]
│  ├─ Generate Report > [submenu]
│  └─ Assist with Task...
├─ ─────────────────────────
├─ Settings
│  ├─ Scan Options
│  ├─ Background Jobs
│  ├─ Agent Configuration
│  └─ Preferences
├─ ─────────────────────────
├─ About Disk Bloat Scanner
├─ Check for Updates
└─ Quit
```

### Component 2: Agent Integration Framework (AIF)

**Purpose**: Extensible framework for invoking headless agents

**Architecture**:

```
┌─────────────────────────────────────────────────────┐
│         Disk Bloat Scanner Main Window              │
│    (Tauri Frontend - SvelteKit + TypeScript)        │
└────────┬────────────────────────────────────────────┘
         │
         │ Tauri Commands
         ↓
┌─────────────────────────────────────────────────────┐
│         Tauri Backend (Rust)                        │
│    (Commands, Event System, Tray Menu)              │
└────┬────────────────────┬────────────────────┬──────┘
     │                    │                    │
     │                    │                    │
     ↓                    ↓                    ↓
  [Local]          [Agent Broker]        [Plugins]
  Scan Cmd         (Tauri Backend)       Wrapper
  Cleanup Cmd
  Validate Cmd
                   ┌─────────────┐
                   │ Agent Pool  │
                   │  Instance   │
                   │  Manager    │
                   └──────┬──────┘
                          │
         ┌────────────────┼────────────────┐
         ↓                ↓                ↓
    [OpenCode]      [Local Agents]   [Future]
    HTTP API        (Subprocess)     Plugins
    Headless        Config:
    Agents          - Port
                    - Auth token
                    - Max instances
                    - Timeout
```

**Agent Broker** (New Module):
```rust
// src-tauri/src/agents/broker.rs
pub struct AgentBroker {
    config: AgentConfig,
    instances: HashMap<String, AgentInstance>,
    opencode_client: Optional<HttpClient>,
    plugin_wrapper: Optional<PluginWrapper>,
}

pub struct AgentInstance {
    id: String,
    agent_type: AgentType,      // OpenCode, Local, Plugin
    process: Optional<Child>,   // If subprocess
    status: AgentStatus,        // Running, Idle, Error
    capabilities: Vec<String>,  // Actions this agent can perform
}

pub enum AgentType {
    OpenCodeHeadless { url: String, auth: String },
    LocalSubprocess { binary_path: String },
    TauriPlugin { plugin_name: String },
}

pub async fn invoke_agent(
    &self,
    agent_name: &str,
    action: &str,
    context: AgentContext,
) -> Result<AgentResponse, AgentError>
```

**Agent Invocation Flow**:
1. User selects agent from tray menu or UI
2. Frontend calls `invoke_agent()` command
3. Broker looks up agent configuration
4. Broker instantiates agent (if not running):
   - OpenCode: HTTP POST to `/agent/invoke`
   - Local subprocess: Spawn process with IPC
   - Plugin: Call via Tauri plugin wrapper
5. Send context (current project, scan results, user intent)
6. Agent processes and returns result
7. Broker sends response back to frontend
8. Frontend displays result (modal, notification, inline)

**Agent Capabilities Registry**:
```json
{
  "agents": [
    {
      "name": "compliance-analyzer",
      "type": "opencode_headless",
      "url": "http://opencode.local:8000/agents/compliance",
      "capabilities": [
        "analyze_compliance",
        "generate_report",
        "suggest_fixes"
      ],
      "context_fields": ["project_path", "compliance_score", "violations"],
      "max_instances": 2,
      "timeout_seconds": 300
    },
    {
      "name": "drift-detector",
      "type": "opencode_headless",
      "url": "http://opencode.local:8000/agents/drift",
      "capabilities": ["detect_drift", "analyze_changes", "notify_hil"],
      "context_fields": ["baseline_id", "project_path"],
      "max_instances": 1,
      "timeout_seconds": 60
    },
    {
      "name": "suggestion-engine",
      "type": "local_subprocess",
      "binary_path": "/opt/disk-bloat-scanner/agents/suggestion-engine",
      "capabilities": ["suggest_next_actions", "prioritize_tasks"],
      "max_instances": 1,
      "timeout_seconds": 30
    }
  ]
}
```

### Component 3: Context-Aware Agent Activation

**Purpose**: Automatically suggest agents based on detected conditions

**Triggering Conditions**:

| Condition | Suggested Agent | Action |
|-----------|-----------------|--------|
| Large files detected | Analysis Agent | "Analyze storage pattern?" |
| Phase regressed | Workflow Agent | "Help with phase approval?" |
| Compliance violations | Fix Agent | "Suggest corrections?" |
| PACS drift detected | Drift Agent | "Review changes?" |
| User idle 5+ minutes | Background Tasks | "Run scheduled jobs?" |
| Dashboard opened | Context Agent | "Continue from last task?" |

**Implementation**:
- Track application state (last action, detected issues, time)
- When condition met, check available agents
- Display subtle notification (not intrusive)
- User can accept (run agent) or dismiss
- Log all agent suggestions for metrics

### Component 4: Background Job Scheduler

**Purpose**: Schedule and manage background tasks via tray menu

**Jobs Configurable**:
1. **Compliance Check** (nightly)
   - Run deep analyzer on configured projects
   - Generate compliance report
   - Alert if score drops >10%

2. **PACS Drift Detection** (hourly)
   - Compare projects against baselines
   - Flag anomalies
   - Send notifications to HIL

3. **Cleanup** (weekly)
   - Find large files, duplicates
   - Generate cleanup report
   - Require approval before deletion

**Configuration UI** (via tray menu):
```
Background Jobs
├─ Compliance Check
│  ├─ Enabled: [Toggle]
│  ├─ Schedule: Daily at 6:00 AM
│  ├─ Projects: [Select...] (A, B, C)
│  ├─ Notify on: [Violations] [New Issues] [Score Drop >10%]
│  └─ Save
├─ PACS Drift Detection
│  ├─ Enabled: [Toggle]
│  ├─ Interval: Every 1 hour
│  ├─ All projects: [Toggle]
│  └─ Save
└─ View Job Log
```

**Execution**:
- Jobs run via Tauri background process (no main window needed)
- Results stored in `.beads/jobs/` directory
- User notified via system notifications
- Results accessible from tray menu

---

## Feature Breakdown: 2 Features (Initial Release)

### Feature 1: Tray Menu UI (24 hrs)

**Purpose**: Native menu bar UI with status and quick actions

**Requirements**:
- macOS, Windows, Linux support (via `tauri-plugin-system-tray`)
- Status icon with multiple states (green, yellow, red)
- Animated icon during scanning
- Submenu support for Projects, Agents, Jobs
- Click-to-perform actions (Scan Now, View Dashboard, etc.)
- Context-aware tooltips

**Deliverables**:
- `src-tauri/src/tray/mod.rs` - Tray menu module
- `src-tauri/src/tray/menu_builder.rs` - Menu construction
- `src-tauri/src/tray/status_manager.rs` - Status icon state management
- `src-tauri/src/commands/tray_commands.rs` - Tauri command handlers
- Configuration: `tray-config.toml`
- 20+ unit & integration tests

**API Contract**:
```typescript
// Tauri commands
command update_tray_status(status: { score, phase, issues, last_scan })
command tray_action_triggered(action: string, context: object)
command get_tray_status() → { score, phase, issues, projects }

// Tauri events
event "tray:status_changed" → { new_status }
event "tray:action" → { action, context }
```

### Feature 2: Agent Integration Framework (30 hrs)

**Purpose**: Framework for invoking headless agents from tray/UI

**Requirements**:
- Agent broker with instance pooling
- Support for: OpenCode HTTP API, local subprocesses, Tauri plugins
- Agent registry (configuration-driven)
- Context-aware activation (suggest agents based on state)
- Error handling and retry logic
- Logging and metrics
- Future-ready for additional agent types

**Deliverables**:
- `src-tauri/src/agents/mod.rs` - Agents module
- `src-tauri/src/agents/broker.rs` - Agent broker
- `src-tauri/src/agents/opencode.rs` - OpenCode HTTP client
- `src-tauri/src/agents/subprocess.rs` - Local subprocess launcher
- `src-tauri/src/agents/registry.rs` - Agent registry
- `src-tauri/src/agents/activation.rs` - Context-aware activation
- Configuration: `agents-config.toml`
- Example agents directory: `agents-examples/`
- 30+ unit & integration tests

**API Contract**:
```typescript
// Tauri commands
command invoke_agent(agent_name: string, action: string, context: object)
  → { result, status, duration_ms }
command list_available_agents(context: object)
  → { agents: [{ name, capabilities, active }] }
command get_agent_suggestions(application_state: object)
  → { suggested_agents: [{ name, action, reason }] }

// Tauri events
event "agent:started" → { agent_name, instance_id }
event "agent:completed" → { agent_name, result }
event "agent:failed" → { agent_name, error }
event "agent:suggestion" → { agent_name, action, reason }
```

---

## Technical Implementation Strategy

### Technology Stack

| Component | Tech | Justification |
|-----------|------|---------------|
| Tray Menu | `tauri-plugin-system-tray` | Cross-platform native menus |
| Status State | `tokio::sync::RwLock` | Thread-safe status updates |
| Agent HTTP | `reqwest` with async | Async HTTP to OpenCode |
| Subprocesses | `tokio::process` | Async subprocess management |
| Configuration | `serde_json`, `toml` | Multi-format config |
| Logging | `log` crate | Structured logging |
| Frontend Events | Tauri events | Real-time UI updates |

### Modular Architecture

```
src-tauri/src/
├── tray/
│   ├── mod.rs                 ← Tray module exports
│   ├── menu_builder.rs        ← Build menu structure
│   └── status_manager.rs      ← Manage icon state
├── agents/
│   ├── mod.rs                 ← Agents module exports
│   ├── broker.rs              ← Main broker logic
│   ├── opencode.rs            ← OpenCode HTTP client
│   ├── subprocess.rs          ← Local agent spawning
│   ├── registry.rs            ← Agent configuration
│   └── activation.rs          ← Context-aware activation
└── commands/
    └── tray_commands.rs       ← Tauri command handlers
```

### Configuration Files

**`tray-config.toml`**:
```toml
[tray]
enabled = true
refresh_interval_ms = 5000
show_in_menu_bar = true

[tray.status_icons]
green = "path/to/green.png"
yellow = "path/to/yellow.png"
red = "path/to/red.png"
scanning = "path/to/scanning.gif"

[tray.menu_items]
show_projects = true
show_agents = true
show_background_jobs = true
show_settings = true
```

**`agents-config.toml`**:
```toml
[agents.registry]
path = ".beads/agents/"
auto_discover = true
max_retries = 3

[[agents.available]]
name = "compliance-analyzer"
type = "opencode_headless"
url = "http://opencode.local:8000/agents/compliance"
capabilities = ["analyze_compliance", "generate_report"]
max_instances = 2
timeout_seconds = 300

[[agents.available]]
name = "suggestion-engine"
type = "local_subprocess"
binary_path = "/opt/agents/suggestion-engine"
capabilities = ["suggest_next_actions"]
max_instances = 1
timeout_seconds = 30

[agents.activation_rules]
enable_auto_suggestions = true
suggestion_cooldown_seconds = 300
track_metrics = true
```

---

## Integration Points

### 1. Disk Bloat Scanner Integration
- Tray menu shows current scan status
- Latest scan results accessible without opening window
- "Scan Now" action triggers background scan

### 2. PACS Integration
- Project dropdown in tray shows all projects
- Per-project compliance scores visible
- Drift detection status shown per project
- Projects submenu allows quick project switching

### 3. EDGS Integration
- EDGS phase shown in status
- Phase progression tracked and visible
- Gate approvals accessible from agent framework
- Compliance scoring reflects EDGS status

### 4. Agent Integration (Future)
- Agents instantiated on-demand from OpenCode
- Agent results fed back to Disk Bloat Scanner state
- Audit trail maintained for agent actions
- Agent performance metrics tracked

---

## Platform-Specific Considerations

### macOS
- Menu bar icon (top right)
- Native context menu
- Notification Center integration
- Background app refresh capability

### Windows
- System tray icon (bottom right)
- Right-click context menu
- Toast notifications
- Task scheduler for background jobs

### Linux
- System tray (GNOME, KDE, etc.)
- Native menu
- Desktop notifications
- Cron/systemd for background jobs

**Implementation**: Use `tauri-plugin-system-tray` for abstraction

---

## Security & Privacy

### Agent Invocation Security
- ✅ Agent calls authenticated (API key, OAuth)
- ✅ Context data sanitized before sending
- ✅ TLS/HTTPS for remote agents
- ✅ Agent responses validated (type checking, size limits)
- ✅ Sensitive data never logged
- ✅ User approval for external agent invocation

### Background Job Security
- ✅ Jobs run with same permissions as user
- ✅ Results stored in `.beads/` (user-owned directory)
- ✅ Jobs audited (what ran, when, result)
- ✅ User can disable jobs at any time

### Tray Menu Security
- ✅ Menu items reflect actual state (no spoofing)
- ✅ Click actions verified (CSRF protection not needed for local)
- ✅ Status icons cannot be manipulated externally

---

## Success Criteria

### Feature 1 (Tray Menu) Acceptance Tests

1. ✅ Tray icon visible in menu bar/system tray
2. ✅ Click tray icon → menu appears
3. ✅ Menu shows current compliance score, phase, issues
4. ✅ "Scan Now" action starts background scan
5. ✅ "View Dashboard" opens main window
6. ✅ Projects submenu shows all projects
7. ✅ Status icon changes color based on state
8. ✅ Icon animated during active scan
9. ✅ Works on macOS, Windows, Linux
10. ✅ All 20+ tests pass

### Feature 2 (Agent Framework) Acceptance Tests

1. ✅ Agent registry loads from config
2. ✅ Agent instance pooling works (max instances enforced)
3. ✅ OpenCode HTTP agent invocation succeeds
4. ✅ Local subprocess agent invocation succeeds
5. ✅ Agent context properly serialized/passed
6. ✅ Agent response properly deserialized
7. ✅ Error handling: agent timeout → user notification
8. ✅ Error handling: agent unreachable → fallback
9. ✅ Context-aware activation suggests agents
10. ✅ Metrics tracked: invocations, success rate, latency
11. ✅ All 30+ tests pass

---

## Risk Mitigation

### Risk 1: Agent Latency (Unresponsive UI)

**Mitigation**:
- Agent invocation async with timeout
- Spinner/loading indicator shown immediately
- "Cancel" button available
- Timeout configurable per agent
- Fallback to local operation if agent fails

### Risk 2: Agent Availability (OpenCode Down)

**Mitigation**:
- Graceful degradation (app works without agents)
- Retry logic with exponential backoff
- Cache agent availability status
- Alert user if preferred agent unavailable
- Suggest alternative agents

### Risk 3: Over-Aggressive Agent Suggestions

**Mitigation**:
- Suggestion frequency configurable
- Cooldown period between suggestions (default: 5 min)
- User can disable suggestions
- Only suggest when truly helpful
- Track suggestion acceptance rate

### Risk 4: Background Job Resource Usage

**Mitigation**:
- Resource limits configurable (CPU, memory)
- Jobs run at low priority
- Only one background job at a time
- User can pause/disable jobs
- Monitor system impact

---

## Deployment & Integration

### Phase Allocation
- **Phase 3 Integration**: Add after Phase 2 Gate completion
- **Can work in parallel**: Independent of PACS, Phase 2 refactoring

### Backward Compatibility
- ✅ No breaking changes to existing commands
- ✅ Tray menu optional (can be disabled)
- ✅ Agents optional (app works without them)
- ✅ Graceful fallback for unavailable features

### Configuration Management
- Default configs in binary
- User overrides in `.beads/tray-config.toml` and `.beads/agents-config.toml`
- Environment variable overrides for testing
- Web UI for configuration (future)

---

## Deliverables Summary

### Code
- [ ] 1,200+ lines Rust (tray module)
- [ ] 1,500+ lines Rust (agent framework)
- [ ] Configuration files (TOML)
- [ ] 50+ unit & integration tests
- [ ] Example agents (stubs for integration)

### Documentation
- [ ] Tray Menu User Guide
- [ ] Agent Integration Guide
- [ ] Configuration Reference
- [ ] Developer Guide (adding new agents)
- [ ] API Reference

### Tools & Utilities
- [ ] Agent registry editor
- [ ] Background job monitor
- [ ] Agent testing harness
- [ ] Configuration validator

---

## Next Steps

1. **Stakeholder Review**: Present proposal to project team
2. **Design Approval**: Get approval on architecture
3. **Environment Setup**: Configure OpenCode local instance (if using)
4. **Sprint Planning**: Break into Beads issues
5. **Sprint 1**: Implement Tray Menu (Feature 1)
6. **Sprint 2**: Implement Agent Framework (Feature 2)
7. **Integration**: Connect agents to Disk Bloat Scanner
8. **Testing**: End-to-end testing on all platforms
9. **Deployment**: Release with next version

---

## References

- **Tauri Documentation**: https://tauri.app/
- **tauri-plugin-system-tray**: https://github.com/tauri-apps/plugins-workspace
- **OpenCode Documentation**: https://opencode.ai/docs/
- **EDGS Methodology**: EDGS_SCHEDULE.md
- **Disk Bloat Scanner**: README.md

---

**Document Status**: Ready for Stakeholder Review  
**Next Review**: Upon HIL Approval  
**Last Updated**: October 27, 2025

