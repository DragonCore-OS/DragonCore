# DragonCore Desktop for Windows - Data Layout

**Version**: v0.1  
**Date**: 2026-03-14  
**Status**: Design Specification

---

## Principle

**Windows stores config and GUI state. WSL stores runtime state.**

Windows **NEVER** writes directly to runtime files. WSL is the sole writer. Windows reads via bridge or file sync.

---

## Directory Structure

```
Windows Side:
%APPDATA%\DragonCore\                    (Roaming AppData)
├── config\
│   ├── dragoncore.toml                   # User configuration
│   ├── settings.json                     # GUI settings
│   └── api_key.cred                      # Encrypted API key (DPAPI)
├── cache\
│   └── state\                             # Cached runtime state (read-only mirror)
│       ├── runs\                          # JSON files from WSL
│       ├── ledger\                        # CSV files from WSL
│       └── sync.timestamp                 # Last sync time
└── temp\                                  # Temporary files

WSL Side (Ubuntu 22.04):
/home/<user>/.local/share/dragoncore/    # WSL user home
├── config\
│   └── dragoncore.toml                   # Synced from Windows
├── runtime_state\
│   ├── runs\                              # JSON state files
│   │   └── *.json
│   ├── ledger\                            # CSV ledger
│   │   └── production_ledger.csv
│   └── archive\                           # Archived runs
├── worktrees\                             # Git worktrees
│   └── <run_id>\                          # Per-run worktree
├── logs\                                  # Runtime logs
│   └── dragoncore.log
└── tmux\                                  # Tmux session info
    └── sessions/
```

---

## Windows Paths (Detailed)

### %APPDATA%\DragonCore\

**Purpose**: Windows-side configuration and GUI state.

**Access**:
- Windows: Read/Write (GUI process)
- WSL: Read-only (via /mnt/c/ path)

**Contents**:

```
%APPDATA%\DragonCore\
├── config\
│   ├── dragoncore.toml              # User-editable configuration
│   │                                # Synced TO WSL on change
│   │
│   ├── settings.json                # GUI-specific settings
│   │                                # {
│   │                                #   "window_position": [100, 100],
│   │                                #   "window_size": [1200, 800],
│   │                                #   "last_run_id": "run-001",
│   │                                #   "log_level": "info"
│   │                                # }
│   │
│   └── api_key.cred                 # Windows DPAPI encrypted
│                                      # Decrypted at runtime, injected to WSL env
│
├── cache\
│   └── state\
│       ├── runs\                     # Mirrors WSL: ~/.local/share/dragoncore/runtime_state/runs/
│       │                             # Read-only from Windows perspective
│       │                             # Updated by periodic sync or on-demand
│       │
│       ├── ledger\                   # Mirrors WSL: ~/.local/share/dragoncore/runtime_state/ledger/
│       │
│       └── sync.timestamp            # Last sync time for cache invalidation
│
└── temp\                             # Temporary files
                                      # Cleaned on exit
```

---

## WSL Paths (Detailed)

### ~/.local/share/dragoncore/

**Purpose**: Runtime state. WSL is the source of truth.

**Access**:
- WSL: Read/Write (DragonCore runtime)
- Windows: Read-only (via sync/bridge)

**Contents**:

```
~/.local/share/dragoncore/
├── config\
│   └── dragoncore.toml               # Copied from Windows on sync
│                                     # Runtime reads this
│
├── runtime_state\
│   ├── runs\
│   │   └── *.json                    # Format: {run_id}.json
│   │                                 # Example: run-20260314-001.json
│   │                                 # Content: Full PersistedRun struct
│   │
│   ├── ledger\
│   │   └── production_ledger.csv     # Format: CSV with headers
│   │                                 # Headers: run_id,timestamp,input_type,final_state,...
│   │
│   └── archive\
│       └── *.tar.gz                  # Archived worktrees
│                                     # Named: {run_id}_{timestamp}.tar.gz
│
├── worktrees\
│   └── <run_id>\                     # Git worktree for each run
│                                     # Created by: git worktree add
│                                     # Removed by: git worktree remove + rm -rf
│
├── logs\
│   └── dragoncore.log                # Runtime log output
│                                     # Rotated: dragoncore.log.1, dragoncore.log.2, etc.
│
└── tmux\
    └── sessions/                     # Tmux session metadata
                                      # Used for recovery and status checking
```

---

## Path Mapping Reference

| Purpose | Windows Path | WSL Path | Sync Direction |
|---------|-------------|----------|----------------|
| User Config | `%APPDATA%\DragonCore\config\dragoncore.toml` | `~/.local/share/dragoncore/config/dragoncore.toml` | Windows → WSL |
| API Key | `%APPDATA%\DragonCore\config\api_key.cred` | `KIMI_API_KEY` env var | Windows → WSL (on start) |
| Runtime State | `%APPDATA%\DragonCore\cache\state\runs\` | `~/.local/share/dragoncore/runtime_state/runs/` | WSL → Windows |
| Ledger | `%APPDATA%\DragonCore\cache\state\ledger\` | `~/.local/share/dragoncore/runtime_state/ledger/` | WSL → Windows |
| Worktrees | Not accessible directly | `~/.local/share/dragoncore/worktrees/` | N/A |
| Logs | `%APPDATA%\DragonCore\cache\logs\` | `~/.local/share/dragoncore/logs/` | WSL → Windows |

---

## Sync Strategy

### Config Sync (Windows → WSL)

**Trigger**: User saves settings in GUI

**Logic**:
```rust
// Windows side
fn sync_config_to_wsl() {
    let windows_path = "%APPDATA%\\DragonCore\\config\\dragoncore.toml";
    let wsl_path = "~/.local/share/dragoncore/config/dragoncore.toml";

    // Copy via WSL
    Command::new("wsl")
        .args(&["cp", windows_path, wsl_path])
        .status()?;
}
```

### State Sync (WSL → Windows)

**Trigger**: User clicks "Refresh" or auto-refresh every 5 seconds

**Logic**:
```rust
// Windows side
fn sync_state_from_wsl() {
    // Copy runs
    Command::new("wsl")
        .args(&["cp", "-r",
            "~/.local/share/dragoncore/runtime_state/runs/*",
            "%APPDATA%\\DragonCore\\cache\\state\\runs\\"])
        .status()?;

    // Copy ledger
    Command::new("wsl")
        .args(&["cp",
            "~/.local/share/dragoncore/runtime_state/ledger/production_ledger.csv",
            "%APPDATA%\\DragonCore\\cache\\state\\ledger\\"])
        .status()?;
}
```

### Log Streaming (WSL → Windows)

**Trigger**: Real-time during runtime operation

**Logic**:
```rust
// Windows side: spawn thread
fn stream_logs() {
    Command::new("wsl")
        .args(&["tail", "-f", "~/.local/share/dragoncore/logs/dragoncore.log"])
        .stdout(Stdio::piped())
        .spawn()?;
    // Read stdout and display in GUI
}
```

---

## File Permissions

### Windows

```powershell
# Config directory: User only
icacls "%APPDATA%\DragonCore\config" /grant:r "%USERNAME%:(OI)(CI)F" /inheritance:r

# Cache directory: User only
icacls "%APPDATA%\DragonCore\cache" /grant:r "%USERNAME%:(OI)(CI)F" /inheritance:r
```

### WSL

```bash
# Runtime directory: User only
chmod 700 ~/.local/share/dragoncore

# Config: User only
chmod 600 ~/.local/share/dragoncore/config/dragoncore.toml
```

---

## Size Estimates

| Directory | Typical Size | Max Size |
|-----------|-------------|----------|
| Config | 10 KB | 100 KB |
| Runtime State (JSON) | 100 KB per 100 runs | 10 MB (10k runs) |
| Ledger (CSV) | 50 KB per 100 runs | 5 MB (10k runs) |
| Worktrees | 10 MB per run | 100 GB (with cleanup) |
| Logs | 10 MB | 100 MB (with rotation) |
| **Total** | **~50 MB** | **~200 GB** (with worktrees) |

---

## Cleanup Strategy

### Windows Cache
- Auto-clean on uninstall
- Manual "Clear Cache" button in settings
- Never delete runtime state (only cache copies)

### WSL Runtime
- Archive old runs (configurable retention)
- Log rotation (keep last 10 files)
- Worktree cleanup after archive

---

## Summary

| Aspect | Windows | WSL |
|--------|---------|-----|
| **Role** | GUI, Config, Cache | Runtime, State, Execution |
| **Writes** | Config only | Everything else |
| **Reads** | Everything (via sync) | Config only |
| **Persistence** | User settings | Runtime truth |
| **Sync** | Push config | Push state/logs |

**Golden Rule**: WSL is the source of truth for runtime. Windows is the source of truth for user preferences.

---

**Status**: Specification complete  
**Next**: Launcher prototype implementation
