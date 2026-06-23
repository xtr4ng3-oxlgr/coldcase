use crate::models::{FileArtifact, TimelineItem};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_target(case_dir: &Path, target: &Path, max_mb: u64) -> Result<()> {
    if !target.exists() || !target.is_dir() {
        return Err(anyhow!("target must be an existing directory: {}", target.display()));
    }

    let conn = crate::db::open(case_dir)?;
    let mut scanned = 0usize;
    let mut findings = 0usize;

    println!("COLDCASE scan started");
    println!("case  : {}", case_dir.display());
    println!("target: {}", target.display());

    for entry in WalkDir::new(target).follow_links(false).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if should_skip(path) {
            continue;
        }

        let artifact = build_artifact(path, max_mb)?;
        crate::db::insert_artifact(&conn, &artifact)?;

        let timeline = TimelineItem {
            timestamp: artifact.modified.clone(),
            event_type: "file_modified".to_string(),
            source: artifact.path.clone(),
            detail: format!("size={} sha256={}", artifact.size, artifact.sha256),
            severity: "info".to_string(),
        };
        crate::db::insert_timeline(&conn, &timeline)?;

        for finding in crate::rules::evaluate_artifact(&artifact) {
            crate::db::insert_finding(&conn, &finding)?;
            findings += 1;
        }

        scanned += 1;
        if scanned % 500 == 0 {
            println!("scanned: {}", scanned);
        }
    }

    println!("scan completed");
    println!("artifacts: {}", scanned);
    println!("findings : {}", findings);
    Ok(())
}

fn should_skip(path: &Path) -> bool {
    let lower = path.to_string_lossy().to_lowercase();
    let blocked = [
        "\\windows\\winsxs\\",
        "\\windows\\servicing\\",
        "\\program files\\windowsapps\\",
        "\\node_modules\\",
        "\\.git\\",
        "\\target\\",
        "\\__pycache__\\",
        "/node_modules/",
        "/.git/",
        "/target/",
        "/__pycache__/",
    ];

    blocked.iter().any(|x| lower.contains(x))
}

fn build_artifact(path: &Path, max_mb: u64) -> Result<FileArtifact> {
    let meta = fs::metadata(path)?;
    let ext = path.extension()
        .map(|x| x.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let normalized = crate::util::normalize_path(path);
    let entropy = crate::hashing::entropy_file(path, 1024 * 1024);
    let tags = crate::rules::tags_for_artifact(&normalized, &ext, entropy);

    Ok(FileArtifact {
        path: normalized,
        size: meta.len(),
        extension: ext,
        modified: crate::util::file_time(path, "modified"),
        created: crate::util::file_time(path, "created"),
        accessed: crate::util::file_time(path, "accessed"),
        sha256: crate::hashing::sha256_file(path, max_mb)?,
        entropy,
        tags,
    })
}
