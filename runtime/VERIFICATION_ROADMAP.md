# DragonCore 長程驗證路線圖

**日期**: 2026-03-17  
**版本**: v0.3.0-beta.2  
**階段**: 正式驗證（不加新功能）

---

## 當前狀態誠實評估

| 命題 | 狀態 | 證據 |
|------|------|------|
| 真實 GPT-OSS-120B 調用 | **[Verified]** | 2026-03-17 測試報告，單run成功 |
| seat-based 多模型路由 | **[Verified]** | 代碼實現，seat_str 傳遞 |
| 基於規則的智能自動切換 | **[Unknown]** | 配置存在，runtime 解析代碼未找到 |
| 19席完整會議流 | **[Unknown]** | 歷史文檔標註 SKIPPED，需重新驗證 |
| 4小時以上連續自動運行 | **[Unknown]** | 無歷史證據，必須補做 |

---

## Phase 1: 雙模型 Seat 路由驗證

### 目標
驗證同個 run 內，不同 seat 確實調用不同模型。

### 配置
```toml
[providers.local_gpt_oss_120b]
provider_type = "open_ai_compatible"
base_url = "http://localhost:8000/v1"
model = "openai/gpt-oss-120b"

[providers.kimi_cli_fast]
provider_type = "kimi_cli"
base_url = "https://api.kimi.com/coding/v1"
model = "kimi-for-coding"

[seat_models]
"Tianshu" = "local_gpt_oss_120b"      # 高能力
"Yuheng" = "local_gpt_oss_120b"       # 高能力
"Tianquan" = "kimi_cli_fast"          # 快速
"Nezha" = "kimi_cli_fast"             # 快速
default = "local_gpt_oss_120b"
```

### 測試案例

#### Case 1: 短任務
- **任務**: "修復單個文件的 syntax error"
- **期望**: Tianquan (Kimi CLI) 快速響應
- **驗收**: DIBL 事件顯示 Tianquan -> kimi_cli_fast

#### Case 2: 複雜任務
- **任務**: "跨 3-5 個文件的架構重構，需要評審"
- **期望**: Tianshu (GPT-OSS-120B) 深度分析，Tianquan (Kimi CLI) 快速執行
- **驗收**: 
  - DIBL 事件顯示兩種 provider
  - 日誌中能看到 `provider_type` 切換

### 驗收標準
- [ ] 同個 run 內至少出現兩種 provider 被實際調用
- [ ] DIBL 事件記錄每個 seat 使用的 provider
- [ ] runtime 日誌能定位 seat -> provider 映射
- [ ] token/latency/artifact 可回放

---

## Phase 2: 19 席會議流驗證

### 目標
驗證多席協作 governance 閉環，非單席執行。

### 測試任務設計
**任務**: "大倉庫 bug 修復 + 跨模塊架構 review"

```
1. Tianquan (CSO) - 制定修復計劃
2. Qinglong (探索) - 調查影響範圍
3. Baihu (紅隊) - 找出邊界情況
4. Yuheng (質量) - review 計劃
5. Luban (平台) - 實現工具
6. Kaiyang (實現審查) - code review
7. Tianshu (終局) - 最終決策
```

### 治理動作驗證
- [ ] 至少 10+ 個不同 seat 真實執行
- [ ] 至少一次 review / veto / revise / final gate
- [ ] worktree artifacts 與 DIBL timeline 一致
- [ ] ledger 狀態與實際輸出一致

### 驗收標準
- [ ] 多席參與同一任務
- [ ] 中間治理動作有記錄
- [ ] 最終狀態正確

---

## Phase 3: 4 小時 Endurance 驗證

### 目標
無人工干預下連續運行 ≥ 4 小時，系統不漂移。

### 測試設計
**隊列式連續任務**（非單超長任務）

| 時間 | 任務類型 | 複雜度 |
|------|----------|--------|
| 0:00 | Code fix | 低 |
| 0:20 | Analysis | 中 |
| 0:40 | Review | 中 |
| 1:00 | Documentation | 低 |
| 1:20 | Red-team | 高 |
| ... | ... | ... |
| 4:00 | 結束驗證 | - |

### 必須記錄字段
```yaml
per_run:
  - run_id
  - start_time
  - end_time
  - seats_involved: [list]
  - provider_per_seat: {seat: provider}
  - prompt_tokens
  - completion_tokens
  - total_tokens
  - latency_per_seat: {seat: ms}
  - timeout_count
  - retry_count
  - veto_count
  - revise_count
  - rollback_count
  - failure_mode: null | timeout | error | rejected
  - final_outcome: approved | rejected | terminated

aggregate:
  - total_runs
  - success_rate
  - avg_latency
  - max_latency
  - token_total
  - human_intervention_count
  - state_corruption_count
  - event_missing_count
```

### 通過門檻
- [ ] 連續運行 ≥ 4h
- [ ] 無人工干預
- [ ] 無 run state 損壞
- [ ] 無 event/ledger 漏寫
- [ ] 任務完成率 ≥ 80%
- [ ] 中後段性能無系統性惡化
- [ ] provider 路由與配置一致

---

## 技術風險說明

### 風險 1: "自動切換"誤報
**當前狀態**: 
- ✅ seat-based 固定映射：已實現
- ❓ 基於 task complexity 的智能切換：配置存在，runtime 解析代碼未找到

**驗證方法**: 檢查 runtime 是否讀取 `model_selection.rules`

### 風險 2: 19 席並發 ≠ 19 席已驗證
**當前狀態**: 
- `max_concurrent_agents = 19` 僅為配置
- 未證明 19 席真實連續穩定運行

**驗證方法**: Phase 2 多席協作測試

### 風險 3: 4 小時無證據
**當前狀態**: 無歷史 endurance 記錄

**驗證方法**: Phase 3 長程 soak

---

## 當前執行指令

1. **今天開始**: Phase 1 雙模型驗證
2. **完成後**: Phase 2 19 席會議流
3. **最後**: Phase 3 4 小時 endurance
4. **不加新功能**: 僅驗證現有代碼

---

## 結論分類標準

| 分類 | 定義 |
|------|------|
| **[Verified]** | 已由日誌/代碼/事件證明 |
| **[Failed]** | 明確未過 |
| **[Unknown]** | 尚無足夠證據 |

---

**DragonCore 團隊**  
2026-03-17
