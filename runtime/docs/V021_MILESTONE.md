# DragonCore Runtime v0.2.1 Milestone

**Goal**: Ledger correctness and metrics accuracy  
**Target**: Fix RV-005 and RV-010  
**Previous**: v0.2.0 Persistence Verified ✅

---

## Problem Statement

### Current State
- Persistence layer: ✅ Working
- State files: ✅ Correctly updated
- Ledger file: ⚠️ Created but empty (header only)
- Metrics: ⚠️ Show zeros (depends on ledger)

### Evidence
```bash
# Ledger has header but no data
$ cat runtime_state/ledger/production_ledger.csv
run_id,timestamp,input_type,final_state,seats_participated,veto_used,...
# (no data rows)

# Metrics show zero
$ dragoncore-runtime metrics
Total runs: 0  <-- Should be 2
Authority violations: 0
```

### Root Cause
Ledger writes buffered, only flushed on `Drop`. Need explicit flush after transitions.

---

## Implementation Plan

### 1. Add Ledger Flush Method

**File**: `src/ledger/mod.rs`

```rust
impl Ledger {
    /// Explicitly flush ledger to disk
    pub fn flush(&mut self) -> Result<()> {
        if let Some(ref path) = self.storage_path {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(false)
                .open(path)?;
            
            let mut writer = csv::Writer::from_writer(file);
            
            // Re-write all entries
            for entry in &self.entries {
                writer.serialize(entry)?;
            }
            
            writer.flush()?;
        }
        Ok(())
    }
}
```

### 2. Update Runtime to Flush After Transitions

**File**: `src/runtime/mod.rs`

Update methods to call `ledger.flush()`:
- `exercise_veto()` - After recording veto
- `final_gate()` - After finalizing
- `archive_run()` - After archiving
- `terminate_run()` - After terminating

Example:
```rust
pub async fn exercise_veto(&self, run_id: &str, seat: Seat, reason: &str) -> Result<()> {
    // ... existing veto logic ...
    
    // Record in ledger AND flush
    {
        let mut ledger = self.ledger.write().await;
        ledger.record_veto(seat)?;
        ledger.flush()?;  // <-- ADD THIS
    }
    
    Ok(())
}
```

### 3. Verify Fix

**Test Sequence**:
```bash
# Create run
$ dragoncore-runtime run --run-id ledger-test --input-type code -t "Test ledger"

# Check ledger has entry
$ cat runtime_state/ledger/production_ledger.csv | wc -l
2  # header + 1 data row

# Veto
$ dragoncore-runtime veto --run-id ledger-test --seat Yuheng --reason "test"

# Check ledger updated
$ cat runtime_state/ledger/production_ledger.csv | grep veto
# Should show veto_used=true

# Check metrics
$ dragoncore-runtime metrics
Total runs: 1  <-- Should show 1
```

---

## Acceptance Criteria

| Check | Criteria | Status |
|-------|----------|--------|
| RV-005 | Ledger has data rows after each transition | ⬜ |
| RV-010 | Metrics show correct counts | ⬜ |
| Regression | All v0.2.0 tests still pass | ⬜ |

---

## Files to Modify

1. `src/ledger/mod.rs` - Add `flush()` method
2. `src/runtime/mod.rs` - Call `flush()` after state changes
3. `Cargo.toml` - May need `csv` crate if not present

---

## Estimated Effort

- Implementation: 30-60 minutes
- Testing: 15-30 minutes
- Documentation: 15 minutes

**Total**: ~2 hours

---

## Success Definition

**v0.2.1 Complete When**:
```bash
# All these commands work:
$ dragoncore-runtime run --run-id test1 ...
$ dragoncore-runtime veto --run-id test1 ...
$ dragoncore-runtime final-gate --run-id test1 ...

# Ledger shows data:
$ cat runtime_state/ledger/production_ledger.csv
run_id,timestamp,...
test1,2026-03-14...,code,Approved,...,true,...

# Metrics accurate:
$ dragoncore-runtime metrics
Total runs: 1
Clean runs: 1
```

**Achievement**: 
> "DragonCore Runtime is operationally verified for the single-node JSON-backed path."

---

## Relation to v0.2.0

v0.2.0 proved: **State CAN persist across CLI invocations**  
v0.2.1 will prove: **Ledger and metrics correctly track that state**

Together: **Single-node operational verification complete.**
