use crate::models::{Finding, ReportSummary, TimelineItem};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate_timeline(case_dir: &Path) -> Result<()> {
    let conn = crate::db::open(case_dir)?;
    let artifacts = crate::db::read_artifacts(&conn)?;

    for artifact in artifacts {
        if !artifact.modified.is_empty() {
            crate::db::insert_timeline(&conn, &TimelineItem {
                timestamp: artifact.modified.clone(),
                event_type: "file_modified".to_string(),
                source: artifact.path.clone(),
                detail: format!("{} bytes sha256={}", artifact.size, artifact.sha256),
                severity: severity_from_tags(&artifact.tags),
            })?;
        }
    }

    println!("timeline rebuilt");
    Ok(())
}

pub fn generate_reports(case_dir: &Path) -> Result<()> {
    crate::util::ensure_dir(&crate::util::reports_dir(case_dir))?;

    let conn = crate::db::open(case_dir)?;
    let meta = crate::db::read_meta(&conn)?;
    let artifacts = crate::db::read_artifacts(&conn)?;
    let findings = crate::db::read_findings(&conn)?;
    let snapshots = crate::db::read_snapshots(&conn)?;
    let timeline = crate::db::read_timeline(&conn)?;

    let score = score_findings(&findings);
    let summary = ReportSummary {
        case_name: meta.name.clone(),
        generated_at: crate::util::now_iso(),
        score,
        verdict: verdict(score),
        artifact_count: artifacts.len(),
        finding_count: findings.len(),
        snapshot_count: snapshots.len(),
    };

    let stamp = crate::util::now_file_stamp();
    let reports = crate::util::reports_dir(case_dir);

    let json_path = reports.join(format!("coldcase_{}.json", stamp));
    let html_path = reports.join(format!("coldcase_{}.html", stamp));
    let sarif_path = reports.join(format!("coldcase_{}.sarif", stamp));

    let json = serde_json::json!({
        "summary": summary,
        "case": meta,
        "artifacts": artifacts,
        "findings": findings,
        "snapshots": snapshots,
        "timeline": timeline,
    });

    fs::write(&json_path, serde_json::to_string_pretty(&json)?)?;
    write_html(&html_path, &json)?;
    write_sarif(&sarif_path, &json)?;

    println!("reports generated:");
    println!("  HTML : {}", html_path.display());
    println!("  JSON : {}", json_path.display());
    println!("  SARIF: {}", sarif_path.display());

    Ok(())
}

fn severity_from_tags(tags: &[String]) -> String {
    if tags.iter().any(|t| t == "script") {
        "high".to_string()
    } else if tags.iter().any(|t| t == "executable" || t == "high-entropy") {
        "medium".to_string()
    } else {
        "info".to_string()
    }
}

fn score_findings(findings: &[Finding]) -> u32 {
    let mut score = 0u32;
    for f in findings {
        let s = match f.severity.as_str() {
            "critical" => 100,
            "high" => 70,
            "medium" => 40,
            "low" => 15,
            _ => 0,
        };
        score = score.max(s);
    }

    let high_count = findings.iter().filter(|f| f.severity == "high").count();
    let medium_count = findings.iter().filter(|f| f.severity == "medium").count();

    if high_count >= 5 {
        score = score.max(85);
    } else if medium_count >= 10 {
        score = score.max(60);
    }

    score.min(100)
}

fn verdict(score: u32) -> String {
    if score >= 90 {
        "critical".to_string()
    } else if score >= 70 {
        "high".to_string()
    } else if score >= 40 {
        "medium".to_string()
    } else if score > 0 {
        "low".to_string()
    } else {
        "clean".to_string()
    }
}

fn write_html(path: &Path, report: &serde_json::Value) -> Result<()> {
    let summary = &report["summary"];
    let findings = report["findings"].as_array().cloned().unwrap_or_default();
    let artifacts = report["artifacts"].as_array().cloned().unwrap_or_default();
    let snapshots = report["snapshots"].as_array().cloned().unwrap_or_default();
    let timeline = report["timeline"].as_array().cloned().unwrap_or_default();

    let finding_rows = findings.iter().take(200).map(|f| {
        format!(
            "<tr><td>{}</td><td>{}</td><td><b>{}</b><br>{}</td><td><code>{}</code></td><td>{}</td></tr>",
            escv(&f["severity"]),
            escv(&f["category"]),
            escv(&f["title"]),
            escv(&f["detail"]),
            escv(&f["artifact"]),
            escv(&f["recommendation"])
        )
    }).collect::<Vec<_>>().join("\n");

    let artifact_rows = artifacts.iter().rev().take(200).map(|a| {
        format!(
            "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{:.2}</td><td><code>{}</code></td></tr>",
            escv(&a["path"]),
            escv(&a["extension"]),
            a["size"].as_u64().unwrap_or(0),
            a["entropy"].as_f64().unwrap_or(0.0),
            escv(&a["sha256"])
        )
    }).collect::<Vec<_>>().join("\n");

    let timeline_rows = timeline.iter().take(250).map(|t| {
        format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td><code>{}</code></td><td>{}</td></tr>",
            escv(&t["timestamp"]),
            escv(&t["severity"]),
            escv(&t["event_type"]),
            escv(&t["source"]),
            escv(&t["detail"])
        )
    }).collect::<Vec<_>>().join("\n");

    let snapshot_rows = snapshots.iter().take(100).map(|s| {
        format!(
            "<tr><td>{}</td><td>{}</td><td><pre>{}</pre></td></tr>",
            escv(&s["category"]),
            escv(&s["name"]),
            escv(&s["value"])
        )
    }).collect::<Vec<_>>().join("\n");

    let score = summary["score"].as_u64().unwrap_or(0);
    let color = if score >= 90 { "#ff304f" } else if score >= 70 { "#ff5a36" } else if score >= 40 { "#ffd166" } else { "#7ef9ff" };

    let html = format!(r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>COLDCASE Report</title>
<style>
body{{background:#05070b;color:#e8f6ff;font-family:Consolas,Segoe UI,Arial;padding:30px}}
h1,h2{{color:#ff304f}}
.card{{background:#0b1018;border:1px solid #202c3f;border-radius:16px;padding:18px;margin:18px 0;box-shadow:0 0 24px rgba(255,48,79,.08)}}
table{{width:100%;border-collapse:collapse;margin-top:12px}}
td,th{{border-bottom:1px solid #1a2130;padding:9px;text-align:left;vertical-align:top}}
th{{color:#7ef9ff}}
code{{color:#d6f7ff}}
pre{{white-space:pre-wrap;max-height:260px;overflow:auto;color:#d6f7ff}}
.score{{font-size:58px;font-weight:900;color:{color}}}
.small{{color:#9fb1c7}}
</style>
</head>
<body>
<h1>COLDCASE</h1>
<p class="small">Local Forensic Triage Workbench · xtr4ng3 · {generated}</p>

<div class="card">
<h2>Verdict</h2>
<div class="score">{score}/100</div>
<p><b>{verdict}</b></p>
<p>Artifacts: {artifacts_count} · Findings: {findings_count} · Snapshots: {snapshots_count}</p>
</div>

<div class="card">
<h2>Findings</h2>
<table><tr><th>Severity</th><th>Category</th><th>Finding</th><th>Artifact</th><th>Recommendation</th></tr>
{finding_rows}
</table>
</div>

<div class="card">
<h2>Artifacts</h2>
<table><tr><th>Path</th><th>Ext</th><th>Size</th><th>Entropy</th><th>SHA-256</th></tr>
{artifact_rows}
</table>
</div>

<div class="card">
<h2>Timeline</h2>
<table><tr><th>Timestamp</th><th>Severity</th><th>Type</th><th>Source</th><th>Detail</th></tr>
{timeline_rows}
</table>
</div>

<div class="card">
<h2>Snapshot</h2>
<table><tr><th>Category</th><th>Name</th><th>Value</th></tr>
{snapshot_rows}
</table>
</div>

<p class="small">COLDCASE is a defensive local triage tool. It does not delete, clean, exploit or upload evidence.</p>
</body>
</html>"#,
        color=color,
        generated=escv(&summary["generated_at"]),
        score=score,
        verdict=escv(&summary["verdict"]).to_uppercase(),
        artifacts_count=summary["artifact_count"].as_u64().unwrap_or(0),
        findings_count=summary["finding_count"].as_u64().unwrap_or(0),
        snapshots_count=summary["snapshot_count"].as_u64().unwrap_or(0),
        finding_rows=finding_rows,
        artifact_rows=artifact_rows,
        timeline_rows=timeline_rows,
        snapshot_rows=snapshot_rows,
    );

    fs::write(path, html)?;
    Ok(())
}

fn write_sarif(path: &Path, report: &serde_json::Value) -> Result<()> {
    let findings = report["findings"].as_array().cloned().unwrap_or_default();
    let mut results = Vec::new();

    for f in findings {
        let sev = f["severity"].as_str().unwrap_or("note");
        let level = match sev {
            "critical" | "high" => "error",
            "medium" => "warning",
            _ => "note",
        };

        results.push(serde_json::json!({
            "ruleId": f["category"].as_str().unwrap_or("coldcase"),
            "level": level,
            "message": {
                "text": format!(
                    "{} - {}",
                    f["title"].as_str().unwrap_or("finding"),
                    f["detail"].as_str().unwrap_or("")
                )
            },
            "locations": [{
                "physicalLocation": {
                    "artifactLocation": {
                        "uri": f["artifact"].as_str().unwrap_or("")
                    }
                }
            }]
        }));
    }

    let sarif = serde_json::json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "COLDCASE",
                    "version": crate::VERSION,
                    "informationUri": "https://github.com/xtr4ng3/coldcase"
                }
            },
            "results": results
        }]
    });

    fs::write(path, serde_json::to_string_pretty(&sarif)?)?;
    Ok(())
}

fn escv(value: &serde_json::Value) -> String {
    if let Some(s) = value.as_str() {
        crate::util::html_escape(s)
    } else {
        crate::util::html_escape(&value.to_string())
    }
}
