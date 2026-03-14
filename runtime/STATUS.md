# DragonCore Runtime - Status

**Version**: v0.2.1  
**Date**: 2026-03-14  
**Phase**: **Verified Core Runtime Phase** ✅

---

## Executive Status

> **DragonCore Runtime v0.2.1 has verified the first complete single-node JSON-backed governance path end to end.**
>
> DragonCore Runtime v0.2.1 已完成首条单节点、JSON 持久化治理路径的端到端验证。

---

## Core Statement

**This is not "almost done." This is "the first complete runtime path is verified and ready for disciplined expansion."**

**核心路径已证真，可以开始扩展，但扩展必须基于已验证路径，不得破坏它。**

---

## What Was Verified (10/10)

| # | Component | Evidence |
|---|-----------|----------|
| 1 | **Run Creation** | JSON state file created immediately |
| 2 | **Seat Execution** | Participation tracked in ledger |
| 3 | **Veto** | Cross-CLI state loading verified |
| 4 | **Final Gate** | State transition: Created → Approved |
| 5 | **Archive** | Terminal state persisted |
| 6 | **Ledger** | CSV immediate write confirmed |
| 7 | **Metrics** | "Total runs: 1" (was 0) |
| 8 | **Tmux Isolation** | 19-seat governance sessions |
| 9 | **Worktree** | Per-run git isolation |
| 10 | **Cross-CLI** | State survives process death |

---

## Phase Definition: Verified Core Runtime

### What This Means
- ✅ Core runtime loop is closed and verified
- ✅ State persistence is proven across CLI invocations
- ✅ Ledger is accurate and immediate
- ✅ Metrics correctly reflect operational state

### What This Does NOT Mean
- ❌ Not all platforms (Windows pending)
- ❌ Not all backends (SQLite pending)
- ❌ Not distributed (v0.6.0)
- ❌ Not all features (Web UI pending)

### Expansion Rule
> **扩展必须基于已验证路径，不得破坏它。**

Any expansion (Windows, SQLite, distributed) must:
1. Preserve the v0.2.1 verification boundary
2. Add, not replace, the verified path
3. Maintain backward compatibility with JSON/CSV format

---

## Verification Evidence

### End-to-End Governance Chain

```bash
# Step 1: Create
$ dragoncore run --run-id demo --input-type code -t "Verify"
[INFO] Started ledger entry for run: demo

# Step 2: Veto (new CLI process)
$ dragoncore veto --run-id demo --seat Yuheng --reason "test"
[INFO] Loaded 1 runs from persistent storage  <-- STATE FOUND

# Step 3: Finalize (new CLI process)
$ dragoncore final-gate --run-id demo --approve
[INFO] Finalized ledger entry for run: demo with state: Approved

# Step 4: Verify Metrics (new CLI process)
$ dragoncore metrics
Total runs: 1          <-- CORRECT
Clean runs: 1          <-- CORRECT
```

### State Consistency

**JSON State** (`runtime_state/runs/demo.json`):
```json
{
  "run_id": "demo",
  "status": "Approved",
  "veto": {"seat": "Yuheng", ...},
  "final_gate": {"seat": "Tianshu", "approved": true},
  "events": [...]
}
```

**CSV Ledger** (`runtime_state/ledger/production_ledger.csv`):
```csv
demo,2026-03-14T02:27:01Z,code,Approved,0,Yuheng,...,true
```

**Metrics Output**:
```
Total runs: 1
Clean runs: 1
```

All three sources consistent for same `run_id`.

---

## Current Phase: Verified Core Runtime

```
┌─────────────────────────────────────────────────────────────┐
│                    VERIFIED CORE RUNTIME                     │
│                         v0.2.1                               │
│                                                              │
│  ✅ Single-node JSON path                                    │
│  ✅ Cross-CLI state continuity                               │
│  ✅ Immediate ledger persistence                             │
│  ✅ Accurate metrics                                         │
│  ✅ 19-seat governance                                       │
│  ✅ Tmux + worktree isolation                               │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
        ┌──────────┐    ┌──────────┐    ┌──────────┐
        │ Windows  │    │  SQLite  │    │   Web    │
        │ Desktop  │    │ Backend  │    │   UI     │
        │  v0.4.0? │    │  v0.3.0? │    │  v0.5.0? │
        └──────────┘    └──────────┘    └──────────┘
              │
              ▼
        Disciplined Expansion
        (Must preserve v0.2.1 core)
```

---

## Next Decision: Two Routes

### Route A: Windows Desktop (WSL-backed)
**Goal**: Lower user installation barrier, expand to Windows developers.

**Scope**:
- Windows installer (MSI)
- WSL integration layer
- Windows Terminal integration
- Optional Tauri GUI

**Value**: High (user expansion)  
**Risk**: Medium (new platform)  
**Effort**: 2-3 weeks

### Route B: v0.3.0 SQLite Persistence
**Goal**: Strengthen local runtime with better backend.

**Scope**:
- `SqliteRunStore` implementation
- Migration from JSON
- SQL-based ledger
- Query interface

**Value**: Medium (production hardening)  
**Risk**: Low (additive)  
**Effort**: 1-2 weeks

### Decision Criteria

| Criteria | Windows | SQLite |
|----------|---------|--------|
| User Growth | High | Low |
| Technical Risk | Medium | Low |
| Strategic Value | High | Medium |

See [NEXT_MILESTONE.md](NEXT_MILESTONE.md) for detailed planning.

---

## Engineering Discipline

### Frozen (v0.2.1 Core)
- JSON persistence format
- CSV ledger schema
- Cross-CLI state loading pattern
- Governance authority chain
- 19-seat structure

### Open for Expansion
- Additional platforms (Windows, macOS)
- Additional backends (SQLite)
- Additional interfaces (GUI, Web)
- Additional features (distributed)

### Golden Rule
> **扩展必须基于已验证路径，不得破坏它。**

New features must:
1. Use existing `RunStore` trait
2. Maintain JSON/CSV compatibility
3. Preserve cross-CLI continuity
4. Keep metrics accurate

---

## Documentation

| File | Purpose |
|------|---------|
| `RELEASE_NOTES_v0.2.1.md` | Full verification evidence |
| `NEXT_MILESTONE.md` | Route A vs Route B analysis |
| `docs/VERIFICATION_REPORT.md` | Detailed test results |
| `docs/PERSISTENCE_DESIGN.md` | Architecture decisions |

---

## Build & Verify

```bash
# Current verified version
cargo build --release
./target/release/dragoncore-runtime --version
# dragoncore-runtime 0.2.1

# Verify end-to-end
dragoncore run --run-id test --input-type code -t "Verify"
dragoncore veto --run-id test --seat Yuheng --reason "test"
dragoncore final-gate --run-id test --approve
dragoncore metrics
# Expected: Total runs: 1
```

---

## Final Statement

**v0.2.1 is the first "solid ground" version of DragonCore Runtime.**

It is not a prototype. It is a verified, operational runtime path that can be:
- Deployed (single-node)
- Extended (disciplined expansion)
- Built upon (Windows, SQLite, distributed)

**The foundation is set. Expansion begins.**

---

**Phase**: Verified Core Runtime ✅  
**Status**: v0.2.1 Complete  
**Next**: Disciplined Expansion (Route A or B)  
**Rule**: Preserve the verified core
