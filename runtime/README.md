# DragonCore Runtime | 龙核运行时

DragonCore Runtime is a **complete, clean-room implementation** of a governance-first multi-agent AI operating system.

**This is NOT a fork of zeroclaw.** It is a from-scratch implementation inspired by the need for true governance in multi-agent systems.

---

## Architecture | 架构

```
dragoncore-runtime/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── config/          # Configuration management
│   ├── governance/      # 19-seat governance protocol (19席治理协议)
│   ├── ledger/          # Production ledger system (生产账本系统)
│   ├── models/          # Model provider interfaces (Kimi/DeepSeek/Qwen)
│   ├── runtime/         # Core runtime engine (运行时核心)
│   ├── tmux/            # Process isolation (进程隔离)
│   └── worktree/        # Git worktree execution (执行环境)
├── Cargo.toml
└── launch.sh            # Launch script
```

---

## Key Features | 核心特性

### 1. 19-Seat Governance Protocol (19席治理协议)

Implements the complete DragonCore governance structure:

- **北斗七星 (Seven Northern Stars)**: Core governance (CEO/CTO/COO/CRO with separated powers)
- **四象 (Four Symbols)**: Campaign layer (exploration, verification, narrative, stability)
- **八仙护法 (Eight Guardians)**: Specialized functions (audit, quality, deployment, termination)

Each seat has explicit authority boundaries:
- `suggest` - Can recommend
- `review` - Can examine
- `veto` - Can block
- `approve` - Can authorize
- `execute` - Can implement
- `final_gate` - Ultimate decision (Tianshu only)
- `archive` - Can preserve
- `terminate` - Can end permanently

### 2. Process Isolation (进程隔离)

Uses **tmux** for true multi-agent concurrency:
- Each seat runs in its own tmux window
- True parallelism (not asyncio pseudo-concurrency)
- Human operators can attach and observe in real-time
- Fault isolation: one crashing agent doesn't affect others

### 3. Git Worktree Execution

Each governance run executes in an isolated Git worktree:
- Clean, reproducible execution environment
- No state pollution between runs
- Full git history available for reference
- Supports parallel runs

### 4. Production Ledger

Every governance action is recorded:
- Run ID, timestamp, final state
- Veto usage, escalation, rollback
- Authority violations, fake closures
- Token usage, wall-clock time
- Human intervention tracking

---

## Quick Start | 快速开始

### Prerequisites

- Rust 1.75+
- tmux
- git
- API keys for at least one provider (Kimi/DeepSeek/Qwen)

### Build

```bash
cd runtime
./launch.sh build
```

### Run

```bash
# Set API keys
export KIMI_API_KEY="your-key-here"

# Start a governance run
./launch.sh run "Implement a new feature"

# Check status
./launch.sh status

# View metrics
./launch.sh metrics

# List all 19 seats
./launch.sh seats
```

---

## CLI Commands | 命令行

```bash
# Initialize configuration
dragoncore-runtime init

# Start a governance run
dragoncore-runtime run --task "Implement feature X"

# Execute a specific seat
dragoncore-runtime execute --run-id RUN-001 --seat "Tianquan" --task "Review code"

# Exercise veto
dragoncore-runtime veto --run-id RUN-001 --seat "Yuheng" --reason "Quality issues"

# Execute final gate (Tianshu only)
dragoncore-runtime final-gate --run-id RUN-001 --approve

# Archive a run
dragoncore-runtime archive --run-id RUN-001 --seat "Yaoguang"

# Terminate a run
dragoncore-runtime terminate --run-id RUN-001 --seat "Fengdudadi" --reason "Risk too high"

# Show status
dragoncore-runtime status
dragoncore-runtime status --run-id RUN-001

# Show metrics
dragoncore-runtime metrics

# Attach to tmux session
dragoncore-runtime attach --run-id RUN-001

# List all seats
dragoncore-runtime seats

# Clean up
dragoncore-runtime cleanup
```

---

## Configuration | 配置

Configuration file: `dragoncore.toml`

```toml
[runtime]
name = "dragoncore"
version = "0.1.0"
data_dir = "./data"
log_level = "info"

[governance]
constitution_path = "./data/constitution"
escalation_timeout = 300
strict_mode = true

[execution]
tmux_prefix = "dragoncore"
worktree_base = "./data/worktrees"
max_concurrent_agents = 19
isolation_enabled = true

[ledger]
storage_path = "./data/ledger"
auto_archive_threshold = 100
retention_days = 365

[[providers]]
name = "kimi"
provider_type = "kimi"
api_key = "your-api-key"
base_url = "https://api.moonshot.cn/v1"
model = "kimi-latest"
timeout = 60
```

---

## Comparison with OpenClaw | 与 OpenClaw 对比

| Aspect | DragonCore | OpenClaw |
|--------|------------|----------|
| **Language** | Rust (from scratch) | Rust |
| **Binary Size** | ~5 MB | ~5 MB |
| **Governance** | 19-seat formal protocol | Flat tool-oriented |
| **Veto** | Multi-level chain | Limited |
| **Archive** | Complete ledger | None |
| **Process Isolation** | tmux (true parallel) | Single process |
| **Execution** | Git worktree | Manual |

---

## Project Status | 项目状态

🟢 **Controlled Production** — Core runtime implemented, ready for testing

- ✅ 19-seat governance protocol
- ✅ Process isolation (tmux)
- ✅ Git worktree execution
- ✅ Production ledger
- ✅ Model provider interfaces (Kimi/DeepSeek/Qwen)
- ✅ CLI interface

---

## License | 许可证

MIT — We open source the governance framework.
The Huaxia civilizational metaphor is ours.

**真龙，不是龙虾。**
**True Dragon. Not Claw.**
