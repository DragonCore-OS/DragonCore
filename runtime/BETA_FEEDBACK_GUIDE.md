# DragonCore Beta 反饋指南

**版本**: v0.3.0-beta.1  
**階段**: Beta 反饋閉環  
**生效日期**: 2026-03-16

---

## 主線凍結聲明

⚠️ **v0.3.0-beta.1 發布後，主線凍結生效**

### ✅ 接受
- Beta 反饋修復
- 回歸缺陷修復
- 文檔糾正

### ❌ 不接受
- 新功能（Windows/Desktop、SQLite、Web UI）
- DIBL v0.1 schema 改動
- CLI 擴展
- 大規模重構

---

## 缺陷分級表

| 級別 | 定義 | 響應時間 | 示例 |
|------|------|----------|------|
| **P0** | 數據損壞、狀態錯亂、run 無法恢復、事件回放錯誤 | 24小時內 | JSONL 損壞、ledger 不一致、projection 完全錯誤 |
| **P1** | CLI 核心命令失效、projection 錯誤、DIBL 事件漏發 | 3天內 | `events` 命令崩潰、veto 事件未記錄 |
| **P2** | 樣本包/腳本問題、文檔不一致、邊界提示不清 | 7天內 | 樣本 JSONL 格式錯誤、文檔步驟缺失 |
| **P3** | 體驗問題、輸出格式問題、提示文案問題 | 下個版本 | 輸出對齊問題、顏色提示不明顯 |

---

## 反饋提交格式（強制）

所有反饋必須包含以下信息，否則不進入修復隊列：

```markdown
## 基本信息
- **版本號**: v0.3.0-beta.1
- **平台**: Ubuntu 22.04 / WSL2
- **Git commit**: (如適用)

## 命令
```bash
# 完整命令行
```

## 輸入樣本
- Run ID: 
- 命令參數:

## 實際輸出
```
# 實際輸出內容
```

## 預期輸出
```
# 預期輸出內容
```

## DIBL 輸出
```bash
dragoncore events --run-id <id>
# 輸出內容
```

## Replay 輸出
```bash
dragoncore replay --run-id <id>
# 輸出內容
```

## 分級建議
P0/P1/P2/P3

## 補充說明
- 可重現步驟：
- 相關日誌：
```

---

## Beta 成功條件

滿足以下條件後，考慮進入下一版本：

| 條件 | 標準 | 當前狀態 |
|------|------|----------|
| P0 缺陷 | 連續 7 天無 P0 | 監控中 |
| 外部測試 | 連續 5 個外部 beta case 無回歸 | 監控中 |
| 驗證腳本 | `verify_beta_release.sh` 持續全綠 | ✅ 已達成 |
| DIBL 互解析 | 樣本互解析無漂移 | ✅ 已達成 |
| 編譯警告 | 無新增 warning | ✅ 已達成 |

---

## 回歸測試最小集

任何修復必須跑以下 11 個測試，無例外：

```bash
cargo test events::

# 測試列表
test events::tests::test_correlation_context ......... ok
test events::tests::test_axi_sample_interop ......... ok
test events::tests::test_internal_events_filtered ... ok
test events::tests::test_operator_projection ........ ok
test events::tests::test_security_events_channel .... ok
test events::tests::test_malformed_jsonl_graceful ... ok
test events::tests::test_jsonl_store_append_and_load  ok
test events::tests::test_event_append_isolation ..... ok
test events::tests::test_different_run_id_isolation . ok
test events::tests::test_replay_output_order_stable . ok
test persistence::tests::test_json_file_store ....... ok
```

---

## Beta 基線報告

### 發布信息
- **發布時間**: 2026-03-16
- **版本**: v0.3.0-beta.1
- **Tag**: v0.3.0-beta.1
- **Commit**: 5691c5e

### 質量基線
- **單元測試**: 11/11 passing
- **編譯警告**: 0
- **DIBL 版本**: v0.1 (凍結)

### 已知邊界
- ✅ 單節點
- ✅ JSON-backed
- ✅ CSV ledger
- ✅ Linux/WSL
- ✅ CLI

### 非 Scope
- ❌ Windows Desktop
- ❌ SQLite
- ❌ Web UI
- ❌ 多節點

---

## 聯繫方式

- **GitHub Issues**: https://github.com/DragonCore-OS/DragonCore/issues
- **標籤**: `beta-feedback`

---

**DragonCore 團隊**  
2026-03-16
