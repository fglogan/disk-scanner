//! Privacy-respecting analytics system (BEAD-030)
//! 
//! Provides local analytics by default with opt-in sharing functionality.
//! Tracks feature usage and performance metrics without collecting personal data.

use crate::error::DiskBlotResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use log::{info, debug};

/// Analytics event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    /// Event name
    pub name: String,
    /// Event category
    pub category: EventCategory,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Event properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Session ID (anonymous)
    pub session_id: String,
}

/// Event categories for analytics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventCategory {
    /// Feature usage events
    FeatureUsage,
    /// Performance metrics
    Performance,
    /// Error events (anonymized)
    Error,
    /// User interface interactions
    UI,
    /// Scan operations
    Scan,
    /// Cleanup operations
    Cleanup,
}

/// Analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    /// Whether local analytics is enabled
    pub local_enabled: bool,
    /// Whether to share analytics (opt-in)
    pub sharing_enabled: bool,
    /// Analytics server endpoint
    pub analytics_endpoint: Option<String>,
    /// Maximum events to store locally
    pub max_local_events: usize,
    /// Events to exclude from tracking
    pub excluded_events: Vec<String>,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            local_enabled: true,
            sharing_enabled: false, // Opt-in by default
            analytics_endpoint: None,
            max_local_events: 10000,
            excluded_events: vec![],
        }
    }
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Scan performance metrics
    pub scan_metrics: ScanMetrics,
    /// Cleanup performance metrics
    pub cleanup_metrics: CleanupMetrics,
    /// UI responsiveness metrics
    pub ui_metrics: UiMetrics,
}

/// Scan performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanMetrics {
    /// Average scan time in seconds
    pub avg_scan_time_sec: f64,
    /// Total files scanned
    pub total_files_scanned: u64,
    /// Average files per second
    pub avg_files_per_sec: f64,
    /// Number of scans performed
    pub scan_count: u32,
}

/// Cleanup performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CleanupMetrics {
    /// Total space cleaned in bytes
    pub total_space_cleaned: u64,
    /// Total files deleted
    pub total_files_deleted: u64,
    /// Average cleanup time in seconds
    pub avg_cleanup_time_sec: f64,
    /// Number of cleanup operations
    pub cleanup_count: u32,
}

/// UI responsiveness metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiMetrics {
    /// Average frame time in milliseconds
    pub avg_frame_time_ms: f64,
    /// UI interaction count
    pub interaction_count: u64,
    /// Feature usage counts
    pub feature_usage: HashMap<String, u64>,
}

/// Analytics manager
pub struct AnalyticsManager {
    config: Arc<Mutex<AnalyticsConfig>>,
    events: Arc<Mutex<Vec<AnalyticsEvent>>>,
    metrics: Arc<Mutex<PerformanceMetrics>>,
    session_id: String,
    storage_path: String,
}

impl AnalyticsManager {
    /// Create a new analytics manager
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let storage_path = home
            .join(".disk-bloat-scanner")
            .join("analytics")
            .to_string_lossy()
            .to_string();

        Self {
            config: Arc::new(Mutex::new(AnalyticsConfig::default())),
            events: Arc::new(Mutex::new(Vec::new())),
            metrics: Arc::new(Mutex::new(PerformanceMetrics {
                scan_metrics: ScanMetrics::default(),
                cleanup_metrics: CleanupMetrics::default(),
                ui_metrics: UiMetrics::default(),
            })),
            session_id: uuid::Uuid::new_v4().to_string(),
            storage_path,
        }
    }

    /// Track an event
    pub async fn track_event(&self, name: &str, category: EventCategory, properties: HashMap<String, serde_json::Value>) -> DiskBlotResult<()> {
        let config = self.config.lock().await;
        
        if !config.local_enabled {
            return Ok(());
        }

        if config.excluded_events.contains(&name.to_string()) {
            return Ok(());
        }

        let event = AnalyticsEvent {
            name: name.to_string(),
            category,
            timestamp: chrono::Utc::now(),
            properties,
            session_id: self.session_id.clone(),
        };

        debug!("Tracking event: {} in category {:?}", name, event.category);

        let mut events = self.events.lock().await;
        events.push(event.clone());

        // Trim events if exceeding max
        if events.len() > config.max_local_events {
            let excess = events.len() - config.max_local_events;
            events.drain(0..excess);
        }

        // Update metrics based on event
        drop(events); // Release lock
        self.update_metrics(&event).await?;

        // Share if enabled
        if config.sharing_enabled {
            self.share_event(&event).await?;
        }

        Ok(())
    }

    /// Track feature usage
    pub async fn track_feature_usage(&self, feature_name: &str) -> DiskBlotResult<()> {
        let mut properties = HashMap::new();
        properties.insert("feature".to_string(), serde_json::Value::String(feature_name.to_string()));
        
        self.track_event(&format!("feature_used_{}", feature_name), EventCategory::FeatureUsage, properties).await?;
        
        // Update UI metrics
        let mut metrics = self.metrics.lock().await;
        *metrics.ui_metrics.feature_usage.entry(feature_name.to_string()).or_insert(0) += 1;
        
        Ok(())
    }

    /// Track scan performance
    pub async fn track_scan_performance(&self, duration_sec: f64, files_scanned: u64) -> DiskBlotResult<()> {
        let mut properties = HashMap::new();
        properties.insert("duration_sec".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(duration_sec).unwrap()));
        properties.insert("files_scanned".to_string(), serde_json::Value::Number(files_scanned.into()));
        
        self.track_event("scan_completed", EventCategory::Performance, properties).await?;
        
        // Update scan metrics
        let mut metrics = self.metrics.lock().await;
        let scan_metrics = &mut metrics.scan_metrics;
        scan_metrics.scan_count += 1;
        scan_metrics.total_files_scanned += files_scanned;
        
        // Update averages
        let total_time = scan_metrics.avg_scan_time_sec * (scan_metrics.scan_count - 1) as f64 + duration_sec;
        scan_metrics.avg_scan_time_sec = total_time / scan_metrics.scan_count as f64;
        
        if duration_sec > 0.0 {
            scan_metrics.avg_files_per_sec = files_scanned as f64 / duration_sec;
        }
        
        Ok(())
    }

    /// Track cleanup performance
    pub async fn track_cleanup_performance(&self, duration_sec: f64, files_deleted: u64, space_freed: u64) -> DiskBlotResult<()> {
        let mut properties = HashMap::new();
        properties.insert("duration_sec".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(duration_sec).unwrap()));
        properties.insert("files_deleted".to_string(), serde_json::Value::Number(files_deleted.into()));
        properties.insert("space_freed".to_string(), serde_json::Value::Number(space_freed.into()));
        
        self.track_event("cleanup_completed", EventCategory::Performance, properties).await?;
        
        // Update cleanup metrics
        let mut metrics = self.metrics.lock().await;
        let cleanup_metrics = &mut metrics.cleanup_metrics;
        cleanup_metrics.cleanup_count += 1;
        cleanup_metrics.total_files_deleted += files_deleted;
        cleanup_metrics.total_space_cleaned += space_freed;
        
        // Update average
        let total_time = cleanup_metrics.avg_cleanup_time_sec * (cleanup_metrics.cleanup_count - 1) as f64 + duration_sec;
        cleanup_metrics.avg_cleanup_time_sec = total_time / cleanup_metrics.cleanup_count as f64;
        
        Ok(())
    }

    /// Update metrics based on event
    async fn update_metrics(&self, event: &AnalyticsEvent) -> DiskBlotResult<()> {
        // Metrics are updated in specific tracking methods
        Ok(())
    }

    /// Share event with analytics server
    async fn share_event(&self, event: &AnalyticsEvent) -> DiskBlotResult<()> {
        let config = self.config.lock().await;
        
        if let Some(endpoint) = &config.analytics_endpoint {
            // In production, would send to analytics server
            debug!("Would share event {} to {}", event.name, endpoint);
        }
        
        Ok(())
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.lock().await.clone()
    }

    /// Get feature usage statistics
    pub async fn get_feature_usage(&self) -> HashMap<String, u64> {
        let metrics = self.metrics.lock().await;
        metrics.ui_metrics.feature_usage.clone()
    }

    /// Save analytics data locally
    pub async fn save_to_disk(&self) -> DiskBlotResult<()> {
        std::fs::create_dir_all(&self.storage_path)?;
        
        let events = self.events.lock().await;
        let metrics = self.metrics.lock().await;
        
        // Save events
        let events_path = std::path::Path::new(&self.storage_path).join("events.json");
        let events_json = serde_json::to_string_pretty(&*events)?;
        std::fs::write(events_path, events_json)?;
        
        // Save metrics
        let metrics_path = std::path::Path::new(&self.storage_path).join("metrics.json");
        let metrics_json = serde_json::to_string_pretty(&*metrics)?;
        std::fs::write(metrics_path, metrics_json)?;
        
        info!("Analytics data saved to disk");
        Ok(())
    }

    /// Clear all analytics data
    pub async fn clear_data(&self) -> DiskBlotResult<()> {
        let mut events = self.events.lock().await;
        events.clear();
        
        let mut metrics = self.metrics.lock().await;
        *metrics = PerformanceMetrics {
            scan_metrics: ScanMetrics::default(),
            cleanup_metrics: CleanupMetrics::default(),
            ui_metrics: UiMetrics::default(),
        };
        
        // Remove from disk
        if std::path::Path::new(&self.storage_path).exists() {
            std::fs::remove_dir_all(&self.storage_path)?;
        }
        
        info!("Analytics data cleared");
        Ok(())
    }

    /// Update configuration
    pub async fn update_config(&self, new_config: AnalyticsConfig) -> DiskBlotResult<()> {
        let mut config = self.config.lock().await;
        *config = new_config;
        info!("Analytics configuration updated");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analytics_local_only_by_default() {
        let analytics = AnalyticsManager::new();
        let config = analytics.config.lock().await;
        
        assert!(config.local_enabled);
        assert!(!config.sharing_enabled);
    }

    #[tokio::test]
    async fn test_track_event() {
        let analytics = AnalyticsManager::new();
        
        let mut props = HashMap::new();
        props.insert("test".to_string(), serde_json::Value::String("value".to_string()));
        
        analytics.track_event("test_event", EventCategory::FeatureUsage, props).await.unwrap();
        
        let events = analytics.events.lock().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "test_event");
    }

    #[tokio::test]
    async fn test_feature_usage_tracking() {
        let analytics = AnalyticsManager::new();
        
        analytics.track_feature_usage("scan_duplicates").await.unwrap();
        analytics.track_feature_usage("scan_duplicates").await.unwrap();
        analytics.track_feature_usage("cleanup").await.unwrap();
        
        let usage = analytics.get_feature_usage().await;
        assert_eq!(usage.get("scan_duplicates"), Some(&2));
        assert_eq!(usage.get("cleanup"), Some(&1));
    }

    #[tokio::test]
    async fn test_performance_metrics() {
        let analytics = AnalyticsManager::new();
        
        analytics.track_scan_performance(10.0, 1000).await.unwrap();
        analytics.track_scan_performance(20.0, 2000).await.unwrap();
        
        let metrics = analytics.get_metrics().await;
        assert_eq!(metrics.scan_metrics.scan_count, 2);
        assert_eq!(metrics.scan_metrics.total_files_scanned, 3000);
        assert_eq!(metrics.scan_metrics.avg_scan_time_sec, 15.0);
    }
}