# DragonCore Runtime - Next Milestone Planning

**Current**: v0.2.1 Complete ✅ (Single-node JSON path verified)  
**Date**: 2026-03-14

---

## Current State

**Verified Boundary**:
- Single-node deployment
- JSON persistence
- CSV ledger
- Cross-CLI state continuity
- 19-seat governance
- Linux/WSL environment

**Status**: Production-ready for verified boundary.

---

## Candidate Next Directions

### Option A: v0.3.0 - SQLite Persistence

**Goal**: Add SQLite backend for better concurrent access and querying.

**Motivation**:
- JSON files are human-readable but not optimal for complex queries
- SQLite enables better concurrent access patterns
- Foundation for future multi-node (SQLite can be networked)

**Scope**:
- [ ] `SqliteRunStore` implementation of `RunStore` trait
- [ ] Migration path from JSON to SQLite
- [ ] Query interface for run history
- [ ] Ledger as SQL table (instead of CSV)
- [ ] Performance benchmarking

**Effort**: ~1-2 weeks  
**Risk**: Low (additive feature)  
**Value**: Medium (production hardening)

---

### Option B: Windows Desktop (WSL-backed)

**Goal**: Native Windows Desktop experience with WSL backend.

**Motivation**:
- Expand user base to Windows developers
- Desktop GUI for governance visualization
- System tray integration for runtime status

**Scope**:
- [ ] Windows installer (MSI/MSIX)
- [ ] WSL integration layer
- [ ] Windows Terminal integration
- [ ] Optional: Tauri-based GUI
- [ ] Windows service mode

**Effort**: ~2-3 weeks  
**Risk**: Medium (new platform)  
**Value**: High (user expansion)

**Implementation Approach**:
```
Windows Frontend (Tauri/Electron)
    ↓
WSL Bridge (named pipes/HTTP)
    ↓
DragonCore Runtime (inside WSL)
    ↓
Linux filesystem, tmux, git
```

---

## Decision Matrix

| Criteria | SQLite v0.3.0 | Windows Desktop |
|----------|---------------|-----------------|
| Technical Risk | Low | Medium |
| User Impact | Low (devs) | High (new users) |
| Code Complexity | Low | Medium |
| Maintenance Burden | Low | Medium |
| Strategic Value | Medium | High |
| Effort | 1-2 weeks | 2-3 weeks |

---

## Recommendation

**Primary**: Windows Desktop (WSL-backed)

Rationale:
1. v0.2.1 is stable enough for new platform
2. Windows users are significant portion of developer market
3. WSL provides good enough Linux environment
4. Can leverage existing v0.2.1 foundation

**Secondary**: SQLite v0.3.1

Rationale:
1. Add after Windows to avoid coupling
2. SQLite is well-understood technology
3. Can be done in parallel with Windows testing

---

## Windows Desktop Milestone Plan

### Phase 1: WSL Integration (Week 1)
- [ ] Detect WSL installation
- [ ] WSL distro management (Ubuntu default)
- [ ] Windows-to-WSL path translation
- [ ] Command proxy (dragoncore.exe → wsl dragoncore)

### Phase 2: Installation (Week 1-2)
- [ ] MSI installer
- [ ] WSL auto-setup
- [ ] Runtime auto-deploy to WSL
- [ ] Windows PATH integration

### Phase 3: Desktop GUI (Week 2-3)
- [ ] Tauri scaffold
- [ ] Run status dashboard
- [ ] Seat participation view
- [ ] Ledger visualization
- [ ] Metrics charts

### Phase 4: Polish (Week 3)
- [ ] System tray
- [ ] Auto-update
- [ ] Documentation
- [ ] Release

---

## SQLite v0.3.1 Plan (if Windows first)

### Week 1: Implementation
- [ ] `SqliteRunStore` struct
- [ ] Schema design
- [ ] Migration tool

### Week 2: Integration
- [ ] Configuration option
- [ ] Backward compatibility
- [ ] Testing

---

## Open Questions

1. **Windows Store**: Do we want Microsoft Store distribution?
2. **macOS**: Should we plan macOS version after Windows?
3. **Cloud**: Do we need cloud-hosted option?
4. **Plugin API**: When to add WASM plugin support?

---

## Immediate Next Steps

Regardless of direction:

1. [ ] Tag v0.2.1 release
2. [ ] Update documentation
3. [ ] Archive verification evidence
4. [ ] Create decision record

Then:

**If Windows**: Create `windows/` directory, start WSL detection  
**If SQLite**: Create `src/persistence/sqlite.rs`, start schema design

---

## Conclusion

**v0.2.1 is a solid foundation.**  
**Next milestone should expand user base (Windows) rather than deepen technology (SQLite).**

**Proposed**: Start Windows Desktop (WSL-backed) milestone.  
**Parallel**: Keep SQLite as v0.3.1 candidate.

---

**Decision Pending**: Awaiting user direction on priority.
