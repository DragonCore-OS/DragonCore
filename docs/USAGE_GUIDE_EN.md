# DragonCore Usage Guide

**Purpose**: Complete guide to installing, configuring, and operating DragonCore Runtime for governance-first multi-agent AI systems.

**Current Status**: v0.2.1 verified - Single-node JSON-backed path operational

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Basic Usage](#basic-usage)
5. [Governance Workflows](#governance-workflows)
6. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Prerequisites

DragonCore Runtime requires Linux environment (native or WSL2):

```bash
# Ubuntu/Debian
sudo apt-get install -y tmux git rustc cargo

# Verify installations
tmux -V
git --version
cargo --version
```

### One-Command Start

```bash
# Clone repository
git clone https://github.com/DragonCore-OS/DragonCore.git
cd DragonCore/runtime

# Build release binary
cargo build --release

# Set API key and run
export KIMI_API_KEY="your-api-key"
./target/release/dragoncore-runtime run --task "Implement user authentication"
```

---

## Installation

### From Source (Recommended)

```bash
# 1. Clone repository
git clone https://github.com/DragonCore-OS/DragonCore.git
cd DragonCore/runtime

# 2. Build release binary
cargo build --release

# 3. Verify binary
./target/release/dragoncore-runtime --version
# Output: dragoncore-runtime 0.2.1

# 4. (Optional) System-wide install
sudo cp target/release/dragoncore-runtime /usr/local/bin/
sudo chmod +x /usr/local/bin/dragoncore-runtime
dragoncore-runtime --help
```

### Build Requirements

| Component | Version | Purpose |
|-----------|---------|---------|
| Rust | 1.75+ | Runtime compilation |
| tmux | 3.0+ | Process isolation |
| Git | 2.30+ | Worktree management |

---

## Configuration

### Initial Setup

```bash
# Initialize configuration directory
dragoncore-runtime init --output ~/.config/dragoncore

# Creates directory structure:
# ~/.config/dragoncore/
# ├── dragoncore.toml          # Main configuration
# └── runtime_state/
#     ├── runs/                # JSON state files
#     ├── ledger/              # CSV production ledger
#     └── worktrees/           # Git worktrees
```

### API Key Configuration

**Method 1: Environment Variables (Recommended)**

```bash
export KIMI_API_KEY="sk-your-kimi-api-key"
```

**Method 2: Config File**

Edit `~/.config/dragoncore/dragoncore.toml`:

```toml
[[providers]]
name = "kimi-cli"
provider_type = "KimiCli"
api_key = "sk-your-key"
model = "kimi-k2"
timeout = 120
```

### Configuration Reference

```toml
[runtime]
name = "dragoncore"
version = "0.1.0"
data_dir = "~/.config/dragoncore/runtime_state"
log_level = "info"

[governance]
constitution_path = "~/.config/dragoncore/constitution"
escalation_timeout = 300
strict_mode = true

[execution]
tmux_prefix = "dragoncore"
worktree_base = "~/.config/dragoncore/runtime_state/worktrees"
max_concurrent_agents = 19
isolation_enabled = true

[ledger]
storage_path = "~/.config/dragoncore/runtime_state/ledger"
auto_archive_threshold = 100
retention_days = 365
```

---

## Basic Usage

### Start a Governance Run

```bash
# Basic run with auto-generated ID
dragoncore-runtime run --task "Implement feature X"

# With specific input type
dragoncore-runtime run \
  --input-type "security_review" \
  --task "Review authentication module"

# With custom run ID
dragoncore-runtime run \
  --run-id "RUN-AUTH-001" \
  --task "Implement OAuth2"
```

### Execute a Seat's Role

```bash
# Tianquan (CSO) - creates execution plan
dragoncore-runtime execute \
  --run-id RUN-20240314_120000-a1b2c3d4 \
  --seat "Tianquan" \
  --task "Create execution plan for OAuth2"

# Yuheng (CRO) - quality review
dragoncore-runtime execute \
  --run-id RUN-20240314_120000-a1b2c3d4 \
  --seat "Yuheng" \
  --task "Review code quality and security"
```

**Note**: Seats can be referenced by English name or Chinese name (e.g., "Tianquan" or "天权").

### Exercise Veto

```bash
dragoncore-runtime veto \
  --run-id RUN-20240314_120000-a1b2c3d4 \
  --seat "Yuheng" \
  --reason "Critical security vulnerability: Missing input validation"
```

**Seats with Veto Authority**:
- Tianxuan (天璇) - Risk guardian
- Yuheng (玉衡) - Quality gate
- Baozheng (包拯) - Independent audit
- Baihu (白虎) - Red team

### Final Gate (Tianshu Only)

```bash
# Approve
dragoncore-runtime final-gate \
  --run-id RUN-20240314_120000-a1b2c3d4 \
  --approve

# Reject
dragoncore-runtime final-gate \
  --run-id RUN-20240314_120000-a1b2c3d4 \
  --approve false
```

---

## Governance Workflows

### Workflow 1: Standard Feature Implementation

```bash
# 1. Initialize run
RUN_ID=$(dragoncore-runtime run --task "Implement feature X" | grep "Started governance run" | awk '{print $4}')

# 2. Strategy planning (Tianquan - CSO)
dragoncore-runtime execute --run-id $RUN_ID --seat Tianquan --task "Create implementation plan"

# 3. Risk assessment (Tianxuan - Risk)
dragoncore-runtime execute --run-id $RUN_ID --seat Tianxuan --task "Assess security risks"

# 4. Code review (Kaiyang - Review)
dragoncore-runtime execute --run-id $RUN_ID --seat Kaiyang --task "Review implementation"

# 5. Quality gate (Yuheng - CRO)
dragoncore-runtime execute --run-id $RUN_ID --seat Yuheng --task "Final quality check"

# 6. Final approval (Tianshu - Final Arbiter)
dragoncore-runtime final-gate --run-id $RUN_ID --approve

# 7. Archive (Yaoguang - Archivist)
dragoncore-runtime archive --run-id $RUN_ID --seat Yaoguang
```

### Workflow 2: Veto and Resolution

```bash
# When veto is exercised during quality gate
dragoncore-runtime veto \
  --run-id $RUN_ID \
  --seat Yuheng \
  --reason "Missing input validation - SQL injection risk"

# Fix the issue...

# Re-review after fix
dragoncore-runtime execute --run-id $RUN_ID --seat Kaiyang --task "Verify fix implemented"
dragoncore-runtime execute --run-id $RUN_ID --seat Yuheng --task "Re-review after fix"

# Continue to final gate
dragoncore-runtime final-gate --run-id $RUN_ID --approve
```

### Workflow 3: Emergency Termination

```bash
# If critical issue detected
dragoncore-runtime terminate \
  --run-id $RUN_ID \
  --seat Zhongkui \
  --reason "Malicious code detected in commit abc123"

# Status: TERMINATED
# All resources cleaned up automatically
```

---

## State Verification

### Cross-CLI State Continuity (v0.2.1 Verified)

DragonCore v0.2.1 guarantees state persistence across CLI invocations:

```bash
# CLI Process 1: Create
dragoncore-runtime run --run-id persistence-test --task "Test state"

# CLI Process 2: Different shell/instance
dragoncore-runtime veto --run-id persistence-test --seat Yuheng --reason "test"
# Output: [INFO] Loaded 1 runs from persistent storage

# CLI Process 3: Another instance
dragoncore-runtime final-gate --run-id persistence-test --approve

# Verify state persisted
dragoncore-runtime metrics
# Output: Total runs: 1
```

### View Ledger

```bash
# Command line
cat ~/.config/dragoncore/runtime_state/ledger/production_ledger.csv

# Structured output
dragoncore-runtime status
```

### View Metrics

```bash
dragoncore-runtime metrics

# Expected output:
# DragonCore Stability Metrics
# ============================
# Total runs: 42
# Clean runs: 40
# Authority violations: 0
# Fake closures: 0
# Rollbacks: 1
# Terminations: 1
```

---

## Troubleshooting

### Issue: tmux not found

```bash
# Ubuntu/Debian
sudo apt-get install tmux

# Verify
tmux -V
```

### Issue: API key not working

```bash
# Check if key is set
echo $KIMI_API_KEY

# Test API directly
curl https://api.moonshot.cn/v1/models \
  -H "Authorization: Bearer $KIMI_API_KEY"
```

### Issue: Worktree already exists

```bash
# Clean up specific worktree
dragoncore-runtime cleanup

# Or manually remove (if cleanup fails)
rm -rf ~/.config/dragoncore/runtime_state/worktrees/RUN-xxx
```

### Debug Mode

```bash
# Enable debug logging
export RUST_LOG=debug
dragoncore-runtime run --task "Test"

# Or use --log-level flag
dragoncore-runtime --log-level debug run --task "Test"
```

---

## Operational Notes

### Best Practices

1. **Always Archive Completed Runs**
   ```bash
   dragoncore-runtime archive --run-id RUN-xxx --seat Yaoguang
   ```

2. **Document Veto Reasons**
   ```bash
   dragoncore-runtime veto --run-id RUN-xxx --seat Yuheng \
     --reason "Line 42: Missing null check. See CWE-476."
   ```

3. **Use Meaningful Run IDs**
   ```bash
   # Good
   dragoncore-runtime run --run-id "RUN-AUTH-OAUTH2-001"
   
   # Avoid
   dragoncore-runtime run --run-id "test1"
   ```

### File Locations

| Purpose | Path |
|---------|------|
| Runtime binary | `target/release/dragoncore-runtime` |
| Configuration | `~/.config/dragoncore/dragoncore.toml` |
| Run states | `~/.config/dragoncore/runtime_state/runs/*.json` |
| Ledger | `~/.config/dragoncore/runtime_state/ledger/production_ledger.csv` |
| Worktrees | `~/.config/dragoncore/runtime_state/worktrees/<run_id>/` |
| Logs | `~/.config/dragoncore/runtime_state/logs/dragoncore.log` |

---

## Open Issues / Next Steps

### Current Limitations

- Single-node only (multi-node planned for v0.6.0)
- Linux/WSL environment required (Windows Desktop planned)
- JSON persistence (SQLite backend planned for v0.3.0)

### Further Reading

- [19_SEATS.md](19_SEATS_EN.md) - Complete seat authority definitions
- [VERIFICATION_REPORT.md](../runtime/docs/VERIFICATION_REPORT.md) - Verification evidence
- [STATUS.md](../runtime/STATUS.md) - Current project status

---

**Operational Status**: v0.2.1 verified - Single-node JSON-backed path ready for use

**True Dragon. Not Claw.**
