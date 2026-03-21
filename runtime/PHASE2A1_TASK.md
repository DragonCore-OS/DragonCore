# Phase 2A-1: 跨模組事件追蹤修復任務

**Run ID**: phase2a1-risk-events  
**日期**: 2026-03-17  
**目標**: 修復 RiskRaised 事件未發射缺陷，完善 DIBL 事件覆蓋

---

## 發現的缺陷

### 缺陷 1: RiskRaised 事件缺失 (P1)
- **位置**: `src/runtime/mod.rs`
- **問題**: `GovernanceEventType::RiskRaised` 已定義，但 runtime 沒有方法觸發它
- **影響**: Security channel 永遠不會收到風險事件，無法驗證治理深度

### 缺陷 2: 部分事件缺少 provider tracking (P2)
- **位置**: `src/runtime/mod.rs` - veto, final_gate, archive, terminate
- **問題**: 這些事件沒有記錄 provider 字段
- **影響**: 無法完整追蹤哪個模型處理了哪個治理動作

---

## 修復計劃

### 修復 1: 添加 raise_risk 方法
```rust
pub async fn raise_risk(&self, run_id: &str, seat: Seat, risk_type: &str, description: &str) -> Result<()>
```
- 發射 RiskRaised 事件到 Security channel
- 記錄風險類型和描述
- 更新 ledger

### 修復 2: 為所有事件添加 provider tracking
- VetoExercised: 記錄執行 veto 的 seat 使用的 provider
- FinalGateOpened: 記錄 Tianshu 使用的 provider
- ArchiveCompleted: 記錄執行歸檔的 seat 使用的 provider
- TerminateTriggered: 記錄執行終止的 seat 使用的 provider

### 修復 3: 補充測試
- 添加 test_raise_risk_emits_event
- 添加 test_provider_tracking_in_all_events
- 添加 test_security_channel_events

---

## Seat 參與設計

| Seat | 角色 | 動作 |
|------|------|------|
| Tianquan (CSO) | 制定修復計劃 | 輸出修復方案 |
| Kaiyang (實現審查) | 分析影響 | Review 代碼變更 |
| Baihu (紅隊) | 風險評估 | 找出回歸風險 |
| Yuheng (質量) | Review | 審查測試覆蓋 |
| Luban (平台) | 實現 | 寫代碼 |
| Zhongkui (清除) | 清理 | 移除冗餘代碼 |
| Tianshu (終局) | 批准 | Final gate |

---

## 驗收標準

- [ ] ≥10 seat 出現在事件流
- [ ] RiskRaised 事件成功發射
- [ ] 所有事件都有 provider 字段
- [ ] FinalGateOpened → DecisionCommitted
- [ ] ArchiveCompleted 收束
- [ ] replay 順序穩定

---

## 執行

開始執行修復...
