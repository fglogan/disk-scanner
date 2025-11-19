//! Scheduled scan functionality with cron-like expressions.
//!
//! This module provides the ability to schedule automatic scans at regular intervals
//! using cron expressions or simple interval-based scheduling.

use crate::error::{ScannerError, ScannerResult};
use chrono::{DateTime, Local, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, interval};
use uuid::Uuid;

/// Schedule configuration for a scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub id: String,
    pub name: String,
    pub scan_path: String,
    pub scan_types: Vec<ScanType>,
    pub schedule: ScheduleType,
    pub enabled: bool,
    pub last_run: Option<DateTime<Local>>,
    pub next_run: Option<DateTime<Local>>,
    pub notification_enabled: bool,
}

/// Types of scans that can be scheduled
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanType {
    LargeFiles,
    Duplicates,
    JunkFiles,
    DevCaches,
    GitRepos,
    NodeModules,
}

/// Schedule type configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleType {
    /// Run at specific intervals
    Interval { minutes: u32 },
    /// Run daily at specific time
    Daily { hour: u8, minute: u8 },
    /// Run weekly on specific day and time
    Weekly { day: u8, hour: u8, minute: u8 },
    /// Run monthly on specific day and time
    Monthly { day: u8, hour: u8, minute: u8 },
    /// Custom cron expression
    Cron { expression: String },
}

impl ScheduleType {
    /// Calculate the next run time based on the schedule
    pub fn next_run_time(&self, from: DateTime<Local>) -> DateTime<Local> {
        match self {
            ScheduleType::Interval { minutes } => {
                from + Duration::minutes(*minutes as i64)
            }
            ScheduleType::Daily { hour, minute } => {
                let mut next = from.date_naive().and_hms_opt(*hour as u32, *minute as u32, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap();
                if next <= from {
                    next = next + Duration::days(1);
                }
                next
            }
            ScheduleType::Weekly { day, hour, minute } => {
                // TODO: Implement weekly scheduling
                from + Duration::weeks(1)
            }
            ScheduleType::Monthly { day, hour, minute } => {
                // TODO: Implement monthly scheduling
                from + Duration::days(30)
            }
            ScheduleType::Cron { expression } => {
                // TODO: Implement cron expression parsing
                from + Duration::hours(1)
            }
        }
    }
}

/// Result of a scheduled scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledScanResult {
    pub schedule_id: String,
    pub started_at: DateTime<Local>,
    pub completed_at: DateTime<Local>,
    pub success: bool,
    pub error: Option<String>,
    pub findings_summary: HashMap<String, usize>,
}

/// Global scheduler instance
static SCHEDULER: Mutex<Option<ScanScheduler>> = Mutex::new(None);

/// Manages scheduled scans
pub struct ScanScheduler {
    config_file: PathBuf,
    schedules: HashMap<String, ScheduleConfig>,
    active_tasks: Arc<Mutex<HashMap<String, bool>>>,
}

impl ScanScheduler {
    /// Initialize the scheduler
    pub fn init() -> ScannerResult<()> {
        let config_file = dirs::config_dir()
            .ok_or_else(|| ScannerError::FileAccessSimple("Could not determine config directory".to_string()))?
            .join("disk-bloat-scanner")
            .join("schedules.json");
        
        // Ensure directory exists
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to create config directory: {}", e)))?;
        }
        
        // Load existing schedules
        let schedules = if config_file.exists() {
            let content = fs::read_to_string(&config_file)
                .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to read schedules: {}", e)))?;
            serde_json::from_str(&content).unwrap_or_else(|_| HashMap::new())
        } else {
            HashMap::new()
        };
        
        let mut scheduler = SCHEDULER.lock().unwrap();
        *scheduler = Some(ScanScheduler {
            config_file,
            schedules,
            active_tasks: Arc::new(Mutex::new(HashMap::new())),
        });
        
        Ok(())
    }
    
    /// Create a new schedule
    pub fn create_schedule(mut config: ScheduleConfig) -> ScannerResult<String> {
        let mut scheduler = SCHEDULER.lock().unwrap();
        let scheduler = scheduler.as_mut()
            .ok_or_else(|| ScannerError::DatabaseSimple("Scheduler not initialized".to_string()))?;
        
        // Generate ID if not provided
        if config.id.is_empty() {
            config.id = Uuid::new_v4().to_string();
        }
        
        // Calculate next run time
        config.next_run = Some(config.schedule.next_run_time(Local::now()));
        
        // Save schedule
        scheduler.schedules.insert(config.id.clone(), config.clone());
        scheduler.save()?;
        
        // Start the schedule if enabled
        if config.enabled {
            Self::start_schedule_task(&config.id)?;
        }
        
        Ok(config.id)
    }
    
    /// Update an existing schedule
    pub fn update_schedule(id: &str, config: ScheduleConfig) -> ScannerResult<()> {
        let mut scheduler = SCHEDULER.lock().unwrap();
        let scheduler = scheduler.as_mut()
            .ok_or_else(|| ScannerError::DatabaseSimple("Scheduler not initialized".to_string()))?;
        
        // Stop existing task if running
        Self::stop_schedule_task(id)?;
        
        // Update schedule
        scheduler.schedules.insert(id.to_string(), config.clone());
        scheduler.save()?;
        
        // Restart if enabled
        if config.enabled {
            Self::start_schedule_task(id)?;
        }
        
        Ok(())
    }
    
    /// Delete a schedule
    pub fn delete_schedule(id: &str) -> ScannerResult<()> {
        let mut scheduler = SCHEDULER.lock().unwrap();
        let scheduler = scheduler.as_mut()
            .ok_or_else(|| ScannerError::DatabaseSimple("Scheduler not initialized".to_string()))?;
        
        // Stop task if running
        Self::stop_schedule_task(id)?;
        
        // Remove schedule
        scheduler.schedules.remove(id);
        scheduler.save()?;
        
        Ok(())
    }
    
    /// Get all schedules
    pub fn get_schedules() -> ScannerResult<Vec<ScheduleConfig>> {
        let scheduler = SCHEDULER.lock().unwrap();
        let scheduler = scheduler.as_ref()
            .ok_or_else(|| ScannerError::DatabaseSimple("Scheduler not initialized".to_string()))?;
        
        Ok(scheduler.schedules.values().cloned().collect())
    }
    
    /// Get a specific schedule
    pub fn get_schedule(id: &str) -> ScannerResult<ScheduleConfig> {
        let scheduler = SCHEDULER.lock().unwrap();
        let scheduler = scheduler.as_ref()
            .ok_or_else(|| ScannerError::DatabaseSimple("Scheduler not initialized".to_string()))?;
        
        scheduler.schedules.get(id)
            .cloned()
            .ok_or_else(|| ScannerError::DatabaseSimple(format!("Schedule {} not found", id)))
    }
    
    /// Enable or disable a schedule
    pub fn toggle_schedule(id: &str, enabled: bool) -> ScannerResult<()> {
        let mut scheduler = SCHEDULER.lock().unwrap();
        let scheduler = scheduler.as_mut()
            .ok_or_else(|| ScannerError::DatabaseSimple("Scheduler not initialized".to_string()))?;
        
        if let Some(schedule) = scheduler.schedules.get_mut(id) {
            schedule.enabled = enabled;
            scheduler.save()?;
            
            if enabled {
                Self::start_schedule_task(id)?;
            } else {
                Self::stop_schedule_task(id)?;
            }
        }
        
        Ok(())
    }
    
    /// Start all enabled schedules
    pub fn start_all_schedules() -> ScannerResult<()> {
        let scheduler = SCHEDULER.lock().unwrap();
        let scheduler = scheduler.as_ref()
            .ok_or_else(|| ScannerError::DatabaseSimple("Scheduler not initialized".to_string()))?;
        
        let enabled_schedules: Vec<String> = scheduler.schedules
            .iter()
            .filter(|(_, config)| config.enabled)
            .map(|(id, _)| id.clone())
            .collect();
        
        drop(scheduler); // Release lock before starting tasks
        
        for id in enabled_schedules {
            Self::start_schedule_task(&id)?;
        }
        
        Ok(())
    }
    
    /// Start a schedule task
    fn start_schedule_task(id: &str) -> ScannerResult<()> {
        let schedule = Self::get_schedule(id)?;
        
        let scheduler = SCHEDULER.lock().unwrap();
        let scheduler = scheduler.as_ref()
            .ok_or_else(|| ScannerError::DatabaseSimple("Scheduler not initialized".to_string()))?;
        
        let active_tasks = Arc::clone(&scheduler.active_tasks);
        drop(scheduler);
        
        // Mark task as active
        {
            let mut tasks = active_tasks.lock().unwrap();
            tasks.insert(id.to_string(), true);
        }
        
        // Spawn async task
        let schedule_id = id.to_string();
        tokio::spawn(async move {
            Self::run_schedule_loop(schedule_id, schedule, active_tasks).await;
        });
        
        Ok(())
    }
    
    /// Stop a schedule task
    fn stop_schedule_task(id: &str) -> ScannerResult<()> {
        let scheduler = SCHEDULER.lock().unwrap();
        let scheduler = scheduler.as_ref()
            .ok_or_else(|| ScannerError::DatabaseSimple("Scheduler not initialized".to_string()))?;
        
        let mut tasks = scheduler.active_tasks.lock().unwrap();
        tasks.insert(id.to_string(), false);
        
        Ok(())
    }
    
    /// Run the schedule loop
    async fn run_schedule_loop(
        schedule_id: String,
        mut schedule: ScheduleConfig,
        active_tasks: Arc<Mutex<HashMap<String, bool>>>,
    ) {
        loop {
            // Check if task should continue
            {
                let tasks = active_tasks.lock().unwrap();
                if !tasks.get(&schedule_id).copied().unwrap_or(false) {
                    break;
                }
            }
            
            // Calculate time until next run
            let now = Local::now();
            let next_run = schedule.next_run.unwrap_or_else(|| {
                schedule.schedule.next_run_time(now)
            });
            
            if next_run > now {
                let duration = (next_run - now).to_std().unwrap_or_default();
                sleep(duration).await;
            }
            
            // Check again if task should continue
            {
                let tasks = active_tasks.lock().unwrap();
                if !tasks.get(&schedule_id).copied().unwrap_or(false) {
                    break;
                }
            }
            
            // Run the scan
            log::info!("Running scheduled scan: {}", schedule.name);
            let result = Self::execute_scheduled_scan(&schedule).await;
            
            // Update schedule
            schedule.last_run = Some(Local::now());
            schedule.next_run = Some(schedule.schedule.next_run_time(Local::now()));
            
            // Save updated schedule
            if let Err(e) = Self::update_schedule(&schedule_id, schedule.clone()) {
                log::error!("Failed to update schedule: {}", e);
            }
            
            // Send notification if enabled
            if schedule.notification_enabled {
                Self::send_notification(&schedule, &result).await;
            }
        }
    }
    
    /// Execute a scheduled scan
    async fn execute_scheduled_scan(schedule: &ScheduleConfig) -> ScheduledScanResult {
        let started_at = Local::now();
        let mut findings_summary = HashMap::new();
        let mut error = None;
        
        // TODO: Actually execute the scans based on schedule.scan_types
        // For now, we'll simulate the scan
        for scan_type in &schedule.scan_types {
            match scan_type {
                ScanType::LargeFiles => {
                    findings_summary.insert("large_files".to_string(), 42);
                }
                ScanType::Duplicates => {
                    findings_summary.insert("duplicates".to_string(), 15);
                }
                ScanType::JunkFiles => {
                    findings_summary.insert("junk_files".to_string(), 123);
                }
                _ => {}
            }
        }
        
        ScheduledScanResult {
            schedule_id: schedule.id.clone(),
            started_at,
            completed_at: Local::now(),
            success: error.is_none(),
            error,
            findings_summary,
        }
    }
    
    /// Send a notification about scan results
    async fn send_notification(schedule: &ScheduleConfig, result: &ScheduledScanResult) {
        // TODO: Implement actual notification using Tauri's notification API
        log::info!(
            "Scheduled scan '{}' completed: {} findings",
            schedule.name,
            result.findings_summary.values().sum::<usize>()
        );
    }
    
    /// Save schedules to disk
    fn save(&self) -> ScannerResult<()> {
        let json = serde_json::to_string_pretty(&self.schedules)
            .map_err(|e| ScannerError::Other(format!("Failed to serialize schedules: {}", e)))?;
        
        fs::write(&self.config_file, json)
            .map_err(|e| ScannerError::FileAccessSimple(format!("Failed to write schedules: {}", e)))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scheduler_init() {
        assert!(ScanScheduler::init().is_ok());
    }
    
    #[test]
    fn test_schedule_creation() {
        ScanScheduler::init().unwrap();
        
        let config = ScheduleConfig {
            id: String::new(),
            name: "Test Schedule".to_string(),
            scan_path: "/test".to_string(),
            scan_types: vec![ScanType::LargeFiles],
            schedule: ScheduleType::Interval { minutes: 60 },
            enabled: false,
            last_run: None,
            next_run: None,
            notification_enabled: true,
        };
        
        let id = ScanScheduler::create_schedule(config).unwrap();
        assert!(!id.is_empty());
        
        let retrieved = ScanScheduler::get_schedule(&id).unwrap();
        assert_eq!(retrieved.name, "Test Schedule");
    }
}