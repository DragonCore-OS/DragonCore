# DragonCore Runtime Verification Checklist | 验证清单

**Version**: 0.1.0  
**Status**: 🔴 Not Started - Awaiting API Key  
**Objective**: Operational verification before production consideration

---

## Status Upgrade Rules | 状态升级规则

| Verified Count | Status Level | Description |
|----------------|--------------|-------------|
| 0/10 | 🔴 buildable skeleton | Code exists, compiles, CLI static commands work |
| 1-5/10 | 🟡 single-path partially verified | Core governance chain executes with real API |
| 6-9/10 | 🟢 operationally verified for tested path | Isolation, ledger, metrics all functional |
| 10/10 | ✅ fully verified | All mechanisms tested and documented |

**Current Status**: 🔴 buildable skeleton (0/10 verified)

**Goal**: Reach 🟡 single-path partially verified (5/10) with minimal test run.

---

## Verification Milestone 1: Single-Path Operational Verification

**Goal**: Verify one complete governance chain works end-to-end.

**Success Criteria**: All 5 checks below transition from `pending` to `verified`.

---

## Checklist

| Check ID | Mechanism | Command | Expected Artifact | Status |
|----------|-----------|---------|-------------------|--------|
| RV-001 | Configuration Init | `dragoncore-runtime init --output .` | `dragoncore.toml` created with correct structure | 🔴 pending |
| RV-002 | Governance Run | `dragoncore-runtime run --task "Implement OAuth2"` | Run ID returned, tmux session created, worktree created | 🔴 pending |
| RV-003 | Seat Execution | `dragoncore-runtime execute --run-id <ID> --seat Tianquan --task "Plan implementation"` | Model response received, output written to artifact | 🔴 pending |
| RV-004 | Veto Chain | `dragoncore-runtime veto --run-id <ID> --seat Yuheng --reason "Security issue"` | Run state changed to REJECTED, veto recorded in ledger | 🔴 pending |
| RV-005 | Ledger Auto-Write | After any run operation | `~/.local/share/runtime/ledger/production_ledger.csv` appended with correct fields | 🔴 pending |
| RV-006 | tmux Isolation | During active run | `tmux ls` shows `dragoncore_RUN-<id>` with 19 windows | 🔴 pending |
| RV-007 | Worktree Isolation | After run start | `~/.local/share/runtime/worktrees/RUN-<id>/` exists as independent git worktree | 🔴 pending |
| RV-008 | Final Gate | `dragoncore-runtime final-gate --run-id <ID> --approve` | Run state changed to APPROVED | 🔴 pending |
| RV-009 | Archive | `dragoncore-runtime archive --run-id <ID> --seat Yaoguang` | Run state changed to ARCHIVED, artifacts preserved | 🔴 pending |
| RV-010 | Metrics Update | After archive | `dragoncore-runtime metrics` shows updated counters | 🔴 pending |

---

## Prerequisites for Verification

Before starting verification, ensure:

- [ ] Valid API key obtained (Kimi/DeepSeek/Qwen)
- [ ] API key set as environment variable or in config
- [ ] tmux installed and working
- [ ] git installed and working
- [ ] Binary compiled: `cargo build --release`

---

## Verification Procedure

### Step 1: Environment Setup

```bash
# Set API key
export KIMI_API_KEY="sk-your-actual-key-here"

# Verify key is set
echo $KIMI_API_KEY
```

**Expected**: API key displayed (masked in logs)

---

### Step 2: Initialize (RV-001)

```bash
./target/release/dragoncore-runtime init --output /tmp/test-verification
```

**Verify**:
```bash
ls -la /tmp/test-verification/dragoncore.toml
```

**Expected**: Config file exists with valid TOML structure

**Status Update**: Change RV-001 from 🔴 to 🟡 in-progress, then ✅ verified

---

### Step 3: Start Governance Run (RV-002)

```bash
./target/release/dragoncore-runtime run --task "Implement OAuth2 authentication"
```

**Verify**:
```bash
# Check run ID returned
tmux ls | grep dragoncore
ls ~/.local/share/runtime/worktrees/
```

**Expected**:
- Run ID displayed (e.g., `RUN-20240314_120000-a1b2c3d4`)
- tmux session exists: `dragoncore_RUN-xxx`
- Worktree directory created

**Status Update**: RV-002 → ✅ verified

---

### Step 4: Execute Seat (RV-003)

```bash
./target/release/dragoncore-runtime execute \
  --run-id RUN-<id> \
  --seat Tianquan \
  --task "Create OAuth2 implementation plan"
```

**Verify**:
- Model response received (may take 10-30 seconds)
- No API errors

**Expected**: Text response from model with execution plan

**Status Update**: RV-003 → ✅ verified

---

### Step 5: Exercise Veto (RV-004)

```bash
./target/release/dragoncore-runtime veto \
  --run-id RUN-<id> \
  --seat Yuheng \
  --reason "Missing input validation - security vulnerability"
```

**Verify**:
```bash
./target/release/dragoncore-runtime status --run-id RUN-<id>
```

**Expected**: Status shows `REJECTED`

**Status Update**: RV-004 → ✅ verified

---

### Step 6: Check Ledger (RV-005)

```bash
cat ~/.local/share/runtime/ledger/production_ledger.csv
```

**Expected**:
- CSV headers present
- At least one data row
- Fields populated: run_id, timestamp, final_state, veto_used, etc.

**Status Update**: RV-005 → ✅ verified

---

### Step 7: Verify tmux (RV-006)

```bash
tmux ls
tmux lsw -t dragoncore_RUN-<id>
```

**Expected**:
- Session exists: `dragoncore_RUN-<id>`
- 19 windows (one per seat)

**Status Update**: RV-006 → ✅ verified

---

### Step 8: Verify Worktree (RV-007)

```bash
cd ~/.local/share/runtime/worktrees/RUN-<id>
git status
pwd
```

**Expected**:
- Directory exists
- Git working tree (detached HEAD)
- Path independent from main repo

**Status Update**: RV-007 → ✅ verified

---

### Step 9: Final Gate (RV-008)

Start a new run for approval test:

```bash
./target/release/dragoncore-runtime run --task "Implement feature X"
# ... execute seats ...
./target/release/dragoncore-runtime final-gate --run-id RUN-<new-id> --approve
```

**Verify**:
```bash
./target/release/dragoncore-runtime status --run-id RUN-<new-id>
```

**Expected**: Status shows `APPROVED`

**Status Update**: RV-008 → ✅ verified

---

### Step 10: Archive (RV-009)

```bash
./target/release/dragoncore-runtime archive --run-id RUN-<id> --seat Yaoguang
```

**Verify**:
```bash
./target/release/dragoncore-runtime status --run-id RUN-<id>
ls ~/.local/share/runtime/worktrees/RUN-<id>
```

**Expected**:
- Status shows `ARCHIVED`
- Artifacts preserved in worktree

**Status Update**: RV-009 → ✅ verified

---

### Step 11: Metrics (RV-010)

```bash
./target/release/dragoncore-runtime metrics
```

**Expected**:
```
Total runs: 2
Clean runs: 1
Authority violations: 0
Fake closures: 0
Rollbacks: 0
Terminations: 0
```

**Status Update**: RV-010 → ✅ verified

---

## Completion Criteria

When all checks are ✅ verified:

1. Update `VERIFICATION_REPORT.md` with actual evidence
2. Update `SMOKE_TEST_RESULTS.md` with real outputs
3. Update README status to:
   ```
   🟢 Single-path operationally verified
   ```
4. Create follow-up milestone for:
   - Concurrent run isolation testing
   - Load testing
   - Security hardening

---

## Failure Handling

If any check fails:

1. Document failure in `KNOWN_GAPS.md`
2. Create GitHub issue with label `verification-failure`
3. Do not update status to verified
4. Fix before proceeding to next milestone

---

## Evidence Collection

For each verified check, capture:

- [ ] Terminal output (copy-paste)
- [ ] Generated file paths
- [ ] Exit codes (echo $?)
- [ ] Timestamps

Store evidence in `runtime/verification-evidence/` directory.

---

## Sign-off

**Verified By**: _______________  
**Date**: _______________  
**API Provider Used**: _______________  

All 10 checks verified: ☐

Ready for Milestone 2 (Concurrent Isolation Testing): ☐

---

*This checklist is a living document. Update as verification proceeds.*
