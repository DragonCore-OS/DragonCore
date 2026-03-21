# Phase 3: 4小時 Endurance 準備狀態報告

**日期**: 2026-03-17/18  
**狀態**: ✅ **框架就緒，等待執行**

---

## 已完成工作

### 1. Endurance Runner 框架

**文件**: `endurance_runner.py`

**功能**:
- 定時創建 governance runs (可配置間隔)
- 自動執行多 seat 任務
- 並行 worker 處理 (3 workers)
- 實時指標收集 (JSONL format)
- 自動監控與報警
- 異常自動終止

**任務類型分佈**:
- 60% - 中等複雜度代碼審查
- 25% - 架構設計討論
- 10% - 高壓對抗性任務 (可能觸發 veto)
- 5% - 緊急修復

### 2. 預測試結果

**執行**: 15分鐘預測試 (已手動停止)  
**驗證項目**:
| 項目 | 狀態 |
|------|------|
| Runner 啟動 | ✅ |
| Run 創建 | ✅ (endurance-1773775288) |
| DragonCore 調用 | ✅ |
| 日誌記錄 | ✅ |
| 狀態報告 | ✅ |

### 3. 監控與日誌

**輸出文件**:
- `/tmp/endurance_live.log` - 實時狀態日誌
- `/tmp/endurance_metrics.jsonl` - 結構化指標
- `PHASE3_REPORT.json` - 最終報告 (測試結束後生成)

---

## 執行計劃

### 完整 4 小時測試

**命令**:
```bash
cd /home/admin/DragonCore-OS/DragonCore/runtime
nohup python3 endurance_runner.py --duration 240 --interval 300 > /tmp/endurance_nohup.log 2>&1 &
```

**預期負載**:
- 總 run 數: ~48
- 總 seat 調用: ~240
- 任務分佈: 混合類型
- 預期成功率: ≥95%

**監控命令**:
```bash
# 實時狀態
tail -f /tmp/endurance_live.log

# DragonCore 事件查詢
dragoncore events --run-id endurance-<timestamp>

# 系統資源
watch -n 30 'ps aux | grep dragoncore | wc -l; df -h .'
```

---

## 驗收標準

| # | 標準 | 門檻 |
|---|------|------|
| 1 | 運行時長 | ≥4 小時 |
| 2 | Run 成功率 | ≥95% |
| 3 | Event 丟失率 | 0% |
| 4 | Provider 漂移 | 0 次 |
| 5 | 內存洩漏 | <50MB/小時 |

---

## 風險與應對

| 風險 | 應對 |
|------|------|
| API 限流 | 指數退避 + 降級 |
| 磁盤滿 | 自動清理舊 worktree |
| 成功率過低 | 自動終止 (<70%) |
| 手動終止 | `pkill -f endurance_runner.py` |

---

## 結論

**Phase 3 框架已完成，準備就緒。**

執行完整測試需要:
1. 後台執行 nohup 命令
2. 4 小時無人工干預運行
3. 結束後檢查 PHASE3_REPORT.json

**建議**: 在生產環境或專用測試環境中執行，避免干擾其他工作。

---

## Phase 1-3 總結

| 階段 | 狀態 |
|------|------|
| Phase 2A-1 | ✅ PASS (RiskRaised + Provider tracking) |
| Phase 2A-2 | ✅ PASS (雙 Provider 環境) |
| Phase 2A-3 | ✅ FULL PASS (19席主治理鏈) |
| Phase 2B | ✅ FULL PASS (高壓對抗性任務) |
| Phase 3 | ⏳ **框架就緒，等待執行** |
