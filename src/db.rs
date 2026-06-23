use crate::models::{CaseMeta, FileArtifact, Finding, SnapshotItem, TimelineItem};
use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::Path;

pub fn open(case_dir: &Path) -> Result<Connection> {
    let db_path = crate::util::case_db_path(case_dir);
    let conn = Connection::open(db_path)?;
    migrate(&conn)?;
    Ok(conn)
}

pub fn migrate(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS case_meta (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS artifacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            size INTEGER NOT NULL,
            extension TEXT NOT NULL,
            modified TEXT NOT NULL,
            created TEXT NOT NULL,
            accessed TEXT NOT NULL,
            sha256 TEXT NOT NULL,
            entropy REAL NOT NULL,
            tags TEXT NOT NULL,
            collected_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS findings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            severity TEXT NOT NULL,
            category TEXT NOT NULL,
            title TEXT NOT NULL,
            detail TEXT NOT NULL,
            recommendation TEXT NOT NULL,
            artifact TEXT NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS snapshots (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category TEXT NOT NULL,
            name TEXT NOT NULL,
            value TEXT NOT NULL,
            collected_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS timeline (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            event_type TEXT NOT NULL,
            source TEXT NOT NULL,
            detail TEXT NOT NULL,
            severity TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_artifacts_path ON artifacts(path);
        CREATE INDEX IF NOT EXISTS idx_artifacts_modified ON artifacts(modified);
        CREATE INDEX IF NOT EXISTS idx_findings_severity ON findings(severity);
        CREATE INDEX IF NOT EXISTS idx_timeline_timestamp ON timeline(timestamp);
        "#
    )?;
    Ok(())
}

pub fn save_meta(conn: &Connection, meta: &CaseMeta) -> Result<()> {
    let pairs = [
        ("name", meta.name.as_str()),
        ("title", meta.title.as_str()),
        ("created_at", meta.created_at.as_str()),
        ("tool", meta.tool.as_str()),
        ("version", meta.version.as_str()),
        ("author", meta.author.as_str()),
    ];

    for (k, v) in pairs {
        conn.execute(
            "INSERT OR REPLACE INTO case_meta (key, value) VALUES (?1, ?2)",
            params![k, v],
        )?;
    }

    Ok(())
}

pub fn read_meta(conn: &Connection) -> Result<CaseMeta> {
    let get = |key: &str| -> Result<String> {
        Ok(conn.query_row("SELECT value FROM case_meta WHERE key = ?1", params![key], |row| row.get(0))?)
    };

    Ok(CaseMeta {
        name: get("name")?,
        title: get("title")?,
        created_at: get("created_at")?,
        tool: get("tool")?,
        version: get("version")?,
        author: get("author")?,
    })
}

pub fn insert_artifact(conn: &Connection, artifact: &FileArtifact) -> Result<()> {
    conn.execute(
        "INSERT INTO artifacts (path, size, extension, modified, created, accessed, sha256, entropy, tags, collected_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            artifact.path,
            artifact.size as i64,
            artifact.extension,
            artifact.modified,
            artifact.created,
            artifact.accessed,
            artifact.sha256,
            artifact.entropy,
            serde_json::to_string(&artifact.tags)?,
            crate::util::now_iso()
        ],
    )?;
    Ok(())
}

pub fn insert_finding(conn: &Connection, finding: &Finding) -> Result<()> {
    conn.execute(
        "INSERT INTO findings (severity, category, title, detail, recommendation, artifact, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            finding.severity,
            finding.category,
            finding.title,
            finding.detail,
            finding.recommendation,
            finding.artifact,
            crate::util::now_iso()
        ],
    )?;
    Ok(())
}

pub fn insert_snapshot(conn: &Connection, item: &SnapshotItem) -> Result<()> {
    conn.execute(
        "INSERT INTO snapshots (category, name, value, collected_at)
         VALUES (?1, ?2, ?3, ?4)",
        params![item.category, item.name, item.value, item.collected_at],
    )?;
    Ok(())
}

pub fn insert_timeline(conn: &Connection, item: &TimelineItem) -> Result<()> {
    conn.execute(
        "INSERT INTO timeline (timestamp, event_type, source, detail, severity)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![item.timestamp, item.event_type, item.source, item.detail, item.severity],
    )?;
    Ok(())
}

pub fn read_findings(conn: &Connection) -> Result<Vec<Finding>> {
    let mut stmt = conn.prepare("SELECT severity, category, title, detail, recommendation, artifact FROM findings ORDER BY id ASC")?;
    let rows = stmt.query_map([], |row| {
        Ok(Finding {
            severity: row.get(0)?,
            category: row.get(1)?,
            title: row.get(2)?,
            detail: row.get(3)?,
            recommendation: row.get(4)?,
            artifact: row.get(5)?,
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn read_artifacts(conn: &Connection) -> Result<Vec<FileArtifact>> {
    let mut stmt = conn.prepare(
        "SELECT path, size, extension, modified, created, accessed, sha256, entropy, tags
         FROM artifacts ORDER BY id ASC"
    )?;
    let rows = stmt.query_map([], |row| {
        let tags_text: String = row.get(8)?;
        let tags: Vec<String> = serde_json::from_str(&tags_text).unwrap_or_default();
        Ok(FileArtifact {
            path: row.get(0)?,
            size: row.get::<_, i64>(1)? as u64,
            extension: row.get(2)?,
            modified: row.get(3)?,
            created: row.get(4)?,
            accessed: row.get(5)?,
            sha256: row.get(6)?,
            entropy: row.get(7)?,
            tags,
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn read_snapshots(conn: &Connection) -> Result<Vec<SnapshotItem>> {
    let mut stmt = conn.prepare("SELECT category, name, value, collected_at FROM snapshots ORDER BY id ASC")?;
    let rows = stmt.query_map([], |row| {
        Ok(SnapshotItem {
            category: row.get(0)?,
            name: row.get(1)?,
            value: row.get(2)?,
            collected_at: row.get(3)?,
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn read_timeline(conn: &Connection) -> Result<Vec<TimelineItem>> {
    let mut stmt = conn.prepare("SELECT timestamp, event_type, source, detail, severity FROM timeline ORDER BY timestamp ASC")?;
    let rows = stmt.query_map([], |row| {
        Ok(TimelineItem {
            timestamp: row.get(0)?,
            event_type: row.get(1)?,
            source: row.get(2)?,
            detail: row.get(3)?,
            severity: row.get(4)?,
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
