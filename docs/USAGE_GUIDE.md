# DragonCore Usage Guide | 龙核使用指南

Complete guide to using DragonCore Runtime for governance-first multi-agent AI operations.

---

## Table of Contents | 目录

1. [Quick Start | 快速开始](#quick-start)
2. [Installation | 安装](#installation)
3. [Configuration | 配置](#configuration)
4. [Basic Usage | 基本使用](#basic-usage)
5. [Governance Workflows | 治理工作流](#governance-workflows)
6. [Advanced Features | 高级功能](#advanced-features)
7. [Troubleshooting | 故障排除](#troubleshooting)

---

## Quick Start | 快速开始

### Prerequisites | 前置条件

```bash
# Ubuntu/Debian
sudo apt-get install -y tmux git rustc cargo

# macOS
brew install tmux git rust

# Verify installations
tmux -V
git --version
cargo --version
```

### One-Command Start | 一键启动

```bash
# Clone repository
git clone https://github.com/DragonCore-OS/DragonCore.git
cd DragonCore

# Build and run
./runtime/launch.sh build
export KIMI_API_KEY="your-api-key"
./runtime/launch.sh run "Implement user authentication"
```

---

## Installation | 安装

### From Source | 从源码安装

```bash
# 1. Clone repository
git clone https://github.com/DragonCore-OS/DragonCore.git
cd DragonCore/runtime

# 2. Build release binary
cargo build --release

# 3. Verify binary
./target/release/dragoncore-runtime --version

# 4. Run tests
./examples/test_governance.sh
```

### System-Wide Installation | 系统范围安装

```bash
# Install to /usr/local/bin
sudo cp target/release/dragoncore-runtime /usr/local/bin/
sudo chmod +x /usr/local/bin/dragoncore-runtime

# Now you can use from anywhere
dragoncore-runtime --help
```

---

## Configuration | 配置

### Initial Setup | 初始设置

```bash
# Initialize configuration
dragoncore-runtime init --output ~/.config/dragoncore

# This creates:
# ~/.config/dragoncore/dragoncore.toml
# ~/.config/dragoncore/data/
# ~/.config/dragoncore/data/ledger/
# ~/.config/dragoncore/data/worktrees/
```

### API Keys | API密钥

Set at least one provider API key:

```bash
# Option 1: Environment variables (recommended)
export KIMI_API_KEY="sk-your-kimi-key"
export DEEPSEEK_API_KEY="sk-your-deepseek-key"
export QWEN_API_KEY="sk-your-qwen-key"

# Option 2: Config file (~/.config/dragoncore/dragoncore.toml)
[[providers]]
name = "kimi"
provider_type = "kimi"
api_key = "sk-your-key"
base_url = "https://api.moonshot.cn/v1"
model = "kimi-latest"
timeout = 60
```

### Configuration File Reference | 配置文件参考

```toml
[runtime]
name = "dragoncore"
version = "0.1.0"
data_dir = "~/.config/dragoncore/data"
log_level = "info"  # trace, debug, info, warn, error

[governance]
constitution_path = "~/.config/dragoncore/data/constitution"
escalation_timeout = 300  # seconds
strict_mode = true  # Enforce all governance rules

[execution]
tmux_prefix = "dragoncore"
worktree_base = "~/.config/dragoncore/data/worktrees"
max_concurrent_agents = 19
isolation_enabled = true

[ledger]
storage_path = "~/.config/dragoncore/data/ledger"
auto_archive_threshold = 100
retention_days = 365
```

---

## Basic Usage | 基本使用

### Start a Governance Run | 开始治理运行

```bash
# Basic run
dragoncore-runtime run --task "Implement feature X"

# With specific input type
dragoncore-runtime run \
  --input-type "security_review" \
  --task "Review authentication module for vulnerabilities"

# With custom run ID
dragoncore-runtime run \
  --run-id "RUN-AUTH-001" \
  --task "Implement OAuth2"
```

### Execute a Seat's Role | 执行席位角色

```bash
# Tianquan (CSO) creates execution plan
dragoncore-runtime execute \
  --run-id RUN-20240314_120000-a1b2c3d4 \
  --seat "Tianquan" \
  --task "Create execution plan for OAuth2 implementation"

# Yuheng (CRO) reviews quality
dragoncore-runtime execute \
  --run-id RUN-20240314_120000-a1b2c3d4 \
  --seat "Yuheng" \
  --task "Review code quality and security controls"

# Using Chinese names
dragoncore-runtime execute \
  --run-id RUN-20240314_120000-a1b2c3d4 \
  --seat "天权" \
  --task "创建OAuth2实现计划"
```

### Exercise Veto | 行使否决

```bash
dragoncore-runtime veto \
  --run-id RUN-20240314_120000-a1b2c3d4 \
  --seat "Yuheng" \
  --reason "Critical security vulnerability: Missing input validation"
```

**Seats with Veto Authority | 有否决权的席位**:
- Tianxuan (天璇) - Risk guardian
- Yuheng (玉衡) - Quality gate
- Baozheng (包拯) - Independent audit
- Baihu (白虎) - Red team

### Final Gate | 终局裁决

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

**Only Tianshu (天枢) can execute final gate.**

---

## Governance Workflows | 治理工作流

### Workflow 1: Standard Feature Implementation | 标准功能实现

```bash
# 1. Initialize
dragoncore-runtime run --task "Implement feature X"
# Returns: RUN-20240314_120000-a1b2c3d4

# 2. Strategy (Tianquan)
dragoncore-runtime execute --run-id RUN-xxx --seat Tianquan --task "Create plan"

# 3. Exploration (Qinglong)
dragoncore-runtime execute --run-id RUN-xxx --seat Qinglong --task "Explore options"

# 4. Risk Assessment (Tianxuan)
dragoncore-runtime execute --run-id RUN-xxx --seat Tianxuan --task "Assess risks"

# 5. Implementation Review (Kaiyang)
dragoncore-runtime execute --run-id RUN-xxx --seat Kaiyang --task "Review code"

# 6. Quality Gate (Yuheng)
dragoncore-runtime execute --run-id RUN-xxx --seat Yuheng --task "Quality check"

# 7. Final Approval (Tianshu)
dragoncore-runtime final-gate --run-id RUN-xxx --approve

# 8. Deployment (Nezha)
dragoncore-runtime execute --run-id RUN-xxx --seat Nezha --task "Deploy"

# 9. Archive (Yaoguang)
dragoncore-runtime archive --run-id RUN-xxx --seat Yaoguang
```

### Workflow 2: Veto and Resolution | 否决与解决

```bash
# ... steps 1-5 from Workflow 1 ...

# 6. Quality Gate - VETO
# Yuheng detects security issue
dragoncore-runtime veto \
  --run-id RUN-xxx \
  --seat Yuheng \
  --reason "Missing input validation - SQL injection risk"

# Run status: REJECTED

# 7. Fix issue (engineering)
# ... fix the code ...

# 8. Re-review (Kaiyang)
dragoncore-runtime execute --run-id RUN-xxx --seat Kaiyang --task "Verify fix"

# 9. Re-review (Yuheng)
dragoncore-runtime execute --run-id RUN-xxx --seat Yuheng --task "Re-review after fix"

# 10. Continue with Workflow 1 step 7...
```

### Workflow 3: Emergency Termination | 紧急终止

```bash
# If critical issue detected at any point:
dragoncore-runtime terminate \
  --run-id RUN-xxx \
  --seat Zhongkui \
  --reason "Malicious code detected in commit abc123"

# Run status: TERMINATED
# All resources cleaned up
```

---

## Advanced Features | 高级功能

### Real-Time Monitoring | 实时监控

```bash
# Attach to tmux session to watch all 19 seats
dragoncore-runtime attach --run-id RUN-20240314_120000-a1b2c3d4

# In tmux:
# - Ctrl+B N: Next window (next seat)
# - Ctrl+B P: Previous window (previous seat)
# - Ctrl+B D: Detach (return to shell)
```

### Parallel Runs | 并行运行

```bash
# Run multiple governance runs simultaneously
# Each runs in its own tmux session and git worktree

dragoncore-runtime run --run-id RUN-A --task "Feature A" &
dragoncore-runtime run --run-id RUN-B --task "Feature B" &
dragoncore-runtime run --run-id RUN-C --task "Feature C" &

wait
```

### Custom Seat Prompts | 自定义席位提示

Edit `~/.config/dragoncore/data/constitution/<seat_id>.yaml`:

```yaml
seat_id: tianshu
seat_name: 天枢
role_boundary: >
  Custom role description for your organization's needs.
  Maintain the core authority but adapt the specifics.

system_prompt: >
  You are Tianshu (天枢), the final arbiter of the DragonCore governance system.
  
  Your specific responsibilities:
  - Make final decisions on all escalated conflicts
  - Ensure governance principles are upheld
  - Approve or reject runs based on collective input
  
  Tone: Authoritative, fair, decisive
```

### Ledger Queries | 账本查询

```bash
# View raw ledger
cat ~/.config/dragoncore/data/ledger/production_ledger.csv

# Get metrics
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

## Troubleshooting | 故障排除

### Issue: tmux not found | 问题：找不到 tmux

```bash
# Ubuntu/Debian
sudo apt-get install tmux

# macOS
brew install tmux

# Verify
tmux -V
```

### Issue: API key not working | 问题：API密钥不工作

```bash
# Check if key is set
echo $KIMI_API_KEY

# Test API directly
curl https://api.moonshot.cn/v1/models \
  -H "Authorization: Bearer $KIMI_API_KEY"

# If empty response, key is invalid
```

### Issue: Permission denied | 问题：权限拒绝

```bash
# Fix permissions
chmod +x ./runtime/launch.sh
chmod +x ./target/release/dragoncore-runtime

# Or use cargo run
cargo run --release -- --help
```

### Issue: Port already in use | 问题：端口已被占用

DragonCore Runtime doesn't use network ports by default. If you see port conflicts:

```bash
# Check what's using the port
lsof -i :8080

# Kill if necessary
kill -9 <PID>
```

### Issue: Worktree already exists | 问题：工作树已存在

```bash
# Clean up old worktrees
dragoncore-runtime cleanup

# Or manually remove
rm -rf ~/.config/dragoncore/data/worktrees/RUN-xxx
```

### Debug Mode | 调试模式

```bash
# Enable debug logging
export RUST_LOG=debug
dragoncore-runtime run --task "Test"

# Or use --log-level flag
dragoncore-runtime --log-level debug run --task "Test"
```

---

## Best Practices | 最佳实践

### 1. Always Archive Completed Runs | 始终归档完成的运行

```bash
# Don't leave runs in limbo
dragoncore-runtime archive --run-id RUN-xxx --seat Yaoguang
```

### 2. Document Veto Reasons | 记录否决原因

```bash
# Be specific
dragoncore-runtime veto \
  --run-id RUN-xxx \
  --seat Yuheng \
  --reason "Line 42: Missing null check. See CWE-476."
```

### 3. Use Meaningful Run IDs | 使用有意义的运行ID

```bash
# Good
dragoncore-runtime run --run-id "RUN-AUTH-OAUTH2-001" --task "OAuth2"

# Avoid
dragoncore-runtime run --run-id "test1" --task "test"
```

### 4. Monitor Metrics Regularly | 定期监控指标

```bash
# Add to crontab for daily monitoring
0 9 * * * /usr/local/bin/dragoncore-runtime metrics >> /var/log/dragoncore-metrics.log
```

---

## Getting Help | 获取帮助

```bash
# General help
dragoncore-runtime --help

# Command-specific help
dragoncore-runtime run --help
dragoncore-runtime execute --help

# Check status
./runtime/launch.sh status

# View logs
tail -f ~/.config/dragoncore/data/dragoncore.log
```

---

## Next Steps | 下一步

1. Read [examples/governance_scenario_1.md](../runtime/examples/governance_scenario_1.md) for detailed scenarios
2. Explore [docs/19_SEATS.md](19_SEATS.md) for seat authority details
3. Review [docs/PRODUCTION_STATUS.md](PRODUCTION_STATUS.md) for monitoring

---

**真龙，不是龙虾。**  
**True Dragon. Not Claw.**
