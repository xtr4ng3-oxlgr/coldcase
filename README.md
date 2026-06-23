# COLDCASE

**COLDCASE** is a local forensic triage workbench built in Rust for case snapshots, file timelines, hashes, suspicious artifacts, startup review and defensive reports.

Created by **xtr4ng3**.

---

## Purpose

When a machine behaves strangely, the first problem is usually not removal.  
The first problem is understanding what changed, where it changed and how to document it.

COLDCASE is designed for local defensive triage:

- create a case workspace,
- capture a system snapshot,
- scan folders,
- calculate hashes,
- label suspicious artifacts,
- generate timelines,
- produce HTML, JSON and SARIF reports.

COLDCASE does not clean, delete, exploit or upload evidence.  
It organizes local evidence so a user, technician or analyst can review it.

---

## Features

- Rust CLI,
- SQLite case database,
- case workspaces,
- file artifact collection,
- SHA-256 hashing,
- entropy estimation,
- file timeline generation,
- Windows snapshot collection,
- process/startup/network/event snapshot commands,
- defensive rule engine,
- HTML report,
- JSON report,
- SARIF report,
- static dashboard,
- GitHub Actions workflow,
- documented architecture.

---

## Commands

Create a new case:

```bash
coldcase new cases/my-case --title "Suspicious download review"
```

Capture local system snapshot:

```bash
coldcase snapshot cases/my-case
```

Scan a folder:

```bash
coldcase scan cases/my-case C:\Users\User\Downloads
```

Build timeline:

```bash
coldcase timeline cases/my-case
```

Generate reports:

```bash
coldcase report cases/my-case
```

Show status:

```bash
coldcase status cases/my-case
```

Write default rules:

```bash
coldcase rules coldcase.rules
```

---

## Case structure

```text
my-case/
├─ CASE.md
├─ coldcase.db
├─ evidence/
├─ exports/
└─ reports/
   ├─ coldcase_<timestamp>.html
   ├─ coldcase_<timestamp>.json
   └─ coldcase_<timestamp>.sarif
```

---

## Typical workflow

```bash
coldcase new cases/test-case --title "PC triage"
coldcase snapshot cases/test-case
coldcase scan cases/test-case C:\Users\User\Downloads
coldcase scan cases/test-case C:\Users\User\AppData\Roaming
coldcase timeline cases/test-case
coldcase report cases/test-case
```

Open the generated HTML report in `reports/`.

---

## What COLDCASE detects

COLDCASE labels artifacts such as:

- script files,
- executable files,
- archives,
- files in sensitive paths,
- high entropy artifacts,
- risky filename keywords,
- suspicious startup entries from snapshot output.

The tool does not decide guilt.  
It raises review points.

---

## Dashboard

The dashboard is static and local:

```text
dashboard/index.html
```

It can load a generated COLDCASE JSON report.

---

## Build

Requires Rust.

```bash
cargo build --release
```

Windows helper:

```bat
build_windows\BUILD_RELEASE.bat
```

---

## Safety

COLDCASE does not:

- delete files,
- quarantine files,
- disable startup entries,
- kill processes,
- upload evidence,
- exploit anything,
- modify the system silently.

It is a local evidence organization and reporting tool.

---

## License

MIT.

**xtr4ng3**
