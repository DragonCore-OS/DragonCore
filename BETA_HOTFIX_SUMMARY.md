# DragonCore v0.3.0-beta.1 Hotfix Summary

**Date**: 2026-03-17  
**Commit**: 91ab5e0  
**Status**: Hotfix applied to main

---

## Beta Feedback Issues Fixed

### 1. Version Number Display (P2) ✅

**Problem**: `--version` showed 0.1.0 instead of v0.3.0-beta.1

**Fix**: Updated `Cargo.toml` version = "0.3.0-beta.1"

**Verification**:
```bash
$ dragoncore --version
dragoncore 0.3.0-beta.1  ✅
```

---

### 2. Git Prerequisites Check (P2) ✅

**Problem**: Cryptic errors when not in git repo or no commits

**Fix**: Added clear checks with helpful error messages

**Before**:
```
fatal: not a git repository
fatal: ambiguous argument 'HEAD': unknown revision
```

**After**:
```
Error: Not a git repository. DragonCore requires:
1. git init
2. At least one commit (git add && git commit)
Please initialize a git repository first.
```

---

### 3. Providers Configuration (P2) ✅

**Problem**: Run created successfully but execution failed with "No providers configured"

**Fix**: Check providers at run creation time with setup guide

**New Error**:
```
Error: No model providers configured.

To use DragonCore, you need to configure at least one AI provider.
Edit dragoncore.toml and add your API keys:

[providers.kimi]
provider_type = "kimi"
api_key = "your-api-key"
...

Get your API key from: https://platform.moonshot.cn/
```

---

### 4. Status Display Consistency (P3) ✅

**Problem**: "Loaded 2 runs" but "Active runs: 0" was confusing

**Fix**: Status now shows both total and active runs clearly

**New Output**:
```
Total runs in storage: 2
Active runs (in memory): 0

All runs:
  run-001: Created
  run-002: Approved [active]
```

---

## Remaining Issues

### API Key Required for Full Testing

To fully test model execution (seat execution), real API keys are needed:

1. **Kimi** (Moonshot AI): https://platform.moonshot.cn/
2. **DeepSeek**: https://platform.deepseek.com/
3. **Qwen**: https://dashscope.aliyun.com/

Without API keys, the following cannot be tested:
- Actual model responses
- Seat execution flow
- End-to-end governance with AI

DIBL events, replay, and projection work correctly without API keys.

---

## Verification Status

| Check | Status |
|-------|--------|
| Version display | ✅ Fixed |
| Git checks | ✅ Fixed |
| Providers check | ✅ Fixed |
| Status display | ✅ Fixed |
| Build warnings | ✅ 0 warnings |
| Tests | ✅ 11/11 passing |

---

## Next Steps

1. Consider tagging v0.3.0-beta.2 with these fixes
2. Continue beta testing with real API keys
3. Monitor for additional feedback

---

**DragonCore Team**
2026-03-17
