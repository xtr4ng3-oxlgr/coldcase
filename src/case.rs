use crate::models::CaseMeta;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

pub fn create_case(case_dir: &Path, title: Option<String>) -> Result<()> {
    if case_dir.exists() {
        return Err(anyhow!("case directory already exists: {}", case_dir.display()));
    }

    crate::util::ensure_dir(case_dir)?;
    crate::util::ensure_dir(&crate::util::reports_dir(case_dir))?;
    crate::util::ensure_dir(&crate::util::evidence_dir(case_dir))?;
    crate::util::ensure_dir(&crate::util::export_dir(case_dir))?;

    let conn = crate::db::open(case_dir)?;

    let name = case_dir.file_name()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or_else(|| "coldcase".to_string());

    let meta = CaseMeta {
        name: name.clone(),
        title: title.unwrap_or_else(|| name.clone()),
        created_at: crate::util::now_iso(),
        tool: crate::APP.to_string(),
        version: crate::VERSION.to_string(),
        author: crate::AUTHOR.to_string(),
    };

    crate::db::save_meta(&conn, &meta)?;

    fs::write(case_dir.join("CASE.md"), format!(
        "# {}\n\nCreated: {}\nTool: COLDCASE {}\nAuthor: xtr4ng3\n\nThis folder is a local forensic triage workspace.\n",
        meta.title, meta.created_at, meta.version
    ))?;

    println!("COLDCASE case created: {}", case_dir.display());
    println!("database: {}", crate::util::case_db_path(case_dir).display());
    Ok(())
}

pub fn status(case_dir: &Path) -> Result<()> {
    let conn = crate::db::open(case_dir)?;
    let meta = crate::db::read_meta(&conn)?;
    let artifacts = crate::db::read_artifacts(&conn)?;
    let findings = crate::db::read_findings(&conn)?;
    let snapshots = crate::db::read_snapshots(&conn)?;
    let timeline = crate::db::read_timeline(&conn)?;

    println!("COLDCASE STATUS");
    println!("case      : {}", meta.name);
    println!("title     : {}", meta.title);
    println!("created   : {}", meta.created_at);
    println!("version   : {}", meta.version);
    println!("artifacts : {}", artifacts.len());
    println!("findings  : {}", findings.len());
    println!("snapshots : {}", snapshots.len());
    println!("timeline  : {}", timeline.len());
    println!("reports   : {}", crate::util::reports_dir(case_dir).display());

    Ok(())
}
