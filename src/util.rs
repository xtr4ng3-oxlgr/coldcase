use chrono::{DateTime, Local, Utc};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_iso() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

pub fn now_file_stamp() -> String {
    Local::now().format("%Y%m%d_%H%M%S").to_string()
}

pub fn system_time_to_iso(time: Option<SystemTime>) -> String {
    match time {
        Some(t) => {
            let dt: DateTime<Utc> = t.into();
            dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        }
        None => String::new(),
    }
}

pub fn normalize_path(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .replace('\t', " ")
}

pub fn ensure_dir(path: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn case_db_path(case_dir: &Path) -> PathBuf {
    case_dir.join("coldcase.db")
}

pub fn reports_dir(case_dir: &Path) -> PathBuf {
    case_dir.join("reports")
}

pub fn evidence_dir(case_dir: &Path) -> PathBuf {
    case_dir.join("evidence")
}

pub fn export_dir(case_dir: &Path) -> PathBuf {
    case_dir.join("exports")
}

pub fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn json_escape(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub fn file_time(path: &Path, kind: &str) -> String {
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return String::new(),
    };

    let t = match kind {
        "created" => meta.created().ok(),
        "accessed" => meta.accessed().ok(),
        _ => meta.modified().ok(),
    };

    system_time_to_iso(t)
}

pub fn unix_seconds_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}
