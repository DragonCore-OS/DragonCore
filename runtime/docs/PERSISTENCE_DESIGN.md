# Run State Persistence Design | 运行状态持久化设计

**Status**: Required for v0.2.0  
**Priority**: P0 Blocker  
**Blocks**: RV-004, RV-005, RV-008, RV-009, RV-010

---

## Problem Statement | 问题陈述

**Current Architecture (v0.1.0)**:
```rust
// Each CLI command executes:
let runtime = DragonCoreRuntime::new(config).await?;
// Creates NEW GovernanceEngine with EMPTY HashMap
```

**Result**: 
- Run created by `dragoncore-runtime run` exists only in that process's memory
- Subsequent commands (`execute`, `veto`, `final-gate`) spawn new processes
- New processes have empty state → "Run not found" errors
- Governance lifecycle cannot complete

**Impact**: 
- ✅ API execution works (seat execution verified)
- ✅ Process isolation works (tmux/worktree verified)
- ❌ Governance closure impossible (no state continuity)

---

## Solution: Durable Run State | 解决方案：持久化运行状态

### v0.2.0: JSON-File Persistence (Immediate)

**Design Philosophy**: 
- Fast to implement
- Human-readable (easy to debug)
- Atomic writes (temp file + rename)
- File-lock for basic concurrency
- Single-node only (sufficient for verification)

**Directory Structure**:
```
~/.local/share/runtime/state/
├── runs/
│   ├── RUN-20260314_012029-ad7b8a9d.json
│   ├── RUN-20260314_012208-6364ccde.json
│   └── TEST-VERIFY-001.json
├── ledger/
│   └── production_ledger.csv
└── artifacts/
    └── RUN-20260314_012029-ad7b8a9d/
        ├── tianshu_output.md
        ├── yuheng_output.md
        └── final_decision.md
```

**Run State JSON Schema**:
```json
{
  "run_id": "RUN-20260314_012029-ad7b8a9d",
  "status": "running",
  "task": "Test OAuth2 implementation",
  "input_type": "feature",
  "created_at": "2026-03-14T01:20:29Z",
  "updated_at": "2026-03-14T01:22:24Z",
  "current_seat": "Yuheng",
  "final_gate": null,
  "worktree_path": "/home/admin/.local/share/runtime/worktrees/RUN-...",
  "tmux_session": "dragoncore_RUN-...",
  "seats_participated": ["Tianquan", "Yuheng"],
  "events": [
    {
      "timestamp": "2026-03-14T01:20:29Z",
      "seat": "Tianquan",
      "action": "execute",
      "output_file": "tianshu_output.md"
    },
    {
      "timestamp": "2026-03-14T01:22:24Z",
      "seat": "Yuheng",
      "action": "veto",
      "reason": "Security vulnerability"
    }
  ],
  "artifacts": ["tianshu_output.md", "yuheng_output.md"],
  "metrics": {
    "tokens_used": 2048,
    "wall_clock_seconds": 115
  }
}
```

**Critical Rule**: 
> Every state transition = memory update + durable write (not either/or)

---

## Implementation Plan | 实现计划

### Step 1: RunStore Trait

```rust
pub trait RunStore: Send + Sync {
    fn create_run(&self, run: &RunState) -> Result<()>;
    fn load_run(&self, run_id: &str) -> Result<RunState>;
    fn save_run(&self, run: &RunState) -> Result<()>;
    fn append_event(&self, run_id: &str, event: RunEvent) -> Result<()>;
    fn list_runs(&self) -> Result<Vec<RunState>>;
}

pub struct JsonFileStore {
    state_dir: PathBuf,
}

impl RunStore for JsonFileStore {
    fn create_run(&self, run: &RunState) -> Result<()> {
        let path = self.run_path(&run.run_id);
        let temp_path = path.with_extension("tmp");
        
        // Atomic write: write to temp, then rename
        let json = serde_json::to_string_pretty(run)?;
        fs::write(&temp_path, json)?;
        fs::rename(&temp_path, &path)?;
        
        Ok(())
    }
    // ...
}
```

### Step 2: Update GovernanceEngine

```rust
pub struct GovernanceEngine {
    runs: HashMap<String, RunState>, // Cache
    store: Box<dyn RunStore>,        // Source of truth
}

impl GovernanceEngine {
    pub fn new(store: Box<dyn RunStore>) -> Result<Self> {
        // Load all runs from disk into memory
        let runs = store.list_runs()?.into_iter()
            .map(|r| (r.run_id.clone(), r))
            .collect();
        Ok(Self { runs, store })
    }
    
    pub fn exercise_veto(&mut self, run_id: &str, seat: Seat, reason: String) -> Result<()> {
        // Load from disk (source of truth)
        let mut run = self.store.load_run(run_id)?;
        
        // Modify
        run.status = RunStatus::Vetoed;
        run.veto = Some(VetoRecord { seat, reason, timestamp: Utc::now() });
        
        // Persist FIRST
        self.store.save_run(&run)?;
        
        // Then update cache
        self.runs.insert(run_id.to_string(), run);
        
        Ok(())
    }
}
```

---

## Success Criteria | 成功标准

v0.2.0 persistence is complete when:

- [ ] `run` creates run.json file
- [ ] `execute` finds and updates that run
- [ ] `veto` persists veto decision
- [ ] `final-gate` changes status to Approved/Rejected
- [ ] `archive` creates archive record
- [ ] Ledger CSV appended on every state change
- [ ] Process restart doesn't lose state
- [ ] Concurrent commands don't corrupt state

**Target**: 10/10 verification within 1 week.
