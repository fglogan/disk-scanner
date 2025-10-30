// Database module for disk bloat scanner
// SQLite foundation with OSM-lite migration planning

use rusqlite::{Connection, Result, Row};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Project monitoring database with OSM-lite migration support
pub struct ProjectDatabase {
    conn: Connection,
    osm_migration_ready: bool,
}

/// Project scan result for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectScanResult {
    pub id: Option<i64>,
    pub project_path: String,
    pub scan_timestamp: DateTime<Utc>,
    pub total_size_mb: f64,
    pub bloat_size_mb: f64,
    pub large_files_count: i32,
    pub duplicates_count: i32,
    pub junk_files_count: i32,
    pub git_repo_status: Option<String>,
    pub project_type: Option<String>,
    pub compliance_score: Option<f64>,
}

/// Project monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMonitorConfig {
    pub id: Option<i64>,
    pub project_path: String,
    pub monitor_enabled: bool,
    pub scan_interval_hours: i32,
    pub alert_thresholds: String, // JSON serialized thresholds
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// OSM-lite migration plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSMMigrationPlan {
    pub current_schema_version: String,
    pub target_osm_version: String,
    pub data_export_path: String,
    pub migration_steps: Vec<String>,
    pub estimated_duration_minutes: i32,
}

impl ProjectDatabase {
    /// Create new database connection with OSM-lite compatibility
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Initialize schema with OSM-lite compatibility markers
        conn.execute_batch(
            r#"
            -- Project scan results table
            CREATE TABLE IF NOT EXISTS project_scans (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_path TEXT NOT NULL,
                scan_timestamp TEXT NOT NULL,
                total_size_mb REAL NOT NULL,
                bloat_size_mb REAL NOT NULL,
                large_files_count INTEGER NOT NULL,
                duplicates_count INTEGER NOT NULL,
                junk_files_count INTEGER NOT NULL,
                git_repo_status TEXT,
                project_type TEXT,
                compliance_score REAL,
                -- OSM-lite compatibility metadata
                osm_entity_type TEXT DEFAULT 'project_scan',
                osm_provenance TEXT DEFAULT '{"source": "disk_bloat_scanner", "version": "0.1.1"}',
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            
            -- Project monitoring configuration
            CREATE TABLE IF NOT EXISTS project_monitors (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_path TEXT UNIQUE NOT NULL,
                monitor_enabled INTEGER NOT NULL DEFAULT 1,
                scan_interval_hours INTEGER NOT NULL DEFAULT 24,
                alert_thresholds TEXT NOT NULL DEFAULT '{}',
                -- OSM-lite compatibility metadata
                osm_entity_type TEXT DEFAULT 'project_monitor',
                osm_provenance TEXT DEFAULT '{"source": "disk_bloat_scanner", "version": "0.1.1"}',
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            
            -- Project change history for trend analysis
            CREATE TABLE IF NOT EXISTS project_changes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_path TEXT NOT NULL,
                change_type TEXT NOT NULL, -- 'size_increase', 'new_files', 'compliance_change'
                change_value REAL NOT NULL,
                change_description TEXT,
                detected_at TEXT NOT NULL,
                -- OSM-lite compatibility metadata
                osm_entity_type TEXT DEFAULT 'project_change',
                osm_provenance TEXT DEFAULT '{"source": "disk_bloat_scanner", "version": "0.1.1"}',
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            
            -- Indexes for performance
            CREATE INDEX IF NOT EXISTS idx_project_scans_path_time 
                ON project_scans(project_path, scan_timestamp);
            CREATE INDEX IF NOT EXISTS idx_project_monitors_path 
                ON project_monitors(project_path);
            CREATE INDEX IF NOT EXISTS idx_project_changes_path_time 
                ON project_changes(project_path, detected_at);
            
            -- OSM-lite migration metadata table
            CREATE TABLE IF NOT EXISTS osm_migration_metadata (
                id INTEGER PRIMARY KEY,
                schema_version TEXT NOT NULL DEFAULT '1.0.0',
                osm_compatibility_version TEXT NOT NULL DEFAULT 'pre-osm',
                migration_status TEXT NOT NULL DEFAULT 'sqlite_native',
                last_migration_check TEXT DEFAULT CURRENT_TIMESTAMP,
                migration_plan TEXT -- JSON serialized migration plan
            );
            
            -- Insert initial migration metadata
            INSERT OR IGNORE INTO osm_migration_metadata (id, schema_version, osm_compatibility_version, migration_status)
            VALUES (1, '1.0.0', 'pre-osm', 'sqlite_native');
            "#,
        )?;
        
        Ok(Self {
            conn,
            osm_migration_ready: true,
        })
    }
    
    /// Store project scan result
    pub fn store_scan_result(&self, result: &ProjectScanResult) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            r#"
            INSERT INTO project_scans (
                project_path, scan_timestamp, total_size_mb, bloat_size_mb,
                large_files_count, duplicates_count, junk_files_count,
                git_repo_status, project_type, compliance_score
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
        )?;
        
        stmt.execute((
            &result.project_path,
            result.scan_timestamp.to_rfc3339(),
            result.total_size_mb,
            result.bloat_size_mb,
            result.large_files_count,
            result.duplicates_count,
            result.junk_files_count,
            &result.git_repo_status,
            &result.project_type,
            result.compliance_score,
        ))?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    /// Get project scan history
    pub fn get_project_history(&self, project_path: &str, limit: i32) -> Result<Vec<ProjectScanResult>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, project_path, scan_timestamp, total_size_mb, bloat_size_mb,
                   large_files_count, duplicates_count, junk_files_count,
                   git_repo_status, project_type, compliance_score
            FROM project_scans 
            WHERE project_path = ?1 
            ORDER BY scan_timestamp DESC 
            LIMIT ?2
            "#,
        )?;
        
        let rows = stmt.query_map((project_path, limit), |row| {
            Ok(ProjectScanResult {
                id: Some(row.get(0)?),
                project_path: row.get(1)?,
                scan_timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(2, "timestamp".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Utc),
                total_size_mb: row.get(3)?,
                bloat_size_mb: row.get(4)?,
                large_files_count: row.get(5)?,
                duplicates_count: row.get(6)?,
                junk_files_count: row.get(7)?,
                git_repo_status: row.get(8)?,
                project_type: row.get(9)?,
                compliance_score: row.get(10)?,
            })
        })?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
    
    /// Configure project monitoring
    pub fn configure_monitoring(&self, config: &ProjectMonitorConfig) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            r#"
            INSERT OR REPLACE INTO project_monitors (
                project_path, monitor_enabled, scan_interval_hours, alert_thresholds
            ) VALUES (?1, ?2, ?3, ?4)
            "#,
        )?;
        
        stmt.execute((
            &config.project_path,
            config.monitor_enabled as i32,
            config.scan_interval_hours,
            &config.alert_thresholds,
        ))?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    /// Get monitored projects
    pub fn get_monitored_projects(&self) -> Result<Vec<ProjectMonitorConfig>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, project_path, monitor_enabled, scan_interval_hours, 
                   alert_thresholds, created_at, updated_at
            FROM project_monitors 
            WHERE monitor_enabled = 1
            ORDER BY project_path
            "#,
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok(ProjectMonitorConfig {
                id: Some(row.get(0)?),
                project_path: row.get(1)?,
                monitor_enabled: row.get::<_, i32>(2)? != 0,
                scan_interval_hours: row.get(3)?,
                alert_thresholds: row.get(4)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(5, "created_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(6, "updated_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Utc),
            })
        })?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
    
    /// Prepare OSM-lite migration plan
    pub fn prepare_osm_migration(&self) -> Result<OSMMigrationPlan> {
        // Check current data volume and complexity
        let scan_count: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM project_scans",
            [],
            |row| row.get(0),
        )?;
        
        let monitor_count: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM project_monitors",
            [],
            |row| row.get(0),
        )?;
        
        // Estimate migration complexity
        let estimated_duration = ((scan_count + monitor_count) as f32 * 0.1).ceil() as i32;
        
        let migration_plan = OSMMigrationPlan {
            current_schema_version: "1.0.0".to_string(),
            target_osm_version: "osm-lite-1.0".to_string(),
            data_export_path: "./data/osm_migration_export.json".to_string(),
            migration_steps: vec![
                "Export SQLite data to OSM-compatible JSON format".to_string(),
                "Validate data integrity and schema compatibility".to_string(),
                "Initialize OSM-lite instance with project schema".to_string(),
                "Import data with provenance preservation".to_string(),
                "Verify data consistency and relationships".to_string(),
                "Update application configuration to use OSM-lite".to_string(),
                "Archive SQLite database as backup".to_string(),
            ],
            estimated_duration_minutes: estimated_duration.max(5),
        };
        
        // Store migration plan
        let plan_json = serde_json::to_string(&migration_plan)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        self.conn.execute(
            "UPDATE osm_migration_metadata SET migration_plan = ?1, last_migration_check = CURRENT_TIMESTAMP WHERE id = 1",
            [plan_json],
        )?;
        
        Ok(migration_plan)
    }
    
    /// Export data for OSM-lite migration
    pub fn export_for_osm_migration(&self, export_path: &str) -> Result<usize> {
        // Export all data in OSM-compatible format
        let scans = self.get_all_scan_results()?;
        let monitors = self.get_monitored_projects()?;
        
        let export_data = serde_json::json!({
            "schema_version": "1.0.0",
            "export_timestamp": Utc::now().to_rfc3339(),
            "entities": {
                "project_scans": scans,
                "project_monitors": monitors
            },
            "metadata": {
                "source": "disk_bloat_scanner",
                "version": "0.1.1",
                "osm_compatibility": "pre-osm"
            }
        });
        
        std::fs::write(export_path, export_data.to_string())
            .map_err(|e| rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some(format!("Failed to write export file: {}", e))
            ))?;
        
        Ok(scans.len() + monitors.len())
    }
    
    /// Get all scan results for migration
    fn get_all_scan_results(&self) -> Result<Vec<ProjectScanResult>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, project_path, scan_timestamp, total_size_mb, bloat_size_mb,
                   large_files_count, duplicates_count, junk_files_count,
                   git_repo_status, project_type, compliance_score
            FROM project_scans 
            ORDER BY scan_timestamp DESC
            "#,
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok(ProjectScanResult {
                id: Some(row.get(0)?),
                project_path: row.get(1)?,
                scan_timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(2, "timestamp".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&Utc),
                total_size_mb: row.get(3)?,
                bloat_size_mb: row.get(4)?,
                large_files_count: row.get(5)?,
                duplicates_count: row.get(6)?,
                junk_files_count: row.get(7)?,
                git_repo_status: row.get(8)?,
                project_type: row.get(9)?,
                compliance_score: row.get(10)?,
            })
        })?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_database_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = ProjectDatabase::new(temp_file.path().to_str().unwrap()).unwrap();
        assert!(db.osm_migration_ready);
    }
    
    #[test]
    fn test_scan_result_storage() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = ProjectDatabase::new(temp_file.path().to_str().unwrap()).unwrap();
        
        let result = ProjectScanResult {
            id: None,
            project_path: "/test/project".to_string(),
            scan_timestamp: Utc::now(),
            total_size_mb: 1024.0,
            bloat_size_mb: 256.0,
            large_files_count: 5,
            duplicates_count: 3,
            junk_files_count: 10,
            git_repo_status: Some("clean".to_string()),
            project_type: Some("rust".to_string()),
            compliance_score: Some(95.5),
        };
        
        let id = db.store_scan_result(&result).unwrap();
        assert!(id > 0);
        
        let history = db.get_project_history("/test/project", 10).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].project_path, "/test/project");
    }
    
    #[test]
    fn test_osm_migration_plan() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = ProjectDatabase::new(temp_file.path().to_str().unwrap()).unwrap();
        
        let plan = db.prepare_osm_migration().unwrap();
        assert_eq!(plan.current_schema_version, "1.0.0");
        assert_eq!(plan.target_osm_version, "osm-lite-1.0");
        assert!(!plan.migration_steps.is_empty());
    }
}