# DragonCore Public Beta Roadmap

**Current Version**: v0.2.1-DIBL  
**Target Public Beta**: v0.3.0  
**Date**: 2026-03-16

---

## Current Status Summary

### ✅ Verified Core (v0.2.1)

| Module | Lines | Status | Coverage |
|--------|-------|--------|----------|
| `main.rs` | 379 | ✅ CLI complete | 13 commands |
| `config/` | 199 | ✅ Config system | TOML load/save |
| `events/` | 632 | ✅ DIBL v0.1 FROZEN | 5 tests pass |
| `governance/` | 375 | ✅ 19-seat core | Authority chain |
| `ledger/` | 457 | ✅ CSV ledger | Metrics accurate |
| `models/` | 532 | ✅ Multi-provider | Kimi/DeepSeek/Qwen |
| `persistence/` | 288 | ✅ JSON store | Cross-CLI verified |
| `runtime/` | 518 | ✅ Runtime core | 8-point emission |
| `tmux/` | 289 | ✅ Isolation | Session management |
| `worktree/` | 352 | ✅ Git isolation | Worktree ops |
| **Total** | **4,021** | **✅ Core stable** | **Production ready** |

### ✅ DIBL v0.1 (FROZEN)

- Schema aligned with AXI
- 8-point event emission
- Interop verified (both directions)
- Test vectors exchanged

---

## Pre-Public Beta Checklist

### 1. Testing & Validation

#### Unit Tests (Current: 5 tests)
- [x] JSON file store persistence
- [x] JSONL event store append/load
- [x] Operator projection
- [x] Correlation context
- [x] AXI interop parsing

#### Integration Tests (Needed: 10+ tests)
- [ ] End-to-end governance flow
- [ ] Cross-CLI state continuity
- [ ] Tmux session lifecycle
- [ ] Worktree creation/cleanup
- [ ] Model provider fallback
- [ ] Ledger accuracy under load
- [ ] Event replay correctness
- [ ] Configuration reload
- [ ] Error recovery paths
- [ ] Concurrent run isolation

#### System Tests (Needed)
- [ ] Long-running stability (4+ hours)
- [ ] Memory leak detection
- [ ] Disk space handling
- [ ] Network failure recovery
- [ ] Git edge cases (large repos, submodules)

### 2. Documentation

#### User Documentation
- [x] README (bilingual)
- [x] Usage guide
- [x] 19 seats reference
- [x] DIBL schema docs
- [ ] Installation guide (detailed)
- [ ] Troubleshooting guide
- [ ] Configuration reference
- [ ] API documentation (generated)

#### Developer Documentation
- [x] Architecture overview
- [ ] Contributing guide
- [ ] Testing guide
- [ ] Release process

### 3. Packaging & Distribution

- [x] Cargo.toml configured
- [x] Release profile optimized
- [ ] Debian package (.deb)
- [ ] macOS binary
- [ ] Docker image
- [ ] Install script (`install.sh` exists but needs validation)

### 4. CLI Completeness

| Command | Status | Notes |
|---------|--------|-------|
| `init` | ✅ | Config initialization |
| `run` | ✅ | Start governance run |
| `execute` | ✅ | Execute seat |
| `veto` | ✅ | Exercise veto |
| `final-gate` | ✅ | Approve/reject |
| `archive` | ✅ | Archive run |
| `terminate` | ✅ | Terminate run |
| `status` | ✅ | Show status |
| `metrics` | ✅ | Stability metrics |
| `attach` | ✅ | Tmux attach |
| `seats` | ✅ | List 19 seats |
| `cleanup` | ✅ | Resource cleanup |
| `events` | ⏳ | NEW: View events (DIBL) |
| `replay` | ⏳ | NEW: Replay run events |

### 5. Observability

- [x] Tracing/logging
- [x] JSONL events
- [x] CSV ledger
- [ ] Metrics export (Prometheus)
- [ ] Health check endpoint
- [ ] Status dashboard

---

## Public Beta Definition

### What "Public Beta" Means

1. **Core runtime is stable** - v0.2.1 verified path preserved
2. **DIBL is frozen** - v0.1 schema locked with AXI
3. **CLI is complete** - All essential commands work
4. **Documentation is sufficient** - Users can install and run
5. **Known issues documented** - No hidden blockers

### What It Does NOT Mean

1. **Feature complete** - v0.4+ will add more
2. **Bug free** - Beta means finding edge cases
3. **Production hardened** - Use at your own risk
4. **API stable** - Internal APIs may change

---

## Release Schedule

### Phase 1: Pre-Beta (1 week)

**Goal**: Stabilize existing features

- [ ] Add 10 integration tests
- [ ] Add `events` CLI command
- [ ] Add `replay` CLI command
- [ ] Validate install script
- [ ] Write troubleshooting guide
- [ ] Fix all compiler warnings

### Phase 2: Beta Release (1 week)

**Goal**: First public release

- [ ] Tag v0.3.0-beta.1
- [ ] Publish release notes
- [ ] Announce on channels
- [ ] Collect feedback

### Phase 3: Beta Iteration (2-4 weeks)

**Goal**: Stabilize based on feedback

- [ ] Address critical issues
- [ ] Add requested features
- [ ] Improve documentation
- [ ] Tag v0.3.0 (stable)

---

## Success Criteria for Public Beta

| Metric | Target |
|--------|--------|
| Installation success rate | >90% |
| First run success rate | >80% |
| CLI command coverage | 100% essential |
| Test pass rate | 100% |
| Documentation completeness | >80% |
| Known issues documented | 100% |

---

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Git version incompatibility | Medium | High | Document required version |
| Tmux not installed | Medium | High | Auto-detect, warn user |
| API key exposure | Low | High | File permissions, docs |
| Large repo performance | Medium | Medium | Test with realistic repos |
| Concurrent run conflicts | Low | High | Add locking mechanism |

---

## Immediate Next Actions

### Today (DragonCore Team)

1. [ ] Add `events` CLI command
2. [ ] Add `replay` CLI command
3. [ ] Add integration test scaffolding
4. [ ] Validate install.sh

### This Week

1. [ ] Write 10 integration tests
2. [ ] Write troubleshooting guide
3. [ ] Test on clean machine
4. [ ] Fix compiler warnings

---

## Sign-off

**Public Beta Ready When**:
- [ ] All Phase 1 items complete
- [ ] Success criteria met
- [ ] No critical known issues
- [ ] Documentation sufficient for new users

**Target Date**: 2026-03-30 (2 weeks)
