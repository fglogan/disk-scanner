# Tauri Tray Menu with Agent Integration: Beads Task Breakdown

**Document ID:** TRAY-AGENTS-BEADS-2025-10-27-v1  
**Total Issues:** 13  
**Total Effort:** 54 event-driven cycles (no time estimates per EDGS)  
**Organization:** 2 Features, 2 Sprints

---

## Feature 1: Tray Menu UI (7 Issues)

### BEAD-TRAY-001: Foundation & Data Models

**Title**: Create tray module foundation: models, error types, core structures  
**Feature**: Tray Menu UI  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: None  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create foundational Rust modules for tray system:
- `src-tauri/src/tray/mod.rs` - Tray module exports
- `src-tauri/src/models/tray_state.rs` - TrayStatus, StatusIcon, MenuItem data structures
- `src-tauri/src/models/agent_state.rs` - Agent registration, capability models
- Add custom error type to `error.rs` (TrayError variant)

**Acceptance Criteria**:
- [ ] All model files created and compile without warnings
- [ ] Structures use derive(Serialize, Deserialize, Debug) for IPC
- [ ] Documentation comments on all public types
- [ ] Unit tests for model serialization (5+ tests)
- [ ] Type-safe enums for Status (Green, Yellow, Red, Scanning)

**References**:
- See proposal.md for data structures
- TrayStatus fields: score, phase, issues, projects

---

### BEAD-TRAY-002: Menu Builder Module

**Title**: Create menu structure builder and item management  
**Feature**: Tray Menu UI  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: BEAD-TRAY-001  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/tray/menu_builder.rs` to build and manage menu:

**Capabilities**:
- Build menu structure: Status Dashboard, Scan Now, Projects, Agents, Settings, etc.
- Submenu support: Projects list, Agent actions, Background jobs
- Dynamic item generation (projects, agents loaded from registry)
- Menu item enable/disable based on state (e.g., disable "Scan Now" if already scanning)
- Click handlers routing to appropriate commands

**Functions**:
```rust
pub fn build_main_menu(state: &TrayState) -> CustomMenu
pub fn build_projects_submenu(projects: &[ProjectInfo]) -> CustomMenu
pub fn build_agents_submenu(available_agents: &[AgentInfo]) -> CustomMenu
pub fn build_background_jobs_submenu(jobs: &[JobInfo]) -> CustomMenu
pub fn update_menu_item_state(menu: &mut CustomMenu, item_id: &str, enabled: bool)
pub fn handle_menu_click(item_id: &str, tray_state: &TrayState) → MenuAction
```

**Menu Structure**:
```
Disk Bloat Scanner
├─ Status Dashboard (opens main window)
│  └─ [Dynamic status text]
├─ ─────────────────────────
├─ Scan Now
├─ Background Jobs
│  ├─ Compliance Check [Toggle]
│  ├─ PACS Drift Detection [Toggle]
│  └─ Configure...
├─ Projects
│  ├─ [Project A] (85/100)
│  ├─ [Project B] (62/100)
│  └─ Manage...
├─ Agents
│  ├─ Available: [count]
│  └─ Run Agent > [submenu]
├─ Settings
├─ About
└─ Quit
```

**Acceptance Criteria**:
- [ ] Main menu builds without errors
- [ ] Submenu generation dynamic and configurable
- [ ] Menu items enable/disable based on state
- [ ] Click handlers properly routed
- [ ] Menu items reflect current state (e.g., toggle shows actual state)
- [ ] Unit tests for menu building (15+ tests)

**References**:
- Tauri CustomMenu API
- Menu item IDs standardized for testing
- State management from BEAD-TRAY-001

---

### BEAD-TRAY-003: Status Manager & Icon Management

**Title**: Implement status state management and icon switching  
**Feature**: Tray Menu UI  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: BEAD-TRAY-001, BEAD-TRAY-002  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/tray/status_manager.rs` to manage status and icon:

**Capabilities**:
- Track current application state (compliance score, phase, issues, etc.)
- Update tray icon based on status (green/yellow/red)
- Animate icon during scanning/processing
- React to state changes and update menu
- Serialize/deserialize state for persistence

**Functions**:
```rust
pub struct StatusManager {
    current_status: RwLock<TrayStatus>,
    icon_cache: HashMap<StatusIcon, Image>,
    animation_handle: Optional<JoinHandle>,
}

pub async fn update_status(
    &self,
    new_status: TrayStatus,
) -> Result<(), TrayError>

pub async fn start_scanning_animation(&self) → Result<(), TrayError>

pub async fn stop_scanning_animation(&self) → Result<(), TrayError>

pub fn get_current_status(&self) → TrayStatus

pub fn select_icon(&self, status: &TrayStatus) → &Image
```

**Icon States**:
- **Green**: compliance_score >= 80, no critical issues
- **Yellow**: compliance_score 60-79 OR has high-priority issues
- **Red**: compliance_score < 60 OR has critical issues
- **Scanning** (animated): Active scan in progress
- **Error** (flashing red): Agent failure or critical error

**Status Calculation**:
```rust
fn calculate_status(
    compliance_score: f32,
    phase: String,
    violations: Vec<Violation>,
) -> TrayStatus {
    if has_critical_violations(&violations) {
        Red
    } else if compliance_score < 60.0 || has_high_priority_violations(&violations) {
        Yellow
    } else {
        Green
    }
}
```

**Acceptance Criteria**:
- [ ] Status manager tracks all required fields
- [ ] Icon selection logic correct for all states
- [ ] Scanning animation smooth (30 FPS minimum)
- [ ] State updates trigger menu refresh
- [ ] Compliance score threshold logic working
- [ ] Unit tests for status calculation (10+ tests)
- [ ] Integration test: status change → icon update

**References**:
- Icon files in `src-tauri/icons/tray/`
- Animation frames for scanning state

---

### BEAD-TRAY-004: Tray Commands & Event Handlers

**Title**: Implement Tauri commands for tray menu interactions  
**Feature**: Tray Menu UI  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0  
**Effort**: Event-driven  
**Dependencies**: BEAD-TRAY-001, BEAD-TRAY-002, BEAD-TRAY-003  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/commands/tray_commands.rs` - main orchestrator:

**Tauri Commands**:
```rust
#[tauri::command]
async fn update_tray_status(
    status: TrayStatus,
    tray_handle: tauri::SystemTrayHandle,
) → Result<(), String>

#[tauri::command]
async fn handle_tray_action(
    action: String,
    context: Option<Value>,
) → Result<Value, String>

#[tauri::command]
async fn get_tray_status() → Result<TrayStatus, String>

#[tauri::command]
async fn get_menu_state() → Result<MenuState, String>
```

**Events Emitted**:
```rust
event "tray:status_changed" → { new_status }
event "tray:action" → { action, context }
event "tray:project_switched" → { project_id }
event "tray:background_job_toggled" → { job_name, enabled }
```

**Action Routing**:
- "scan_now" → Trigger background scan
- "view_dashboard" → Open main window
- "toggle_job" → Enable/disable background job
- "switch_project" → Change active project
- "run_agent" → Invoke agent (see BEAD-AGENTS-005)

**Acceptance Criteria**:
- [ ] All commands handle errors gracefully
- [ ] Events emitted correctly for all actions
- [ ] Context properly serialized/deserialized
- [ ] Commands accessible from UI and tray
- [ ] Integration test: tray action → command → result
- [ ] All 10+ tests pass

**References**:
- Tauri command attribute macro
- Event system documentation

---

### BEAD-TRAY-005: Configuration Management

**Title**: Load and manage tray configuration  
**Feature**: Tray Menu UI  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0  
**Effort**: Event-driven  
**Dependencies**: BEAD-TRAY-001  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create tray configuration system:

**Files**:
- `src-tauri/src/config/tray_config.rs` - Configuration loading
- `tray-config.toml` - Default configuration
- `.beads/tray-config.toml` - User overrides

**Configuration Options**:
```toml
[tray]
enabled = true
refresh_interval_ms = 5000
show_in_menu_bar = true
show_projects = true
show_agents = true

[tray.status_thresholds]
critical_score = 60.0
warning_score = 80.0
```

**Loading Logic**:
1. Load defaults from embedded config
2. Override with `.beads/tray-config.toml` if exists
3. Override with environment variables
4. Validate configuration
5. Store in singleton ConfigManager

**Acceptance Criteria**:
- [ ] Default config embedded in binary
- [ ] User config in `.beads/` loads correctly
- [ ] Environment variable overrides work
- [ ] Validation catches invalid configs
- [ ] Config reloadable without restart
- [ ] Unit tests for config loading (8+ tests)

**References**:
- `serde` for TOML parsing
- Embedded config in binary using `include_str!`

---

### BEAD-TRAY-006: Background Job Scheduler

**Title**: Implement background job scheduling and execution  
**Feature**: Tray Menu UI  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P1  
**Effort**: Event-driven  
**Dependencies**: BEAD-TRAY-001, BEAD-TRAY-004  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create background job scheduler:

**Files**:
- `src-tauri/src/scheduler/mod.rs` - Scheduler module
- `src-tauri/src/scheduler/jobs.rs` - Job definitions
- `src-tauri/src/scheduler/executor.rs` - Job execution

**Job Types**:
1. **Compliance Check**
   - Run deep analyzer on projects
   - Generate report
   - Alert if score drops

2. **PACS Drift Detection**
   - Compare projects against baselines
   - Flag anomalies
   - Send notifications

3. **Cleanup Suggestion**
   - Find large files, duplicates
   - Generate recommendations
   - Wait for user approval

**Functions**:
```rust
pub struct JobScheduler {
    jobs: Vec<ScheduledJob>,
    executor: Executor,
}

pub async fn schedule_job(
    &mut self,
    job_name: &str,
    schedule: &str,
) → Result<(), Error>

pub async fn execute_job(&self, job_name: &str) → Result<JobResult, Error>

pub async fn toggle_job(&mut self, job_name: &str, enabled: bool) → Result<(), Error>

pub fn get_job_status(&self, job_name: &str) → JobStatus

pub fn list_jobs(&self) → Vec<JobInfo>
```

**Scheduling Format**:
```toml
[[jobs]]
name = "compliance-check"
enabled = true
schedule = "0 6 * * *"      # Cron format
projects = ["all"]
notify_on = ["violations", "score_drop"]

[[jobs]]
name = "drift-detection"
enabled = true
schedule = "0 * * * *"      # Every hour
projects = ["all"]
```

**Job Execution**:
- Run in background (no main window needed)
- Use low priority thread pool
- Store results in `.beads/jobs/`
- Notify user via system notifications
- Maintain job log

**Acceptance Criteria**:
- [ ] Jobs schedule correctly
- [ ] Jobs execute at scheduled time
- [ ] Toggle enable/disable works
- [ ] Results stored with timestamp
- [ ] User notifications sent
- [ ] Job logs queryable
- [ ] Unit tests for scheduler (12+ tests)

**References**:
- `chrono` for scheduling
- `tokio::task` for background execution

---

### BEAD-TRAY-007: Tray Menu Tests & Documentation

**Title**: Comprehensive testing and documentation for tray system  
**Feature**: Tray Menu UI  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0  
**Effort**: Event-driven  
**Dependencies**: BEAD-TRAY-001 through BEAD-TRAY-006  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create 40+ tests and comprehensive documentation:

**Test Suite** (40+ tests):
- Model serialization (5 tests)
- Menu building (15 tests)
- Status calculation (10 tests)
- Command handling (8 tests)
- Configuration loading (5 tests)
- Scheduler (12 tests)
- Integration tests (platform-specific)

**Test Fixtures**:
- Mock applications in various states
- Sample configuration files
- Fake OpenCode responses (for agent testing)

**Documentation**:
- User Guide: How to use tray menu
- Configuration Reference: All tray-config.toml options
- Developer Guide: Adding new menu items/jobs
- Troubleshooting: Common issues and solutions

**Acceptance Criteria**:
- [ ] All 40+ tests pass
- [ ] Coverage >70% for tray module
- [ ] User guide is clear and complete
- [ ] Configuration reference documented
- [ ] Developer guide includes examples
- [ ] Works on macOS, Windows, Linux

**References**:
- Tauri testing patterns
- Platform-specific tray menu APIs

---

## Feature 2: Agent Integration Framework (6 Issues)

### BEAD-AGENTS-001: Agent Foundation & Registry

**Title**: Create agent framework foundation and registry system  
**Feature**: Agent Integration  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: BEAD-TRAY-001  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create foundational agent framework:

**Files**:
- `src-tauri/src/agents/mod.rs` - Agents module
- `src-tauri/src/agents/registry.rs` - Agent registry
- `src-tauri/src/models/agent_config.rs` - Agent configuration models
- `agents-config.toml` - Agent definitions

**Capabilities**:
- Load agent definitions from config (TOML)
- Registry tracks all available agents
- Query agents by capability
- Filter agents by type (OpenCode, local, plugin)
- Validate agent configurations

**Functions**:
```rust
pub struct AgentRegistry {
    agents: HashMap<String, AgentConfig>,
    config_path: PathBuf,
}

pub fn load_registry(config_path: &Path) → Result<Self, Error>

pub fn get_agent(&self, name: &str) → Option<&AgentConfig>

pub fn list_agents_with_capability(&self, capability: &str) → Vec<&AgentConfig>

pub fn validate_config(&self) → Result<(), Vec<String>>

pub fn reload(&mut self) → Result<(), Error>
```

**Agent Configuration Structure**:
```toml
[[agents.available]]
name = "compliance-analyzer"
type = "opencode_headless"
url = "http://opencode.local:8000/agents/compliance"
capabilities = ["analyze_compliance", "generate_report"]
context_fields = ["project_path", "compliance_score"]
max_instances = 2
timeout_seconds = 300
auth_type = "api_key"
auth_key_env = "OPENCODE_API_KEY"

[[agents.available]]
name = "suggestion-engine"
type = "local_subprocess"
binary_path = "/opt/agents/suggestion-engine"
capabilities = ["suggest_next_actions"]
max_instances = 1
timeout_seconds = 30
```

**Acceptance Criteria**:
- [ ] Registry loads from TOML without errors
- [ ] All agent fields accessible
- [ ] Query by capability works
- [ ] Filter by type works
- [ ] Validation catches invalid configs
- [ ] Unit tests for registry (15+ tests)

**References**:
- See proposal.md for agent types and structure

---

### BEAD-AGENTS-002: Agent Broker & Instance Management

**Title**: Implement agent broker with instance pooling  
**Feature**: Agent Integration  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: BEAD-AGENTS-001  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/agents/broker.rs` - main agent broker:

**Capabilities**:
- Maintain pool of agent instances
- Respect max_instances per agent
- Lazy instantiation (only create when needed)
- Track instance state (running, idle, error)
- Clean up unused instances
- Handle agent lifecycle

**Functions**:
```rust
pub struct AgentBroker {
    registry: AgentRegistry,
    instances: Arc<RwLock<HashMap<String, Vec<AgentInstance>>>>,
    http_client: reqwest::Client,
}

pub async fn invoke_agent(
    &self,
    agent_name: &str,
    action: &str,
    context: AgentContext,
) → Result<AgentResponse, AgentError>

pub async fn get_agent_status(&self, agent_name: &str) → AgentStatus

pub async fn list_running_instances(&self) → Vec<InstanceInfo>

pub async fn shutdown_agent(&self, agent_name: &str) → Result<(), Error>

async fn get_or_create_instance(&self, config: &AgentConfig) → Result<AgentInstance, Error>

async fn cleanup_idle_instances(&self) → Result<(), Error>
```

**Instance Lifecycle**:
1. Query registry for agent config
2. Check instance pool
3. If no available instance and count < max: create new
4. Send request to instance
5. Get response
6. Release instance back to pool
7. Periodic cleanup of idle instances

**Acceptance Criteria**:
- [ ] Instances created on demand
- [ ] Max instances limit enforced
- [ ] Idle instances cleaned up
- [ ] Instance state tracked correctly
- [ ] Error handling for unavailable agents
- [ ] Unit tests for broker (18+ tests)

**References**:
- Agent invocation flow in proposal.md
- Instance pooling patterns

---

### BEAD-AGENTS-003: OpenCode HTTP Agent Client

**Title**: Implement HTTP client for OpenCode headless agents  
**Feature**: Agent Integration  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0  
**Effort**: Event-driven  
**Dependencies**: BEAD-AGENTS-001, BEAD-AGENTS-002  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/agents/opencode.rs` - OpenCode HTTP client:

**Capabilities**:
- Connect to remote OpenCode headless agent
- Serialize context to JSON request
- Send POST to agent endpoint
- Handle authentication (API key, OAuth)
- Deserialize agent response
- Handle HTTP errors and retries
- Timeout management

**Functions**:
```rust
pub struct OpenCodeAgent {
    url: String,
    auth: AuthConfig,
    client: reqwest::Client,
    timeout: Duration,
}

pub async fn invoke(
    &self,
    action: &str,
    context: AgentContext,
) → Result<AgentResponse, AgentError>

async fn prepare_request(
    &self,
    action: &str,
    context: &AgentContext,
) → Result<Request, AgentError>

async fn send_with_retries(
    &self,
    request: Request,
) → Result<Response, AgentError>
```

**Request Format**:
```json
{
  "action": "analyze_compliance",
  "context": {
    "project_path": "/path/to/project",
    "compliance_score": 78.5,
    "violations": [...]
  },
  "auth": {
    "type": "api_key",
    "key": "sk-xxxx"
  }
}
```

**Response Format**:
```json
{
  "status": "success",
  "result": {
    "analysis": {...},
    "recommendations": [...]
  },
  "execution_time_ms": 245
}
```

**Acceptance Criteria**:
- [ ] HTTP requests formatted correctly
- [ ] Authentication headers set properly
- [ ] Response deserialization works
- [ ] Timeout enforced
- [ ] Retries work (exponential backoff)
- [ ] Error messages clear
- [ ] Unit tests with mock HTTP (15+ tests)

**References**:
- `reqwest` async HTTP client
- OpenCode API documentation

---

### BEAD-AGENTS-004: Local Subprocess Agent Launcher

**Title**: Implement subprocess launcher for local agents  
**Feature**: Agent Integration  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P1  
**Effort**: Event-driven  
**Dependencies**: BEAD-AGENTS-001, BEAD-AGENTS-002  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/agents/subprocess.rs` - local subprocess launcher:

**Capabilities**:
- Spawn subprocess for local agent binary
- Pass context via stdin or arguments
- Read response from stdout
- Handle subprocess errors
- Timeout and signal handling
- Resource limits

**Functions**:
```rust
pub struct LocalAgent {
    binary_path: PathBuf,
    timeout: Duration,
}

pub async fn invoke(
    &self,
    action: &str,
    context: AgentContext,
) → Result<AgentResponse, AgentError>

async fn spawn_process(&self) → Result<Child, AgentError>

async fn send_context(&self, process: &mut Child, context: &AgentContext) → Result<(), AgentError>

async fn read_response(&mut self, process: &mut Child) → Result<AgentResponse, AgentError>

async fn wait_with_timeout(&self, process: &mut Child) → Result<(), AgentError>
```

**Context Passing** (stdin):
```json
{
  "action": "suggest_next_actions",
  "context": {
    "project_path": "/path",
    "last_action": "scan_completed",
    "current_state": {...}
  }
}
```

**Response Reading** (stdout):
- Read JSON from stdout
- Deserialize to AgentResponse
- Handle partial/invalid JSON gracefully

**Acceptance Criteria**:
- [ ] Process spawning works
- [ ] Context passed via stdin
- [ ] Response read from stdout
- [ ] Timeout enforced via signal
- [ ] Process cleanup on error
- [ ] Resource limits respected
- [ ] Unit tests with mock binaries (12+ tests)

**References**:
- `tokio::process` module
- Signal handling (SIGTERM, SIGKILL)

---

### BEAD-AGENTS-005: Context-Aware Agent Activation

**Title**: Implement automatic agent suggestion based on application state  
**Feature**: Agent Integration  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P1  
**Effort**: Event-driven  
**Dependencies**: BEAD-AGENTS-001, BEAD-AGENTS-002  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/agents/activation.rs` - context-aware activation:

**Activation Logic**:
Monitor application state and suggest agents when conditions met:

| Condition | Suggested Agent | Action |
|-----------|-----------------|--------|
| Compliance score < 60 | Compliance Analyzer | "Analyze compliance issues?" |
| Phase regressed | Workflow Agent | "Help with phase approval?" |
| PACS drift detected | Drift Agent | "Review drift changes?" |
| User idle 5+ min | Background Tasks | "Run scheduled jobs?" |

**Functions**:
```rust
pub struct ActivationEngine {
    rules: Vec<ActivationRule>,
    last_suggestion: HashMap<String, Instant>,
    suggestion_cooldown: Duration,
}

pub async fn check_activation(
    &mut self,
    app_state: &ApplicationState,
) → Vec<AgentSuggestion>

pub fn should_suggest_agent(
    &self,
    agent_name: &str,
    rule: &ActivationRule,
    app_state: &ApplicationState,
) → bool

pub fn dismiss_suggestion(&self, agent_name: &str)

pub fn get_suggestion_metrics(&self) → SuggestionMetrics
```

**Suggestion Cooldown**:
- Default: 5 minutes between suggestions for same agent
- Configurable per agent
- Prevents notification spam
- User can customize thresholds

**Metrics Tracking**:
- Suggestions made: count
- Suggestions accepted: count
- Acceptance rate: %
- Avg. time from suggestion to acceptance
- Top suggested agents

**Acceptance Criteria**:
- [ ] Conditions checked periodically
- [ ] Suggestions generated correctly
- [ ] Cooldown prevents spam
- [ ] Metrics tracked accurately
- [ ] User can enable/disable suggestions
- [ ] Unit tests for activation logic (15+ tests)

**References**:
- See proposal.md for triggering conditions

---

### BEAD-AGENTS-006: Agent Framework Tests & Documentation

**Title**: Comprehensive testing and documentation for agent framework  
**Feature**: Agent Integration  
**Epic**: Tray Menu + Agent Integration v1.0  
**Priority**: P0  
**Effort**: Event-driven  
**Dependencies**: BEAD-AGENTS-001 through BEAD-AGENTS-005  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create 50+ tests and comprehensive documentation:

**Test Suite** (50+ tests):
- Registry loading (8 tests)
- Broker instance management (12 tests)
- OpenCode HTTP client (15 tests, mock server)
- Subprocess launcher (12 tests, mock binaries)
- Activation engine (10 tests)
- Integration tests (5 tests)

**Test Fixtures**:
- Mock OpenCode server
- Mock subprocess binaries
- Sample agent configurations
- Various application states

**Documentation**:
- Agent Integration Guide: How to add new agents
- Configuration Reference: agents-config.toml
- API Reference: Agent request/response formats
- Troubleshooting: Common agent issues
- Developer Guide: Testing agents locally

**Example Agents** (Stubs):
- `agents-examples/compliance-analyzer` - Stub Python script
- `agents-examples/suggestion-engine` - Stub Rust binary
- With documentation on how to implement real agent

**Acceptance Criteria**:
- [ ] All 50+ tests pass
- [ ] Coverage >70% for agents module
- [ ] Integration guide complete
- [ ] Configuration reference documented
- [ ] Example agents provided
- [ ] Troubleshooting guide comprehensive

**References**:
- OpenCode agent development
- Best practices for agent design

---

## Summary

### Timeline (Event-Driven, No Time Estimates)

**Feature 1 Sequence** (7 issues):
```
BEAD-TRAY-001 → BEAD-TRAY-002 → BEAD-TRAY-003 → BEAD-TRAY-004
                                 ↓
                            BEAD-TRAY-005
                            ↓
                     BEAD-TRAY-006 → BEAD-TRAY-007
```

**Feature 2 Sequence** (6 issues):
```
BEAD-AGENTS-001 → BEAD-AGENTS-002 → BEAD-AGENTS-003
                  ↓
            BEAD-AGENTS-004
            ↓
       BEAD-AGENTS-005 → BEAD-AGENTS-006
```

**Parallelization**:
- Feature 1 and Feature 2 can work in parallel (independent modules)
- Within each feature, follow dependency chain

### Metrics

| Category | Count |
|----------|-------|
| Total Issues | 13 |
| Feature 1 Issues | 7 |
| Feature 2 Issues | 6 |
| Total Tests | 200+ |
| Expected Code Lines | 2,700 (Rust) |
| Documentation Pages | 20+ |

### Effort Allocation

| Component | Cycles |
|-----------|--------|
| Feature 1: Tray Menu | 24 cycles |
| Feature 2: Agent Framework | 30 cycles |
| **Total** | **54 cycles** |

---

**Document Status**: Ready for Implementation  
**Next Step**: Stakeholder approval of proposal.md → Create these issues in bd → Begin Feature 1 Sprint  
**Last Updated**: October 27, 2025

