use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseMeta {
    pub name: String,
    pub title: String,
    pub created_at: String,
    pub tool: String,
    pub version: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub severity: String,
    pub category: String,
    pub title: String,
    pub detail: String,
    pub recommendation: String,
    pub artifact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileArtifact {
    pub path: String,
    pub size: u64,
    pub extension: String,
    pub modified: String,
    pub created: String,
    pub accessed: String,
    pub sha256: String,
    pub entropy: f64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotItem {
    pub category: String,
    pub name: String,
    pub value: String,
    pub collected_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineItem {
    pub timestamp: String,
    pub event_type: String,
    pub source: String,
    pub detail: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub case_name: String,
    pub generated_at: String,
    pub score: u32,
    pub verdict: String,
    pub artifact_count: usize,
    pub finding_count: usize,
    pub snapshot_count: usize,
}
