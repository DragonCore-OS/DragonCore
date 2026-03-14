# DragonCore Desktop for Windows - Runtime Architecture

**Version**: v0.1  
**Date**: 2026-03-14  
**Status**: Design Phase

---

## Executive Summary

DragonCore Desktop for Windows is a **WSL2-backed** Windows application. The Windows side provides GUI/launcher, while the actual runtime executes inside WSL2 Linux.

**Core Principle**: Windows is the host, WSL2 is the runtime. Never run DragonCore natively on Windows.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    Windows Host (Windows 10/11)                  │
│                                                                  │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────┐ │
│  │  Launcher/GUI   │    │  Config Bridge  │    │ Log Bridge  │ │
│  │  (Tauri/Electron)│   │  (Settings UI)  │    │ (Log Viewer)│ │
│  └────────┬────────┘    └────────┬────────┘    └──────┬──────┘ │
│           │                      │                     │        │
│  ┌────────▼──────────────────────▼─────────────────────▼──────┐ │
│  │                    WSL2 Interface Layer                     │ │
│  │         (Named Pipes / Windows Sockets / WSL API)          │ │
│  └────────────────────────────┬────────────────────────────────┘ │
└───────────────────────────────┼──────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      WSL2 Linux Runtime                          │
│                     (Ubuntu 22.04 LTS)                           │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │              DragonCore Runtime v0.2.1 (Verified)          │ │
│  │                                                            │ │
│  │   ┌─────────────┐   ┌─────────────┐   ┌────────────────┐  │ │
│  │   │ Governance  │   │ JSON State  │   │  CSV Ledger    │  │ │
│  │   │   Engine    │   │  (runtime/) │   │   (ledger/)    │  │ │
│  │   └─────────────┘   └─────────────┘   └────────────────┘  │ │
│  │                                                            │ │
│  │   ┌─────────────┐   ┌─────────────┐   ┌────────────────┐  │ │
│  │   │    Tmux     │   │ Git Worktree│   │  Kimi CLI API  │  │ │
│  │   │  Sessions   │   │   (repos/)  │   │   (verified)   │  │ │
│  │   └─────────────┘   └─────────────┘   └────────────────┘  │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Definitions

### 1. Windows GUI/Launcher

**Purpose**: User-facing entry point. Abstracts WSL complexity.

**Technology**: Tauri (Rust + WebView) or Electron

**Responsibilities**:
- Detect WSL2 installation
- Install/Update DragonCore runtime in WSL
- Provide GUI for common operations
- Display logs and metrics
- Manage Windows-side shortcuts

**Does NOT**:
- Run DragonCore natively
- Replace tmux/worktree (runtime handles these)

---

### 2. WSL2 Linux Runtime

**Purpose**: Execution environment. Hosts verified v0.2.1 runtime.

**Distribution**: Ubuntu 22.04 LTS (default)

**Responsibilities**:
- Execute all DragonCore commands
- Manage tmux sessions
- Manage git worktrees
- Store runtime state (JSON + CSV)
- Communicate with Kimi API

**Invariant**: Must be v0.2.1 verified path. No modifications.

---

### 3. Config Bridge

**Purpose**: Synchronize configuration between Windows and WSL.

**Windows Side**:
- Store: `%APPDATA%\DragonCore\config\`
- User-friendly config editor
- API Key input (secure storage)

**WSL Side**:
- Store: `~/.config/dragoncore/`
- DragonCore native format

**Bridge Logic**:
```
Windows User edits config → Save to Windows store
                        → Sync to WSL via wsl cp
                        → Reload DragonCore runtime
```

---

### 4. File Bridge

**Purpose**: Allow Windows file access to WSL runtime state.

**Mapping**:
| WSL Path | Windows Path |
|----------|--------------|
| `~/.local/share/runtime/` | `%APPDATA%\DragonCore\runtime_state\` |
| `~/.local/share/runtime/worktrees/` | `%APPDATA%\DragonCore\worktrees\` |
| `~/.local/share/runtime/ledger/` | `%APPDATA%\DragonCore\ledger\` |

**Access Pattern**:
- Windows reads (display state, logs)
- WSL writes (runtime operations)
- Windows NEVER writes directly to runtime files

---

### 5. Log Bridge

**Purpose**: Stream runtime logs to Windows GUI.

**Implementation**:
```bash
# WSL side: tail logs
wsl tail -f ~/.local/share/runtime/logs/dragoncore.log

# Windows side: capture and display
```

**Log Files**:
- `dragoncore.log` - Runtime operations
- `ledger.csv` - Governance record
- `runs/*.json` - Run states

---

### 6. Lifecycle Management

**States**:
```
[Not Installed] → Install WSL → [WSL Ready]
                                     ↓
[WSL Ready] → Install Runtime → [Runtime Ready]
                                     ↓
[Runtime Ready] → Configure API → [Configured]
                                     ↓
[Configured] → Start Runtime → [Running]
                                     ↓
                                [Stop / Restart]
```

**Operations**:
- **Install**: One-click setup WSL + Ubuntu + DragonCore
- **Start**: Launch DragonCore daemon in tmux
- **Stop**: Graceful shutdown
- **Restart**: Stop + Start
- **Update**: Pull new DragonCore version

---

## Communication Protocol

### Windows → WSL Command Execution

```rust
// Windows side
let output = Command::new("wsl")
    .args(&["dragoncore", "run", "--run-id", "test"])
    .output()?;
```

### WSL → Windows State Sync

```rust
// Windows side polls WSL state
let state = Command::new("wsl")
    .args(&["cat", "~/.local/share/runtime/runs/test.json"])
    .output()?;
```

---

## Security Model

### API Key Storage
- Windows: Windows Credential Manager / DPAPI
- WSL: Environment variable (injected by Windows launcher)
- Never stored in plain text files

### File Permissions
- Windows config: User-only read
- WSL runtime: User-only read/write

---

## Error Handling

### Common Failures

| Error | Windows Handling | WSL Recovery |
|-------|-----------------|--------------|
| WSL not installed | Prompt install | N/A |
| Ubuntu not found | Auto-install | N/A |
| DragonCore not found | Auto-install | N/A |
| API key missing | Config dialog | N/A |
| Runtime crash | Restart prompt | Check logs |

---

## Performance Considerations

### Startup Time
- WSL cold start: ~2-3 seconds
- DragonCore daemon: ~1 second
- Total: ~3-4 seconds acceptable

### File Access
- Use WSL2 filesystem (not Windows mounted)
- Runtime state lives in ext4 (WSL)
- Windows reads via WSL bridge (not direct)

---

## Success Criteria

User can:
1. Install without understanding tmux/worktree
2. Enter API key via GUI (securely stored)
3. Click "Start Runtime"
4. Run verified v0.2.1 path: `run` → `veto` → `final-gate` → `metrics`
5. View ledger/logs in Windows GUI

Without ever opening WSL terminal manually.

---

## Out of Scope (v0.1)

- Native Windows runtime (no WSL)
- SQLite backend
- Multi-window management UI
- Advanced persona/seat configurator
- Performance optimization

---

**Status**: Architecture defined  
**Next**: Scope definition → Data layout → Launcher prototype
