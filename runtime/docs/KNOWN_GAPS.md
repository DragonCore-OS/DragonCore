# DragonCore Runtime - Known Gaps & Roadmap

**Version**: v0.2.1 Complete  
**Last Updated**: 2026-03-14

## Status

**All v0.2.x blockers RESOLVED.**  
**Runtime is operationally verified for single-node deployment.**

---

## ✅ RESOLVED

### FG-005: Run State Persistence
**Status**: ✅ **RESOLVED in v0.2.0**

JSON-file backed persistence with cross-CLI state continuity.

### FG-006: Ledger Auto-Write
**Status**: ✅ **RESOLVED in v0.2.1**

Ledger now writes immediately on every state change.

### FG-007: Metrics Accuracy  
**Status**: ✅ **RESOLVED in v0.2.1**

Metrics correctly derived from ledger.

---

## 🔵 FUTURE (Post-v0.2.1)

### FG-008: SQLite Persistence
**Status**: 🔵 **v0.3.0**

Add SQLite backend option for better concurrent access.

**Current**: JSON files sufficient for single-node  
**Plan**: v0.3.0 add SQLite option

### FG-009: Worktree Auto-Cleanup
**Status**: 🔵 **v0.3.0**

Add auto-cleanup after archive + retention period.

**Current**: Worktrees preserved for forensics (design intent)  
**Plan**: v0.3.0 add configurable retention

### FG-010: Multi-Provider Support
**Status**: 🔵 **v0.4.0**

Add OpenAI, Anthropic providers.

**Current**: Kimi CLI fully functional  
**Plan**: v0.4.0 add provider abstraction

### FG-011: Distributed Consensus
**Status**: 🔵 **v0.6.0**

Raft-based multi-node support.

**Current**: Single-node only  
**Plan**: v0.6.0 distributed mode

---

## Milestone History

| Version | Status | Achievement |
|---------|--------|-------------|
| v0.1.0 | ✅ | Runtime skeleton, API integration |
| v0.2.0 | ✅ | **Persistence verified** |
| v0.2.1 | ✅ | **Ledger correctness, metrics accuracy** |
| v0.3.0 | 🔵 | SQLite, advanced isolation |
| v0.4.0 | 🔵 | Multi-provider support |
| v0.5.0 | 🔵 | Web UI, plugin system |
| v0.6.0 | 🔵 | Distributed consensus |

---

## Summary

**v0.2.1 Status**: All core functionality verified.  
**Blockers**: None.  
**Ready for**: Production use (single-node).  
**Next focus**: v0.3.0 production hardening features.
