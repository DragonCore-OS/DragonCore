# DragonCore Runtime - Known Gaps | 已知缺陷

**Version**: 0.1.0  
**Last Updated**: 2026-03-14  
**Status**: Development Phase

This document tracks known limitations, unfinished features, and platform restrictions.

---

## 🚨 Critical Gaps | 关键缺陷

These must be resolved before any production use.

### CG-001: Real API Integration Untested
- **Description**: Model provider code exists but not tested with real API keys
- **Impact**: Cannot execute actual governance runs
- **Workaround**: None - requires real API keys
- **ETA**: Immediate (first verification priority)

### CG-002: End-to-End Workflow Unverified
- **Description**: Full run → veto → final-gate → archive cycle not exercised
- **Impact**: Unknown if state transitions work correctly
- **Workaround**: None
- **ETA**: After CG-001

### CG-003: Ledger Persistence Unverified
- **Description**: CSV ledger write logic exists but not tested
- **Impact**: No proof of audit trail functionality
- **Workaround**: Manual verification
- **ETA**: After CG-002

---

## ⚠️ Functional Gaps | 功能缺陷

### FG-001: Limited Model Provider Support
- **Status**: Hardcoded to Kimi/DeepSeek/Qwen
- **Missing**: OpenAI, Anthropic, local models (Ollama)
- **Priority**: Medium
- **ETA**: v0.2.0

### FG-002: No Streaming Responses
- **Status**: All model calls are blocking
- **Impact**: Poor UX for long-running operations
- **Priority**: Medium
- **ETA**: v0.3.0

### FG-003: No Retry Logic
- **Status**: API failures are immediate errors
- **Impact**: Transient failures kill runs
- **Priority**: High
- **ETA**: v0.2.0

### FG-004: Single-Threaded Model Calls
- **Status**: Seats execute sequentially
- **Impact**: Slow for multi-seat workflows
- **Priority**: Low (correctness before speed)
- **ETA**: v0.4.0

### FG-005: No Persistence for Run State ⭐ P0 BLOCKER
- **Status**: Run state only in memory, lost between CLI commands
- **Impact**: **ARCHITECTURE BLOCKER** - Cannot complete governance lifecycle
  - Run created by `dragoncore-runtime run` is invisible to `execute/veto/final-gate`
  - Each CLI command spawns new process with empty GovernanceEngine
  - Veto, final-gate, archive operations cannot find prior run state
  - Ledger never gets written because runs are never "finalized"
- **Root Cause**: `DragonCoreRuntime::new()` creates fresh in-memory engine every time
- **Priority**: **P0 - BLOCKS ALL GOVERNANCE CLOSURE TESTING**
- **Solution**: JSON-backed run persistence (v0.2.0) → SQLite (v0.3.0)
- **ETA**: v0.2.0 (immediate)

---

## 🖥️ Platform Limitations | 平台限制

### PL-001: Linux Primary Target
- **Tested On**: Ubuntu 22.04 LTS
- **Likely Works**: Most Linux distributions
- **Untested**: macOS, Windows, WSL
- **Priority**: Linux first, others later

### PL-002: tmux Required
- **Dependency**: tmux must be installed
- **Not Supported**: Screen, byobu alternatives
- **Priority**: Low (tmux is standard)

### PL-003: Git Required
- **Dependency**: git 2.20+ required
- **Issue**: worktree commands may vary by version
- **Priority**: Medium

---

## 🔒 Security Gaps | 安全缺陷

### SG-001: API Keys in Config File
- **Status**: Stored as plaintext in TOML
- **Risk**: Key exposure if config leaked
- **Mitigation**: File permissions 600
- **Solution**: Keyring integration (future)
- **Priority**: Medium
- **ETA**: v0.3.0

### SG-002: No Input Sanitization
- **Status**: Task strings passed directly to models
- **Risk**: Prompt injection possible
- **Mitigation**: None currently
- **Priority**: High
- **ETA**: v0.2.0

### SG-003: No Audit Log Tamper Protection
- **Status**: Ledger is plain CSV
- **Risk**: Can be manually edited
- **Mitigation**: File permissions
- **Solution**: Cryptographic signatures (future)
- **Priority**: Low (production phase)

---

## 📊 Observability Gaps | 可观测性缺陷

### OG-001: Limited Metrics
- **Status**: Only basic counters
- **Missing**: Latency histograms, token usage graphs
- **Priority**: Medium
- **ETA**: v0.3.0

### OG-002: No Structured Logging
- **Status**: Plain text logging
- **Missing**: JSON format for log aggregation
- **Priority**: Low
- **ETA**: v0.3.0

### OG-003: No Health Endpoint
- **Status**: No runtime health check
- **Impact**: Cannot monitor service status
- **Priority**: Medium
- **ETA**: v0.2.0

---

## 📚 Documentation Gaps | 文档缺陷

### DG-001: Missing API Reference
- **Status**: No generated docs for internal APIs
- **Priority**: Low
- **ETA**: v0.3.0

### DG-002: Limited Troubleshooting
- **Status**: Basic error messages
- **Missing**: Common failure scenarios
- **Priority**: Medium
- **ETA**: v0.2.0

### DG-003: No Architecture Diagram
- **Status**: Text descriptions only
- **Priority**: Low
- **ETA**: v0.3.0

---

## 🧪 Testing Gaps | 测试缺陷

### TG-001: No Unit Tests
- **Status**: Zero automated tests
- **Impact**: Refactoring is risky
- **Priority**: High
- **ETA**: v0.2.0

### TG-002: No Integration Tests
- **Status**: Only manual testing
- **Impact**: Cannot verify workflows
- **Priority**: High
- **ETA**: v0.2.0

### TG-003: No CI/CD Pipeline
- **Status**: No automated builds
- **Impact**: Manual release process
- **Priority**: Medium
- **ETA**: v0.3.0

---

## 🐛 Known Issues | 已知问题

### KI-001: Config File Path Confusion
- **Issue**: Init creates config, but runtime may look elsewhere
- **Workaround**: Always use `--config` flag
- **Priority**: High
- **ETA**: v0.1.1

### KI-002: Error Messages Not Localized
- **Issue**: All errors in English only
- **Priority**: Low
- **ETA**: Future

### KI-003: Seat Names Case Sensitive
- **Issue**: "Tianshu" works, "tianshu" may not
- **Workaround**: Use exact case
- **Priority**: Low
- **ETA**: v0.1.1

---

## 🚫 Won't Fix (By Design) | 按设计不修复

### WF-001: No Windows Native Support
- **Reason**: tmux is Unix-only
- **Alternative**: WSL2

### WF-002: No Built-in Model
- **Reason**: Keep runtime small
- **Alternative**: External providers

### WF-003: No Web UI
- **Reason**: CLI-first philosophy
- **Alternative**: tmux attach for visualization

---

## 📋 Gap Resolution Roadmap | 缺陷解决路线图

### v0.1.1 (Immediate)
- [ ] CG-001: Real API testing
- [ ] CG-002: End-to-end workflow
- [ ] KI-001: Config path fix

### v0.2.0 (Next Milestone)
- [ ] CG-003: Ledger verification
- [ ] FG-003: Retry logic
- [ ] FG-005: Run state persistence
- [ ] SG-002: Input sanitization
- [ ] TG-001: Unit tests
- [ ] TG-002: Integration tests

### v0.3.0 (Production Prep)
- [ ] FG-001: More providers
- [ ] FG-002: Streaming
- [ ] SG-001: Keyring storage
- [ ] OG-001: Better metrics
- [ ] DG-001: API docs
- [ ] TG-003: CI/CD

### v1.0.0 (Production Ready)
- [ ] All critical gaps resolved
- [ ] Security audit passed
- [ ] Load testing complete
- [ ] Documentation complete

---

## How to Contribute | 如何贡献

If you find a gap not listed here:

1. Check if it's already filed in issues
2. If not, create an issue with label `gap`
3. Reference this document

If you want to fix a gap:

1. Comment on the issue to claim it
2. Reference gap ID in commit message (e.g., "Fixes CG-001")
3. Update this document when resolved

---

*This document is a living document. Last updated: 2026-03-14*
