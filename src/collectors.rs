use crate::models::{Finding, SnapshotItem, TimelineItem};
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn collect_snapshot(case_dir: &Path) -> Result<()> {
    let conn = crate::db::open(case_dir)?;
    let collected_at = crate::util::now_iso();

    println!("COLDCASE snapshot started");

    let mut items = Vec::new();

    items.push(SnapshotItem {
        category: "system".to_string(),
        name: "os".to_string(),
        value: std::env::consts::OS.to_string(),
        collected_at: collected_at.clone(),
    });

    items.push(SnapshotItem {
        category: "system".to_string(),
        name: "arch".to_string(),
        value: std::env::consts::ARCH.to_string(),
        collected_at: collected_at.clone(),
    });

    if let Ok(host) = std::env::var("COMPUTERNAME").or_else(|_| std::env::var("HOSTNAME")) {
        items.push(SnapshotItem {
            category: "system".to_string(),
            name: "hostname".to_string(),
            value: host,
            collected_at: collected_at.clone(),
        });
    }

    if cfg!(windows) {
        collect_windows_snapshot(&mut items, &collected_at);
    } else {
        collect_unix_snapshot(&mut items, &collected_at);
    }

    for item in &items {
        crate::db::insert_snapshot(&conn, item)?;
    }

    let timeline = TimelineItem {
        timestamp: collected_at.clone(),
        event_type: "snapshot".to_string(),
        source: "coldcase".to_string(),
        detail: format!("snapshot collected: {} items", items.len()),
        severity: "info".to_string(),
    };
    crate::db::insert_timeline(&conn, &timeline)?;

    for item in &items {
        if item.category == "startup" {
            let lower = item.value.to_lowercase();
            if lower.contains("powershell") || lower.contains("wscript") || lower.contains("mshta") || lower.contains("\\temp\\") || lower.contains("\\appdata\\") {
                crate::db::insert_finding(&conn, &Finding {
                    severity: "medium".to_string(),
                    category: "startup".to_string(),
                    title: "Startup entry requires review".to_string(),
                    detail: format!("{} -> {}", item.name, item.value),
                    recommendation: "Verify publisher, path and reason for startup execution.".to_string(),
                    artifact: item.value.clone(),
                })?;
            }
        }
    }

    println!("snapshot completed: {} items", items.len());
    Ok(())
}

fn collect_windows_snapshot(items: &mut Vec<SnapshotItem>, collected_at: &str) {
    for (category, name, command) in [
        ("system", "systeminfo", vec!["cmd", "/C", "systeminfo"]),
        ("startup", "wmic_startup", vec!["cmd", "/C", "wmic startup get Caption,Command,Location /format:csv"]),
        ("process", "tasklist", vec!["cmd", "/C", "tasklist /fo csv"]),
        ("network", "netstat", vec!["cmd", "/C", "netstat -ano"]),
        ("events", "system_errors", vec!["cmd", "/C", "wevtutil qe System /q:*[System[(Level=1 or Level=2)]] /c:20 /rd:true /f:text"]),
    ] {
        let value = run_capture(&command, 80_000);
        if !value.trim().is_empty() {
            items.push(SnapshotItem {
                category: category.to_string(),
                name: name.to_string(),
                value,
                collected_at: collected_at.to_string(),
            });
        }
    }
}

fn collect_unix_snapshot(items: &mut Vec<SnapshotItem>, collected_at: &str) {
    for (category, name, command) in [
        ("system", "uname", vec!["uname", "-a"]),
        ("process", "ps", vec!["ps", "aux"]),
        ("network", "netstat", vec!["sh", "-c", "netstat -an 2>/dev/null || ss -tunap 2>/dev/null"]),
    ] {
        let value = run_capture(&command, 80_000);
        if !value.trim().is_empty() {
            items.push(SnapshotItem {
                category: category.to_string(),
                name: name.to_string(),
                value,
                collected_at: collected_at.to_string(),
            });
        }
    }
}

fn run_capture(args: &[&str], max_chars: usize) -> String {
    if args.is_empty() {
        return String::new();
    }

    let output = Command::new(args[0]).args(&args[1..]).output();
    match output {
        Ok(out) => {
            let mut text = String::new();
            text.push_str(&String::from_utf8_lossy(&out.stdout));
            text.push_str(&String::from_utf8_lossy(&out.stderr));
            if text.len() > max_chars {
                text.truncate(max_chars);
            }
            text
        }
        Err(_) => String::new(),
    }
}
