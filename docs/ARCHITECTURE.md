# Architecture

COLDCASE uses a case-based architecture:

```text
case workspace
  -> SQLite database
  -> snapshot collector
  -> file scanner
  -> rule evaluator
  -> timeline builder
  -> report generator
  -> static dashboard
```

Main components:

- `case.rs`: case creation and status.
- `collectors.rs`: system snapshot collectors.
- `scanner.rs`: folder scanning.
- `hashing.rs`: SHA-256 and entropy.
- `rules.rs`: triage rules.
- `db.rs`: SQLite storage.
- `report.rs`: HTML/JSON/SARIF generation.
