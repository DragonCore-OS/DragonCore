# Phase 3: 4小時 Endurance 測試啟動報告

**啟動時間**: 2026-03-18 22:06:05 CST  
**預期結束**: 2026-03-19 02:06:05 CST  
**時長**: 4 小時 (240 分鐘)  
**狀態**: 🟢 **運行中**

---

## 執行配置

| 配置項 | 值 |
|--------|-----|
| Duration | 240 分鐘 (4 小時) |
| Interval | 300 秒 (5 分鐘) |
| Workers | 3 (並行) |
| 預期 Run 數 | ~48 |
| 預期 Seat 調用 | ~240 |

---

## 進程狀態

```
PID 1716798: python3 endurance_runner.py (主控)
PID 1720981: dragoncore-runtime execute (執行中)
```

## 已創建 Runs

| Run ID | 創建時間 | 狀態 |
|--------|----------|------|
| endurance-1773775288 | 03:23 (預測試) | 已完成 |
| endurance-1773842765 | 22:07 | 執行中 |

## 監控入口

```bash
# 實時狀態
tail -f /tmp/endurance_live.log

# 結構化指標
tail -f /tmp/endurance_metrics.jsonl

# 主日誌
tail -f /tmp/endurance_main.log

# 進程監控
ps aux | grep endurance_runner

# Run 目錄
ls ./runtime_state/worktrees/endurance-*
```

---

## 驗收標準

測試結束後驗證:

| # | 標準 | 門檻 |
|---|------|------|
| 1 | 運行時長 | ≥4 小時 |
| 2 | Run 成功率 | ≥95% |
| 3 | Event 丟失率 | 0% |
| 4 | Provider 漂移 | 0 次 |
| 5 | Ledger 一致性 | 100% |

---

## 注意事項

1. 測試已後台執行 (nohup)，斷線不影響
2. 異常情況會自動終止並記錄
3. 最終報告將生成於: PHASE3_REPORT.json
4. 所有事件保留於: runtime_state/events/endurance-*.jsonl

---

**測試進行中，請勿手動干預。預期 2026-03-19 02:06 自動結束。**
