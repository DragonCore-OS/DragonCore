# DragonCore Desktop for Windows - Runtime Architecture

**Version**: v0.1  
**Date**: 2026-03-14  
**Status**: Design Specification

---

## Purpose

Define the host architecture for DragonCore Desktop on Windows — a WSL2-backed Windows application where Windows provides the GUI/launcher and WSL2 hosts the actual runtime.

**Core Principle**: Windows is the host, WSL2 is the runtime. DragonCore never runs natively on Windows.

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

**Explicitly Does NOT**:
- Run DragonCore natively on Windows
- Replace tmux/worktree (runtime handles these internally)

### 2. WSL2 Linux Runtime

**Purpose**: Execution environment. Hosts verified v0.2.1 runtime.

**Distribution**: Ubuntu 22.04 LTS (default)

**Responsibilities**:
- Execute all DragonCore commands
- Manage tmux sessions
- Manage git worktrees
- Store runtime state (JSON + CSV)
- Communicate with Kimi API

**Invariant**: Must be v0.2.1 verified path. No modifications to core runtime.

### 3. Config Bridge

**Purpose**: Synchronize configuration between Windows and WSL.

**Windows Side**:
- Location: `%APPDATA%\DragonCore\config\`
- User-friendly config editor
- API Key input (secure storage via DPAPI)

**WSL Side**:
- Location: `~/.config/dragoncore/`
- DragonCore native format

**Sync Logic**:
```
Windows User edits config → Save to Windows store
                        → Sync to WSL via wsl cp
                        → Reload DragonCore runtime
```

### 4. File Bridge

**Purpose**: Allow Windows file access to WSL runtime state.

**Critical Rule**: Windows NEVER writes directly to runtime files.

| WSL Path (Truth) | Windows Path (Read-Only Cache) |
|------------------|-------------------------------|
| `~/.local/share/runtime/` | `%APPDATA%\DragonCore\runtime_state\` |
| `~/.local/share/runtime/worktrees/` | `%APPDATA%\DragonCore\worktrees\` |
| `~/.local/share/runtime/ledger/` | `%APPDATA%\DragonCore\ledger\` |

**Access Pattern**:
- **WSL**: Read/Write (runtime operations)
- **Windows**: Read-only (display state, logs)
- **Sync**: WSL → Windows (one-way for state)

### 5. Log Bridge

**Purpose**: Stream runtime logs to Windows GUI in real-time.

**Implementation**:
```bash
# WSL side: tail logs continuously
wsl tail -f ~/.local/share/runtime/logs/dragoncore.log

# Windows side: capture and display in GUI
```

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
- **Windows**: Windows Credential Manager / DPAPI encrypted
- **WSL**: Environment variable (injected by Windows launcher on startup)
- **Rule**: Never stored in plain text files

### File Permissions
- **Windows config**: User-only read (ACL)
- **WSL runtime**: User-only read/write (chmod 700)

---

## Success Criteria

User can:
1. Install without understanding tmux/worktree
2. Enter API key via GUI (securely stored)
3. Click "Start Runtime" and see it running
4. Run verified v0.2.1 path: `run` → `veto` → `final-gate` → `metrics`
5. View ledger/logs in Windows GUI

**Without ever opening WSL terminal manually.**

---

## Explicitly Excluded (v0.1)

- ❌ Native Windows runtime (no WSL)
- ❌ SQLite backend
- ❌ Multi-window management UI
- ❌ Advanced persona/seat configurator
- ❌ Performance optimization

---

**Status**: Architecture defined  
**Next**: Data layout → Launcher prototype
