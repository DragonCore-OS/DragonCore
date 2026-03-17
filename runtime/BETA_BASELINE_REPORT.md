# DragonCore Beta 基線報告

**報告編號**: BETA-2026-03-16-001  
**版本**: v0.3.0-beta.1  
**發布日期**: 2026-03-16  
**狀態**: 正式基線

---

## 1. 發布信息

| 項目 | 值 |
|------|-----|
| 版本號 | v0.3.0-beta.1 |
| Git Tag | v0.3.0-beta.1 |
| Commit | 5691c5e |
| 發布時間 | 2026-03-16 |
| 發布人 | DragonCore Team |

---

## 2. 質量基線

### 2.1 測試結果

```
Test Result: ok. 11 passed; 0 failed; 0 ignored
```

| # | 測試名稱 | 類別 | 狀態 |
|---|---------|------|------|
| 1 | test_correlation_context | DIBL | ✅ |
| 2 | test_axi_sample_interop | DIBL | ✅ |
| 3 | test_internal_events_filtered | DIBL | ✅ |
| 4 | test_operator_projection | DIBL | ✅ |
| 5 | test_security_events_channel | DIBL | ✅ |
| 6 | test_malformed_jsonl_graceful | DIBL | ✅ |
| 7 | test_jsonl_store_append_and_load | DIBL | ✅ |
| 8 | test_event_append_isolation | DIBL | ✅ |
| 9 | test_different_run_id_isolation | DIBL | ✅ |
| 10 | test_replay_output_order_stable | DIBL | ✅ |
| 11 | test_json_file_store | Persistence | ✅ |

### 2.2 編譯狀態

```bash
$ cargo build --release 2>&1 | grep -c "^warning:"
0
```

- **編譯警告**: 0
- **Release 構建**: ✅ 成功
- **二進製大小**: ~5.4 MB

### 2.3 驗證腳本

```bash
$ ./verify_beta_release.sh
# Output: ALL CHECKS PASSED
```

---

## 3. 功能基線

### 3.1 核心功能

| 功能 | 狀態 | 備註 |
|------|------|------|
| Run 創建 | ✅ | JSON state 持久化 |
| Seat 執行 | ✅ | 19席可用 |
| Veto | ✅ | Security 通道 |
| Final Gate | ✅ | Tianshu 專用 |
| Archive | ✅ | Ops 通道 |
| Terminate | ✅ | Security 通道 |
| Events CLI | ✅ | DIBL 查詢 |
| Replay CLI | ✅ | 事件回放 |

### 3.2 DIBL v0.1 (凍結)

- **Schema**: 14 字段，snake_case
- **存儲**: JSONL
- **路徑**: `runtime_state/events/{run_id}.jsonl`
- **通道**: Control, Ops, Security, Research
- **Scope**: Internal, OperatorVisible, Exportable

### 3.3 AXI 互解析

- **DragonCore → AXI**: ✅ 驗證通過
- **AXI → DragonCore**: ✅ 驗證通過
- **樣本交換**: ✅ 完成

---

## 4. 已知邊界

### 4.1 當前支持

- ✅ 單節點部署
- ✅ JSON 持久化
- ✅ CSV ledger
- ✅ Linux/WSL 環境
- ✅ CLI 介面
- ✅ Tmux 隔離
- ✅ Git worktree

### 4.2 不在 Scope

- ❌ Windows Desktop (v0.5.0)
- ❌ SQLite 後端 (v0.4.0)
- ❌ Web UI (v0.5.0)
- ❌ 多節點分布式 (v0.6.0)
- ❌ 流式響應
- ❌ 重試邏輯

---

## 5. 樣本包

| 樣本文件 | 場景 | 事件數 | 驗證 |
|---------|------|--------|------|
| sample-run-created.jsonl | 正常創建 | 3 | ✅ |
| sample-risk-veto.jsonl | 風險與否決 | 5 | ✅ |
| sample-archive.jsonl | 完整歸檔 | 6 | ✅ |
| dragoncore_sample.jsonl | 完整流程 | 8 | ✅ |
| axi_sample.jsonl | AXI 互操作 | 4 | ✅ |

---

## 6. 文檔基線

| 文檔 | 狀態 | 用途 |
|------|------|------|
| README.md | ✅ | 項目介紹 |
| STATUS.md | ✅ | v0.2.1 狀態 |
| DIBL_STATUS.md | ✅ | DIBL v0.1 狀態 |
| PUBLIC_BETA_ROADMAP.md | ✅ | 公開測試路線圖 |
| PUBLIC_BETA_STATUS.md | ✅ | 封板狀態 |
| BETA_FEEDBACK_GUIDE.md | ✅ | 反饋指南 |
| BETA_BASELINE_REPORT.md | ✅ | 本報告 |
| src/events/README.md | ✅ | DIBL 技術文檔 |

---

## 7. 後續監控

### 7.1 Beta 成功條件

| 條件 | 目標 | 當前 | 狀態 |
|------|------|------|------|
| P0 缺陷 | 連續 7 天無 P0 | - | 🟡 監控中 |
| 外部測試 | 5 個 case 無回歸 | - | 🟡 等待中 |
| 驗證腳本 | 持續全綠 | ✅ | 🟢 達成 |
| DIBL 互解析 | 無漂移 | ✅ | 🟢 達成 |
| 編譯警告 | 無新增 | ✅ | 🟢 達成 |

### 7.2 回歸測試

任何修復必須通過：
```bash
cargo test  # 11 個測試
./verify_beta_release.sh  # 硬門檻
```

---

## 8. 版本歷史

| 版本 | 日期 | 說明 |
|------|------|------|
| v0.1.0 | 2026-03-14 | Buildable skeleton |
| v0.2.0 | 2026-03-14 | Persistence verified |
| v0.2.1 | 2026-03-14 | Ledger & metrics verified |
| **v0.3.0-beta.1** | **2026-03-16** | **DIBL v0.1, Public Beta** |

---

## 9. 簽署

**報告生成**: 自動化腳本  
**審核**: DragonCore Team  
**生效**: 2026-03-16

---

*本報告為所有 Beta 反饋判斷的基準面。*
