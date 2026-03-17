# DragonCore Runtime v0.3.0-beta.1

**Release Date**: 2026-03-16  
**Status**: Public Beta  
**Tag**: `v0.3.0-beta.1`

---

## What's New

### DIBL v0.1 (DragonCore Internal Broadcast Layer)

- **Schema frozen** and aligned with AXI
- **14 fields** including `actor`, `correlation_id`, `parent_event_id`
- **snake_case** serialization for cross-platform compatibility
- **JSONL storage** at `runtime_state/events/{run_id}.jsonl`
- **8-point event emission** covering full governance lifecycle

### New CLI Commands

```bash
dragoncore events --run-id <id>          # View DIBL events
dragoncore events --format json          # JSON output
dragoncore replay --run-id <id>          # Replay events with projection
```

### Testing

- **11 unit tests** (100% passing)
- **AXI interop verified** (bidirectional parsing)
- **0 compiler warnings** in release build
- **Beta release verification script** included

---

## Current Limitations (Beta)

This is a **beta release**. Known limitations:

- **Single-node only** - Distributed mode planned for v0.6.0
- **JSON-backed** - SQLite backend planned for v0.4.0
- **Linux/WSL** - Windows Desktop planned for v0.5.0
- **CLI only** - Web UI not yet available

---

## Installation

```bash
# Download and extract
curl -L https://github.com/DragonCore-OS/DragonCore/releases/download/v0.3.0-beta.1/dragoncore-runtime-v0.3.0-beta.1-linux-x64.tar.gz | tar xz

# Run verification
./dragoncore-runtime --version
./verify_beta_release.sh
```

---

## Verification

Run the beta release verification script:

```bash
cd runtime
./verify_beta_release.sh
```

All checks must pass.

---

## Feedback

This is a **beta release**. Please report issues:

- GitHub Issues: https://github.com/DragonCore-OS/DragonCore/issues
- DIBL Schema: Frozen v0.1 (no breaking changes planned)

---

## DIBL v0.1 Schema (Frozen)

```rust
struct GovernanceEvent {
    event_id: Uuid,
    run_id: String,
    seat_id: Option<String>,
    channel: EventChannel,      // control | ops | security | research
    event_type: GovernanceEventType,
    scope: EventScope,          // internal | operator_visible | exportable
    severity: Severity,         // info | warn | critical
    summary: String,
    details_ref: Option<String>,
    artifact_refs: Vec<String>,
    created_at: DateTime<Utc>,
    correlation_id: Option<String>,
    parent_event_id: Option<Uuid>,
    actor: String,              // who triggered the event
    trigger_context: Option<String>,
}
```

---

**Signed**: DragonCore Team  
**Date**: 2026-03-16
