use crate::models::{FileArtifact, Finding};
use anyhow::Result;
use std::path::Path;

const DEFAULT_RULES: &str = r#"
# COLDCASE default rules
# These are defensive triage labels used by the scanner.
#
# This file is intentionally simple and human-readable.

HIGH_EXTENSIONS = ps1, vbs, js, jse, wsf, hta, bat, cmd, scr, pif, lnk
ARCHIVE_EXTENSIONS = zip, rar, 7z, iso, img
EXECUTABLE_EXTENSIONS = exe, dll, sys, msi
SENSITIVE_PATHS = AppData, Temp, Downloads, Descargas, Public
HIGH_ENTROPY_THRESHOLD = 7.25
"#;

pub fn write_default_rules(output: &Path) -> Result<()> {
    std::fs::write(output, DEFAULT_RULES)?;
    println!("rules written: {}", output.display());
    Ok(())
}

pub fn evaluate_artifact(artifact: &FileArtifact) -> Vec<Finding> {
    let mut findings = Vec::new();
    let path_lower = artifact.path.to_lowercase();
    let ext = artifact.extension.to_lowercase();

    let script_ext = ["ps1", "vbs", "js", "jse", "wsf", "hta", "bat", "cmd"];
    let exe_ext = ["exe", "dll", "sys", "msi", "scr", "pif"];
    let archive_ext = ["zip", "rar", "7z", "iso", "img"];

    if script_ext.contains(&ext.as_str()) {
        findings.push(Finding {
            severity: "high".to_string(),
            category: "script".to_string(),
            title: "Executable script artifact".to_string(),
            detail: format!("Script-like file discovered: {}", artifact.path),
            recommendation: "Review origin, timestamp, content and execution context before trusting it.".to_string(),
            artifact: artifact.path.clone(),
        });
    }

    if exe_ext.contains(&ext.as_str()) {
        findings.push(Finding {
            severity: "medium".to_string(),
            category: "executable".to_string(),
            title: "Executable artifact".to_string(),
            detail: format!("Executable file discovered: {}", artifact.path),
            recommendation: "Verify signature, source, hash reputation and creation time.".to_string(),
            artifact: artifact.path.clone(),
        });
    }

    if archive_ext.contains(&ext.as_str()) {
        findings.push(Finding {
            severity: "low".to_string(),
            category: "archive".to_string(),
            title: "Archive artifact".to_string(),
            detail: format!("Archive file discovered: {}", artifact.path),
            recommendation: "Inspect archive contents before extracting or running files.".to_string(),
            artifact: artifact.path.clone(),
        });
    }

    for sensitive in ["\\appdata\\", "\\temp\\", "\\downloads\\", "\\descargas\\", "\\public\\"] {
        if path_lower.contains(sensitive) {
            findings.push(Finding {
                severity: "medium".to_string(),
                category: "path".to_string(),
                title: "Sensitive location".to_string(),
                detail: format!("Artifact is located in a high-noise or high-risk path: {}", artifact.path),
                recommendation: "Correlate with download time, startup entries and recent activity.".to_string(),
                artifact: artifact.path.clone(),
            });
            break;
        }
    }

    if artifact.entropy >= 7.25 && artifact.size > 4096 {
        findings.push(Finding {
            severity: "medium".to_string(),
            category: "entropy".to_string(),
            title: "High entropy artifact".to_string(),
            detail: format!("Entropy {:.2} detected in {}", artifact.entropy, artifact.path),
            recommendation: "High entropy can indicate compressed, encrypted or packed content. Review context.".to_string(),
            artifact: artifact.path.clone(),
        });
    }

    if path_lower.contains("crack") || path_lower.contains("keygen") || path_lower.contains("activator") {
        findings.push(Finding {
            severity: "high".to_string(),
            category: "naming".to_string(),
            title: "High-risk filename keyword".to_string(),
            detail: format!("Artifact name includes a risky keyword: {}", artifact.path),
            recommendation: "Avoid executing untrusted activators, cracks or keygens.".to_string(),
            artifact: artifact.path.clone(),
        });
    }

    findings
}

pub fn tags_for_artifact(path: &str, ext: &str, entropy: f64) -> Vec<String> {
    let mut tags = Vec::new();
    let lower = path.to_lowercase();

    if ["exe", "dll", "sys", "msi"].contains(&ext) {
        tags.push("executable".to_string());
    }
    if ["ps1", "vbs", "js", "jse", "hta", "bat", "cmd"].contains(&ext) {
        tags.push("script".to_string());
    }
    if ["zip", "rar", "7z", "iso"].contains(&ext) {
        tags.push("archive".to_string());
    }
    if lower.contains("\\appdata\\") || lower.contains("\\temp\\") || lower.contains("\\downloads\\") || lower.contains("\\descargas\\") {
        tags.push("sensitive-path".to_string());
    }
    if entropy >= 7.25 {
        tags.push("high-entropy".to_string());
    }

    tags
}
