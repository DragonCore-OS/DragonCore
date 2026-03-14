# DragonCore Desktop for Windows - v0.1 Scope

**Version**: v0.1  
**Date**: 2026-03-14  
**Status**: Scope Frozen

---

## Scope Statement

**IN SCOPE**: Minimum viable Windows desktop experience for DragonCore Runtime v0.2.1.

**OUT OF SCOPE**: Everything else. See "Explicitly Excluded" section.

---

## In Scope (v0.1)

### 1. Installation

**User Story**: As a Windows user, I can install DragonCore Desktop without manual WSL setup.

**Features**:
- [ ] Detect WSL2 installation
- [ ] Install WSL2 if missing (with user consent)
- [ ] Install Ubuntu 22.04 LTS distribution
- [ ] Install DragonCore Runtime v0.2.1 in WSL
- [ ] Create Windows shortcuts (Start Menu, Desktop optional)

**Success Criteria**: Fresh Windows machine → Install → Runtime ready in < 5 minutes.

### 2. First-Time Initialization

**User Story**: As a first-time user, I am guided through initial setup.

**Features**:
- [ ] Welcome dialog
- [ ] WSL status check
- [ ] Runtime status check
- [ ] Working directory initialization
- [ ] Configuration file creation

**Success Criteria**: User clicks through setup wizard, no terminal required.

### 3. API Key Configuration

**User Story**: As a user, I can securely configure my Kimi API key.

**Features**:
- [ ] API Key input dialog (masked)
- [ ] Secure storage (Windows Credential Manager / DPAPI)
- [ ] Validation (test API call)
- [ ] Sync to WSL environment variable

**Success Criteria**: API key entered once, persisted securely, runtime can use it.

### 4. Start Runtime

**User Story**: As a user, I can start DragonCore runtime with one click.

**Features**:
- [ ] "Start Runtime" button
- [ ] Launch DragonCore daemon in WSL tmux
- [ ] Show runtime status (running/stopped)
- [ ] Display real-time logs

**Success Criteria**: Click button → Runtime starts → Logs appear → Status shows "Running".

### 5. View Ledger

**User Story**: As a user, I can view governance ledger without opening files manually.

**Features**:
- [ ] "View Ledger" button/menu
- [ ] Display `production_ledger.csv` in table format
- [ ] Sort by timestamp, run_id, final_state
- [ ] Basic filtering (by run_id, by date range)

**Success Criteria**: User can see all runs with their final states, veto usage, etc.

### 6. View Logs

**User Story**: As a user, I can view runtime logs in real-time.

**Features**:
- [ ] Log viewer panel
- [ ] Real-time tail (auto-scroll)
- [ ] Log level filtering (INFO, WARN, ERROR)
- [ ] Search within logs
- [ ] Export logs to file

**Success Criteria**: Runtime operations appear in log viewer within 1 second.

### 7. Stop / Restart Runtime

**User Story**: As a user, I can stop and restart runtime gracefully.

**Features**:
- [ ] "Stop Runtime" button
- [ ] Graceful shutdown (wait for current operation)
- [ ] "Restart Runtime" button (stop + start)
- [ ] Force kill option (if graceful fails)

**Success Criteria**: Stop → Status shows "Stopped" → Start → Status shows "Running".

---

## User Flow (v0.1)

```
[Install DragonCore Desktop]
           ↓
[First Launch]
           ↓
    ┌──────┴──────┐
    ↓             ↓
[WSL Check]  [Already OK]
    ↓             ↓
[Install WSL]   [Initialize]
    ↓             ↓
    └──────┬──────┘
           ↓
[API Key Setup]
           ↓
[Test Connection]
           ↓
    ┌──────┴──────┐
    ↓             ↓
[Success]    [Retry/Help]
    ↓
[Main Window]
    ↓
┌───┴───┬─────────┬────────┐
↓       ↓         ↓        ↓
Start  View     View    Stop/
Runtime Ledger  Logs    Restart
```

---

## Explicitly Excluded (v0.1)

### ❌ Native Windows Runtime
DragonCore runs **ONLY** in WSL2. No port to Windows native.

### ❌ SQLite Backend
Use existing JSON/CSV persistence. SQLite is v0.3.0+, not v0.1.

### ❌ Full Multi-Window Management UI
No drag-and-drop seat configuration. No visual governance graph editor. No real-time seat activity dashboard. Basic buttons and lists only.

### ❌ Advanced Persona/Seat Configurator
No custom seat creation. No authority modification UI. Use default 19-seat configuration.

### ❌ Performance Optimization
No caching layer. No connection pooling. No background sync optimization. Functional correctness over speed.

### ❌ macOS Support
Windows only for v0.1. macOS is future milestone.

### ❌ Web Interface
Desktop app only. No browser-based UI.

### ❌ Plugin System
No WASM plugins. No external seat loading.

---

## UI Mockup (Textual)

```
┌─────────────────────────────────────────┐
│  DragonCore Desktop          [─] [□] [X]│
├─────────────────────────────────────────┤
│                                         │
│  Status: ● Running                      │
│                                         │
│  [Start Runtime] [Stop Runtime]         │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│  Recent Runs:                           │
│  ┌─────────────────────────────────┐   │
│  │ Run ID    │ Status   │ Time     │   │
│  │───────────│──────────│──────────│   │
│  │ run-001   │ Approved │ 2m ago   │   │
│  │ run-002   │ Archived │ 1h ago   │   │
│  └─────────────────────────────────┘   │
│                                         │
│  [View Ledger] [View Logs] [Settings]   │
│                                         │
└─────────────────────────────────────────┘
```

---

## Success Definition

**v0.1 is successful when**:

A Windows user can:
1. Install DragonCore Desktop (one installer, no manual WSL setup)
2. Enter API key via GUI (securely stored)
3. Click "Start Runtime" and see it running
4. Execute verified v0.2.1 path:
   ```
   dragoncore run --run-id test --input-type code -t "Hello"
   dragoncore veto --run-id test --seat Yuheng --reason "test"
   dragoncore final-gate --run-id test --approve
   dragoncore metrics
   ```
5. View ledger and logs in Windows GUI
6. Stop and restart runtime

**Without**:
- Opening PowerShell/CMD
- Understanding tmux
- Understanding git worktrees
- Editing configuration files manually

---

## Deliverables

| # | Deliverable | Format |
|---|-------------|--------|
| 1 | Windows Installer | MSI or MSIX |
| 2 | Desktop Application | Tauri or Electron executable |
| 3 | User Documentation | Markdown + In-app help |
| 4 | Test Report | Verified on clean Windows 10/11 VM |

---

## Timeline Estimate

| Phase | Duration | Deliverable |
|-------|----------|-------------|
| Architecture | 1 day | This document |
| Data Layout | 1 day | WINDOWS_DATA_LAYOUT.md |
| Launcher Prototype | 3-4 days | Working installer + basic GUI |
| Integration | 2-3 days | Full v0.2.1 path integration |
| Polish | 2 days | UI polish, error handling |
| Testing | 2 days | Clean VM testing |
| **Total** | **~2 weeks** | v0.1 Release |

---

## Risks

| Risk | Mitigation |
|------|-----------|
| WSL2 not available (Home S mode) | Document requirement, fail gracefully |
| WSL2 startup slow | Show progress, cache warm state |
| File permission issues | Test with standard user (non-admin) |
| API key sync failure | Retry logic, manual override option |

---

**Scope Status**: Frozen for v0.1  
**Changes**: Require explicit approval, must not break success criteria
