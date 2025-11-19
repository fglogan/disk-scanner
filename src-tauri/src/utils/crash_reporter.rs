//! Crash reporting system with privacy-respecting implementation (BEAD-029)
//! 
//! Provides crash tracking and reporting functionality with opt-in by default
//! and full user control over what data is shared.

use crate::error::DiskBlotResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::panic;

/// Crash report information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    /// Unique crash ID
    pub id: String,
    /// Timestamp of the crash
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Application version
    pub app_version: String,
    /// Operating system info
    pub os_info: OsInfo,
    /// Error message
    pub error_message: String,
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
    /// Context information
    pub context: CrashContext,
}

/// Operating system information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsInfo {
    /// OS name (e.g., "Windows", "macOS", "Linux")
    pub name: String,
    /// OS version
    pub version: String,
    /// Architecture (e.g., "x86_64", "aarch64")
    pub arch: String,
}

/// Crash context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashContext {
    /// Current operation when crash occurred
    pub operation: Option<String>,
    /// Memory usage in MB
    pub memory_usage_mb: Option<u64>,
    /// Number of files being processed
    pub files_processing: Option<usize>,
    /// Active feature flags
    pub feature_flags: Vec<String>,
}

/// Privacy settings for crash reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Whether crash reporting is enabled
    pub enabled: bool,
    /// Include stack traces
    pub include_stack_traces: bool,
    /// Include system information
    pub include_system_info: bool,
    /// Include operation context
    pub include_context: bool,
    /// Anonymize file paths
    pub anonymize_paths: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            enabled: false, // Opt-in by default
            include_stack_traces: false,
            include_system_info: true,
            include_context: true,
            anonymize_paths: true,
        }
    }
}

/// Crash reporter for handling application crashes
pub struct CrashReporter {
    settings: Arc<Mutex<PrivacySettings>>,
    app_version: String,
    crash_endpoint: String,
    local_storage_path: String,
}

impl CrashReporter {
    /// Create a new crash reporter
    pub fn new(app_version: String) -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let storage_path = home
            .join(".disk-bloat-scanner")
            .join("crash_reports")
            .to_string_lossy()
            .to_string();

        Self {
            settings: Arc::new(Mutex::new(PrivacySettings::default())),
            app_version,
            crash_endpoint: "https://api.sentry.io/...".to_string(), // Would be real Sentry DSN
            local_storage_path: storage_path,
        }
    }

    /// Initialize crash reporting
    pub async fn initialize(&self) -> DiskBlotResult<()> {
        let settings = self.settings.lock().await;
        
        if !settings.enabled {
            info!("Crash reporting is disabled");
            return Ok(());
        }

        // Set up panic hook
        let reporter = self.clone();
        panic::set_hook(Box::new(move |panic_info| {
            let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };

            let location = panic_info.location()
                .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
                .unwrap_or_else(|| "Unknown location".to_string());

            error!("Panic occurred: {} at {}", message, location);
            
            // In production, would queue crash report for sending
        }));

        info!("Crash reporting initialized");
        Ok(())
    }

    /// Report a crash
    pub async fn report_crash(&self, error: &dyn std::error::Error, context: CrashContext) -> DiskBlotResult<()> {
        let settings = self.settings.lock().await;
        
        if !settings.enabled {
            return Ok(());
        }

        let os_info = self.get_os_info();
        
        let mut report = CrashReport {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            app_version: self.app_version.clone(),
            os_info: os_info.clone(),
            error_message: error.to_string(),
            stack_trace: if settings.include_stack_traces {
                Some(self.get_stack_trace(error))
            } else {
                None
            },
            context: if settings.include_context {
                context
            } else {
                CrashContext {
                    operation: None,
                    memory_usage_mb: None,
                    files_processing: None,
                    feature_flags: vec![],
                }
            },
        };

        if settings.anonymize_paths {
            report = self.anonymize_report(report);
        }

        // Store locally first
        self.store_report_locally(&report).await?;

        // Send if online
        if self.is_online().await {
            self.send_report(&report).await?;
        }

        Ok(())
    }

    /// Get operating system information
    fn get_os_info(&self) -> OsInfo {
        OsInfo {
            name: std::env::consts::OS.to_string(),
            version: "Unknown".to_string(), // Would use sysinfo crate in production
            arch: std::env::consts::ARCH.to_string(),
        }
    }

    /// Get stack trace from error
    fn get_stack_trace(&self, error: &dyn std::error::Error) -> String {
        // In production, would use backtrace crate
        format!("{:?}", error)
    }

    /// Anonymize paths in crash report
    fn anonymize_report(&self, mut report: CrashReport) -> CrashReport {
        // Replace user-specific paths with placeholders
        if let Some(ref mut trace) = report.stack_trace {
            *trace = trace.replace(dirs::home_dir().unwrap_or_default().to_str().unwrap_or(""), "~");
        }
        report.error_message = report.error_message.replace(dirs::home_dir().unwrap_or_default().to_str().unwrap_or(""), "~");
        report
    }

    /// Store crash report locally
    async fn store_report_locally(&self, report: &CrashReport) -> DiskBlotResult<()> {
        let path = std::path::Path::new(&self.local_storage_path);
        std::fs::create_dir_all(path)?;
        
        let file_path = path.join(format!("{}.json", report.id));
        let json = serde_json::to_string_pretty(report)?;
        std::fs::write(file_path, json)?;
        
        Ok(())
    }

    /// Send crash report to server
    async fn send_report(&self, report: &CrashReport) -> DiskBlotResult<()> {
        // In production, would send to Sentry or similar service
        info!("Would send crash report {} to {}", report.id, self.crash_endpoint);
        Ok(())
    }

    /// Check if online
    async fn is_online(&self) -> bool {
        // Simple check - in production would be more sophisticated
        true
    }

    /// Update privacy settings
    pub async fn update_settings(&self, new_settings: PrivacySettings) -> DiskBlotResult<()> {
        let mut settings = self.settings.lock().await;
        *settings = new_settings;
        info!("Crash reporter settings updated");
        Ok(())
    }

    /// Get current settings
    pub async fn get_settings(&self) -> PrivacySettings {
        self.settings.lock().await.clone()
    }

    /// Clear stored crash reports
    pub async fn clear_reports(&self) -> DiskBlotResult<()> {
        if std::path::Path::new(&self.local_storage_path).exists() {
            std::fs::remove_dir_all(&self.local_storage_path)?;
            info!("Cleared all stored crash reports");
        }
        Ok(())
    }
}

impl Clone for CrashReporter {
    fn clone(&self) -> Self {
        Self {
            settings: Arc::clone(&self.settings),
            app_version: self.app_version.clone(),
            crash_endpoint: self.crash_endpoint.clone(),
            local_storage_path: self.local_storage_path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crash_reporter_disabled_by_default() {
        let reporter = CrashReporter::new("1.0.0".to_string());
        let settings = reporter.get_settings().await;
        assert!(!settings.enabled);
    }

    #[tokio::test]
    async fn test_crash_report_creation() {
        let context = CrashContext {
            operation: Some("scanning".to_string()),
            memory_usage_mb: Some(512),
            files_processing: Some(1000),
            feature_flags: vec!["analytics".to_string()],
        };

        let error = std::io::Error::new(std::io::ErrorKind::Other, "Test error");
        
        let reporter = CrashReporter::new("1.0.0".to_string());
        
        // Enable reporting
        let mut settings = PrivacySettings::default();
        settings.enabled = true;
        reporter.update_settings(settings).await.unwrap();
        
        // Should not panic
        reporter.report_crash(&error, context).await.unwrap();
    }

    #[tokio::test]
    async fn test_anonymization() {
        let reporter = CrashReporter::new("1.0.0".to_string());
        
        let report = CrashReport {
            id: "test".to_string(),
            timestamp: chrono::Utc::now(),
            app_version: "1.0.0".to_string(),
            os_info: reporter.get_os_info(),
            error_message: format!("Error at {}/test.txt", dirs::home_dir().unwrap().display()),
            stack_trace: Some(format!("Stack at {}/src/main.rs", dirs::home_dir().unwrap().display())),
            context: CrashContext {
                operation: None,
                memory_usage_mb: None,
                files_processing: None,
                feature_flags: vec![],
            },
        };

        let anonymized = reporter.anonymize_report(report);
        assert!(anonymized.error_message.contains("~/test.txt"));
        assert!(anonymized.stack_trace.unwrap().contains("~/src/main.rs"));
    }
}