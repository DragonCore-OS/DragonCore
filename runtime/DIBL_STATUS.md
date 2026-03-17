# DIBL v0.1 - FROZEN ✅

**Date**: 2026-03-16  
**Version**: v0.1-FROZEN  
**Status**: ✅ Production Ready - Schema Locked

---

## Summary

DIBL (DragonCore Internal Broadcast Layer) v0.1 已與 AXI 完全對齊並凍結。

---

## Implementation Status

### DragonCore Side

| Component | Status | Location |
|-----------|--------|----------|
| Event Types | ✅ | `src/events/mod.rs` |
| Event Store (JSONL) | ✅ | `src/events/mod.rs` |
| Broadcaster (pub/sub) | ✅ | `src/events/mod.rs` |
| DIBL Manager | ✅ | `src/events/mod.rs` |
| Operator Projection | ✅ | `src/events/mod.rs` |
| 8-Point Emission | ✅ | `src/runtime/mod.rs` |
| Policy Filter | ✅ | `src/events/mod.rs` |

### AXI Side

| Component | Status |
|-----------|--------|
| Event Types | ✅ |
| Event Store (JSONL) | ✅ |
| Broadcaster (pub/sub) | ✅ |
| 8-Point Emission | ✅ |
| CLI Tools (runs, watch) | ✅ |

---

## Schema Alignment (FROZEN)

| Field | Type | Status |
|-------|------|--------|
| event_id | Uuid | ✅ |
| run_id | String | ✅ |
| seat_id | Option<String> | ✅ |
| channel | EventChannel | ✅ snake_case |
| event_type | GovernanceEventType | ✅ snake_case |
| scope | EventScope | ✅ snake_case |
| severity | Severity | ✅ snake_case |
| summary | String | ✅ |
| details_ref | Option<String> | ✅ |
| artifact_refs | Vec<String> | ✅ |
| created_at | DateTime<Utc> | ✅ |
| correlation_id | Option<String> | ✅ |
| parent_event_id | Option<Uuid> | ✅ |
| actor | String | ✅ |
| trigger_context | Option<String> | ✅ |

**Serialization**: All enums use `#[serde(rename_all = "snake_case")]`

---

## 8-Point Emission (Both Sides)

| # | Event | Channel | Scope | Actor |
|---|-------|---------|-------|-------|
| 1 | RunCreated | Control | OperatorVisible | system/operator |
| 2 | SeatStarted | Control | Internal | {seat} |
| 3 | SeatCompleted | Research | Internal | {seat} |
| 4 | RiskRaised | Security | OperatorVisible | system/{seat} |
| 5 | VetoExercised | Security | OperatorVisible | {seat} |
| 6 | FinalGateOpened | Control | OperatorVisible | Tianshu |
| 7 | DecisionCommitted | Control | Exportable | Tianshu |
| 8 | ArchiveCompleted | Ops | OperatorVisible | {seat} |
| 8 | TerminateTriggered | Security | OperatorVisible | {seat} |

---

## Storage Format (FROZEN)

```
runtime_state/events/{run_id}.jsonl
```

- Append-only JSON Lines
- Each line = one GovernanceEvent
- Human-readable, debuggable
- Cross-platform compatible

---

## Interop Verification

| Test | Status |
|------|--------|
| DragonCore parses AXI events | ✅ |
| AXI parses DragonCore events | ✅ |
| snake_case serialization | ✅ |
| All 14 fields compatible | ✅ |
| Test vectors exchanged | ✅ |

Test vectors:
- `test_vectors/dragoncore_sample.jsonl` (8 events)
- `test_vectors/axi_sample.jsonl` (4 events)

---

## Core Principles (Locked)

1. **JSON/Ledger is source of truth** - Broadcast is derived view
2. **Persist before broadcast** - Event emitted only after durable write
3. **Non-blocking emission** - Event failure doesn't block main operation
4. **Layered visibility** - Internal/OperatorVisible/Exportable
5. **Snake_case everywhere** - Cross-platform serialization

---

## Public API (Stable)

```rust
// Load events
pub fn load_run_events(&self, run_id: &str) -> Result<Vec<GovernanceEvent>>;

// Replay events
pub fn replay_run_events(&self, run_id: &str) -> Result<Vec<GovernanceEvent>>;

// Operator projection (filtered)
pub fn get_operator_projection(&self, run_id: &str) -> Result<OperatorProjection>;

// Check events exist
pub fn run_has_events(&self, run_id: &str) -> bool;
```

---

## Next Steps (Post-v0.1)

### v0.2 Candidates (Not Started)

- Event replay reconstruction
- Ledger integration (event refs in CSV)
- Metrics aggregation from events
- Operator dashboard UI

### v0.3 Candidates (Not Started)

- Distributed event sync
- Event query interface
- Historical analysis tools

---

## Verification

| Check | Status |
|-------|--------|
| Build | ✅ Success |
| Unit Tests | ✅ 5 passed |
| Interop Test | ✅ Passed |
| Schema Alignment | ✅ Confirmed |
| Documentation | ✅ Complete |

---

## Freeze Notice

**DIBL v0.1 is FROZEN as of 2026-03-16.**

Changes require:
1. Cross-team review (DragonCore + AXI)
2. Backward compatibility assessment
3. Version bump to v0.2

---

**Signed**: DragonCore Team + AXI Team  
**Date**: 2026-03-16
