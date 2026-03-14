# DragonCore Runtime Smoke Test Results | 冒烟测试结果

**Test Date**: 2026-03-14  
**Version Tested**: 0.1.0  
**Platform**: Ubuntu 22.04 LTS  
**Tester**: Build System

---

## Executive Summary | 执行摘要

| Test Category | Passed | Failed | Skipped | Status |
|---------------|--------|--------|---------|--------|
| Compilation | 1 | 0 | 0 | ✅ PASS |
| CLI Structure | 5 | 0 | 0 | ✅ PASS |
| Static Commands | 4 | 0 | 0 | ✅ PASS |
| API-Dependent | 0 | 0 | 4 | ⏸️ SKIP |
| **Total** | **10** | **0** | **4** | ⚠️ PARTIAL |

**Overall Status**: ✅ Buildable and runnable for basic commands. 🔴 API-dependent features not tested.

---

## Test Environment | 测试环境

```bash
# System Info
$ uname -a
Linux dev-server 5.15.0-91-generic #101-Ubuntu SMP x86_64 GNU/Linux

# Rust Version
$ rustc --version
rustc 1.75.0 (82e1608df 2023-12-21)

# Cargo Version
$ cargo --version
cargo 1.75.0

# tmux Version
$ tmux -V
tmux 3.2a

# Git Version
$ git --version
git version 2.34.1
```

---

## Test Results Detail | 测试结果详情

### TC-001: Release Compilation | 发布编译

**Command**:
```bash
cargo build --release
```

**Expected**: Binary created without errors

**Actual**:
```
   Compiling dragoncore-runtime v0.1.0
    Finished release [optimized] target(s) in 46.14s
```

**Result**: ✅ **PASS**

**Evidence**:
```bash
$ ls -lh target/release/dragoncore-runtime
-rwxrwxr-x 2 admin admin 5.0M Mar 14 08:20 target/release/dragoncore-runtime

$ file target/release/dragoncore-runtime
target/release/dragoncore-runtime: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=..., for GNU/Linux 3.2.0, stripped
```

---

### TC-002: Binary Version Check | 二进制版本检查

**Command**:
```bash
./target/release/dragoncore-runtime --version
```

**Expected**: Version string displayed

**Actual**:
```
dragoncore-runtime 0.1.0
```

**Result**: ✅ **PASS**

---

### TC-003: Help Display | 帮助显示

**Command**:
```bash
./target/release/dragoncore-runtime --help
```

**Expected**: Full help text with all 13 commands

**Actual**: ✅ Full help displayed (see VERIFICATION_REPORT.md)

**Result**: ✅ **PASS**

---

### TC-004: Seats Command | 席位命令

**Command**:
```bash
./target/release/dragoncore-runtime seats
```

**Expected**: All 19 seats listed with Chinese names

**Actual**: ✅ All 19 seats displayed correctly

**Result**: ✅ **PASS**

---

### TC-005: Init Command | 初始化命令

**Command**:
```bash
./target/release/dragoncore-runtime init --output /tmp/test-init
```

**Expected**: Config file created

**Actual**:
```
2026-03-14T00:25:58.294358Z INFO dragoncore_runtime: Initializing DragonCore configuration
Configuration saved to: "/tmp/test-init/dragoncore.toml"
```

**Verification**:
```bash
$ cat /tmp/test-init/dragoncore.toml
[runtime]
name = "dragoncore"
version = "0.1.0"
...
```

**Result**: ✅ **PASS**

---

### TC-006: Metrics Command (Empty State) | 指标命令（空状态）

**Command**:
```bash
./target/release/dragoncore-runtime metrics
```

**Expected**: Empty metrics table (no runs yet)

**Actual**:
```
DragonCore Stability Metrics
============================
Total runs: 0
Clean runs: 0
Authority violations: 0
Fake closures: 0
Rollbacks: 0
Terminations: 0
```

**Result**: ✅ **PASS** (Zero state is correct for fresh install)

---

### TC-007: Run Command (No API Key) | 运行命令（无API密钥）

**Command**:
```bash
./target/release/dragoncore-runtime run --task "Test"
```

**Expected**: Error about missing API key or graceful degradation

**Actual**:
```
Error: No providers configured. Set KIMI_API_KEY, DEEPSEEK_API_KEY, or QWEN_API_KEY environment variable.
```

**Result**: ✅ **PASS** (Graceful error handling)

---

### TC-008: Status Command (No Runs) | 状态命令（无运行）

**Command**:
```bash
./target/release/dragoncore-runtime status
```

**Expected**: "No active runs" or empty list

**Actual**: ✅ Empty list displayed

**Result**: ✅ **PASS**

---

### TC-009: Execute Command (No API Key) | 执行命令（无API密钥）

**Command**:
```bash
./target/release/dragoncore-runtime execute \
  --run-id test-run \
  --seat Tianshu \
  --task "Test"
```

**Expected**: Error about no active run or missing API

**Actual**:
```
Error: Run not found
```

**Result**: ✅ **PASS** (Correct error - no run exists)

---

### TC-010: Veto Command (No Active Run) | 否决命令（无活跃运行）

**Command**:
```bash
./target/release/dragoncore-runtime veto \
  --run-id test-run \
  --seat Yuheng \
  --reason "Test"
```

**Expected**: Error about run not found

**Actual**:
```
Error: Run not found
```

**Result**: ✅ **PASS**

---

### TC-011: Final-Gate Command (No Active Run) | 终局命令（无活跃运行）

**Command**:
```bash
./target/release/dragoncore-runtime final-gate \
  --run-id test-run \
  --approve
```

**Expected**: Error about run not found

**Actual**:
```
Error: Run not found
```

**Result**: ✅ **PASS**

---

## Skipped Tests | 跳过的测试

These tests require API keys and were skipped in this run.

### TS-001: Full Run Cycle | 完整运行周期

**Status**: ⏸️ **SKIPPED** - Requires API key

**Test Steps**:
1. `dragoncore-runtime run --task "Implement OAuth2"`
2. `dragoncore-runtime execute --seat Tianquan --task "Plan"`
3. `dragoncore-runtime execute --seat Yuheng --task "Review"`
4. `dragoncore-runtime final-gate --approve`
5. `dragoncore-runtime archive --seat Yaoguang`

**Evidence Needed**:
- Run ID generated
- Ledger CSV updated
- tmux session created
- Worktree created

---

### TS-002: Veto Mechanism | 否决机制

**Status**: ⏸️ **SKIPPED** - Requires API key

**Test Steps**:
1. Start run
2. Execute seat with veto authority
3. Exercise veto
4. Verify run state changed to REJECTED
5. Verify ledger records veto

---

### TS-003: Termination Mechanism | 终止机制

**Status**: ⏸️ **SKIPPED** - Requires API key

**Test Steps**:
1. Start run
2. Execute terminate command
3. Verify run state changed to TERMINATED
4. Verify tmux session cleaned up
5. Verify ledger records termination

---

### TS-004: Concurrent Runs | 并发运行

**Status**: ⏸️ **SKIPPED** - Requires API keys

**Test Steps**:
1. Start Run A
2. Start Run B concurrently
3. Verify separate tmux sessions
4. Verify separate worktrees
5. Verify no cross-contamination

---

## Issues Found | 发现的问题

### IF-001: Config Path Ambiguity

**Severity**: Low

**Description**: Init creates config in specified directory, but runtime may not find it without `--config` flag.

**Workaround**: Always use `--config` flag

**Fix Target**: v0.1.1

---

### IF-002: Limited Error Context

**Severity**: Low

**Description**: Some errors don't include helpful context (e.g., "Run not found" doesn't suggest creating one).

**Workaround**: Refer to documentation

**Fix Target**: v0.2.0

---

## Recommendations | 建议

### For Immediate Testing | 立即测试建议

```bash
# Set API key
export KIMI_API_KEY="your-key-here"

# Run smoke test with API
./examples/test_governance.sh

# Or manual test
./target/release/dragoncore-runtime run --task "Test implementation"
```

### For CI/CD | 持续集成建议

```yaml
# .github/workflows/smoke-test.yml
name: Smoke Test
on: [push, pull_request]
jobs:
  smoke-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - run: ./target/release/dragoncore-runtime --version
      - run: ./target/release/dragoncore-runtime seats
      - run: ./target/release/dragoncore-runtime metrics
```

---

## Sign-off | 签字

**Tested By**: Build System  
**Date**: 2026-03-14  
**Status**: ⚠️ **Buildable, API-dependent features require manual verification**

**Next Steps**:
1. Obtain API keys
2. Run TS-001 through TS-004
3. Document actual outputs
4. Update VERIFICATION_REPORT.md

---

*This report is generated automatically during build. For manual test results, see VERIFICATION_REPORT.md*
