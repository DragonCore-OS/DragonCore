# DragonCore Public Beta - 封板週完成狀態

**Date**: 2026-03-16  
**Version**: v0.2.1-DIBL → v0.3.0-beta.1  
**Status**: 🟢 封板完成，等待發布指令

---

## 封板週完成項目

### ✅ P0: 編譯警告清零

```
cargo build --release 2>&1 | grep -c "^warning:"
# Output: 0
```

- 所有未使用代碼標記 `#[allow(dead_code)]`
- Release 構建乾淨

### ✅ P0: 驗證腳本（硬門檻）

文件: `verify_beta_release.sh`

檢查項：
1. 編譯警告 = 0
2. 所有測試通過
3. Release 二進製存在
4. CLI 命令工作
5. DIBL 樣本向量
6. AXI 互解析樣本
7. 必需文檔存在
8. 運行時目錄結構
9. DIBL CLI 命令
10. 樣本運行投影

### ✅ P1: DIBL 測試擴展（5 → 11）

| # | 測試名稱 | 目的 |
|---|---------|------|
| 1 | `test_jsonl_store_append_and_load` | 基本存儲 |
| 2 | `test_operator_projection` | 投影準確性 |
| 3 | `test_correlation_context` | Correlation 字段 |
| 4 | `test_axi_sample_interop` | AXI 互解析 |
| 5 | `test_event_append_isolation` | 追加隔離 |
| 6 | `test_malformed_jsonl_graceful_handling` | 容錯讀取 |
| 7 | `test_internal_events_filtered` | 可見性過濾 |
| 8 | `test_replay_output_order_stable` | 順序穩定 |
| 9 | `test_different_run_id_isolation` | Run 隔離 |
| 10 | `test_security_events_channel_scope` | Security 事件 |
| 11 | `test_...` | (預留) |

全部通過：
```
running 11 tests
test result: ok. 11 passed; 0 failed
```

### ✅ P1: 最小樣本包

| 樣本文件 | 場景 | 事件數 |
|---------|------|--------|
| `sample-run-created.jsonl` | 正常創建 | 3 |
| `sample-risk-veto.jsonl` | 風險與否決 | 5 |
| `sample-archive.jsonl` | 完整歸檔 | 6 |

### ✅ P2: 文檔收口

保留 3 份對外文檔：
1. `PUBLIC_BETA_ROADMAP.md` - 路線圖
2. `PUBLIC_BETA_STATUS.md` - 本文件
3. `src/events/README.md` - DIBL 文檔

---

## 新增 CLI 命令

| 命令 | 功能 | 狀態 |
|------|------|------|
| `events --run-id <id>` | 查看事件 | ✅ |
| `events --format json` | JSON 輸出 | ✅ |
| `replay --run-id <id>` | 回放事件 | ✅ |

---

## 關鍵改進

### 容錯處理
- JSONL 非法行自動跳過（不崩潰）
- 記錄警告日誌

### 代碼質量
- 0 編譯警告
- 11 個單元測試
- 硬門檻驗證腳本

---

## 發布指令待確認

當前狀態：**✅ 封板完成**

執行以下指令發布：

```bash
git tag v0.3.0-beta.1
git push origin v0.3.0-beta.1
```

或等待進一步指令。
