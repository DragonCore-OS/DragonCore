# Phase 3: 4小時 Endurance 測試規範

**版本**: v1.0  
**目標**: 驗證 DragonCore 在連續高負載下的穩定性  
**時長**: ≥4 小時 (14,400 秒)  
**日期**: 2026-03-17

---

## 測試目標

| # | 目標 | 驗證方式 |
|---|------|----------|
| 1 | 連續運行 ≥4 小時 | 時間戳記錄 |
| 2 | 無人工干預 | 自動化執行 |
| 3 | Provider 路由無漂移 | Seat→Provider 映射一致性檢查 |
| 4 | State/Ledger/Events 持續一致 | 交叉驗證 |
| 5 | 中後段性能無退化 | 延遲/吞吐量趨勢分析 |

---

## 測試架構

### 組件

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Task Generator │────▶│  DragonCore     │────▶│  Metrics        │
│  (定時生成任務)  │     │  Runtime        │     │  Collector      │
└─────────────────┘     └─────────────────┘     └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Task Queue     │     │  DIBL Events    │     │  Health Monitor │
│  (FIFO)         │     │  (JSONL)        │     │  (異常檢測)      │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

### 工作負載設計

**任務類型分佈** (模擬真實場景):
- 60% - 中等複雜度代碼審查 (3-5 seat)
- 25% - 架構設計討論 (5-8 seat)
- 10% - 高壓對抗性任務 (8-12 seat, 可能觸發 veto)
- 5% - 緊急修復 (快速路徑)

**任務頻率**:
- 每 5 分鐘啟動一個新 run
- 每個 run 並行執行 3-5 seat
- 預計總 run 數: ~50 runs
- 預計總 seat 調用: ~250 seat executions

---

## 硬性驗收標準

### 通過標準 (必須全部滿足)

| # | 標準 | 門檻 | 檢測方式 |
|---|------|------|----------|
| 1 | 總運行時長 | ≥4 小時 | start_time - end_time |
| 2 | Run 成功率 | ≥95% | 成功 run / 總 run |
| 3 | Event 丟失率 | 0% | DIBL 完整性檢查 |
| 4 | Ledger 一致性 | 100% | 交叉驗證 |
| 5 | Provider 漂移 | 0 次 | Seat→Provider 映射變更 |
| 6 | 內存洩漏 | <50MB/小時 | 進程內存監控 |
| 7 | API 超時率 | <5% | 請求成功率 |

### 性能指標

| 指標 | 前1小時 | 後1小時 | 退化限制 |
|------|---------|---------|----------|
| Avg Seat Latency | baseline | +<20% | 通過 |
| Event Write TPS | baseline | +<10% | 通過 |
| Memory Usage | baseline | +<30% | 通過 |

---

## 異常檢測規則

### 立即報警 (Alert)

```
IF run_failure_rate > 10% IN 10_minutes THEN alert
IF provider_mismatch_detected THEN alert
IF event_sequence_gap_detected THEN alert
IF memory_usage > 2GB THEN alert
```

### 自動終止 (Terminate)

```
IF run_failure_rate > 30% THEN terminate
IF disk_space < 1GB THEN terminate
IF API_key_invalid THEN terminate
```

---

## 輸出物

### 實時輸出
- `/tmp/endurance_live.log` - 實時狀態
- `/tmp/endurance_metrics.jsonl` - 指標流

### 最終報告
- `PHASE3_REPORT.md` - 完整報告
- `runtime_state/events/endurance_*.jsonl` - 所有事件
- `runtime_state/ledger/endurance_ledger.csv` - Ledger
- `runtime_state/metrics/endurance_stats.json` - 統計

---

## 執行計劃

### 預熱階段 (15 分鐘)
- 驗證環境就緒
- 測試單個 run
- 確認 provider 可用

### 主測試階段 (4 小時)
- 自動化循環執行
- 每 5 分鐘一個新 run
- 實時監控與報警

### 收尾階段 (15 分鐘)
- 生成報告
- 數據驗證
- 清理資源

---

## 風險與應對

| 風險 | 應對措施 |
|------|----------|
| Provider API 限流 | 指數退避重試 + 降級 |
| 磁盤空間不足 | 自動清理舊 worktree |
| 內存洩漏 | 定期強制 GC + 監控 |
| 網絡中斷 | 斷點續傳 + 超時處理 |

---

**下一步**: 實現自動化 endurance runner 腳本。
