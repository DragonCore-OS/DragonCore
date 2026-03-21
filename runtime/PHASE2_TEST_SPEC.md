# Phase 2: 19席治理流硬驗證規範

**日期**: 2026-03-17  
**版本**: v0.3.0-beta.2 + Phase 1  
**目標**: 證明19席治理流在真實任務下形成完整閉環

---

## 硬門檻（必須全部滿足）

| # | 門檻 | 標準 |
|---|------|------|
| 1 | Seat 覆蓋 | ≥10個不同 seat 出現在事件流 |
| 2 | 治理深度 | ≥1個 security channel 事件 (RiskRaised/VetoExercised/TerminateTriggered) |
| 3 | 閉環完整 | FinalGateOpened → DecisionCommitted + ArchiveCompleted，或合理 TerminateTriggered |
| 4 | 事件一致性 | replay --run-id 順序穩定，與 artifact/ledger 對上 |
| 5 | 輸出完備 | 5份輸出物齊全 |

---

## Phase 2A: 3個中等複雜度任務

### 任務 2A-1: 跨模組事件追蹤修復

**目標**: 修復 DIBL 事件追蹤缺陷，補齊測試

**背景**: 
發現 DIBL 事件在某些情況下未正確記錄 provider 字段，需要：
1. 定位問題模組
2. 修復缺陷
3. 補充測試覆蓋
4. 驗證 replay 功能

**輸入**:
```
任務類型: code_fix
影響範圍: src/events/mod.rs, src/runtime/mod.rs
缺陷描述: provider 字段在部分事件類型中丟失
要求:
  - 定位所有未記錄 provider 的事件類型
  - 統一添加 provider tracking
  - 補 3 個以上單元測試
  - 驗證 replay 能正確顯示 provider
```

**期望 Seat 參與**:
- Tianquan (CSO): 制定修復計劃
- Kaiyang (實現審查): 分析代碼影響
- Baihu (紅隊): 找出潛在回歸風險
- Yuheng (質量): review 修復方案
- Luban (平台): 實現工具函數
- Zhongkui (清除): 清理冗餘代碼
- Tianshu (終局): 最終批准

**驗收欄位**:
- [ ] ≥10 seat 有事件記錄
- [ ] RiskRaised 或 VetoExercised 出現（如發現風險）
- [ ] FinalGateOpened 後 DecisionCommitted
- [ ] ArchiveCompleted 收束
- [ ] replay 輸出與實際一致

---

### 任務 2A-2: 多模型配置系統完善

**目標**: 完善多模型配置解析，支持動態模型選擇

**背景**:
已實現 seat-based 模型映射，但缺少：
1. 配置驗證（檢查 provider 是否存在）
2. 動態切換（根據任務複雜度）
3. 容錯處理（provider 失敗時切換）

**輸入**:
```
任務類型: feature_enhancement
影響範圍: src/config/mod.rs, src/models/mod.rs
需求:
  - 添加配置驗證：seat_models 引用的 provider 必須存在
  - 實現 provider 健康檢查
  - 添加 fallback 機制：主 provider 失敗時切換備用
  - 記錄切換事件到 DIBL
```

**期望 Seat 參與**:
- Tianji (CTO): 技術方案設計
- Qinglong (探索): 調研 fallback 策略
- Xuanwu (穩定性): 評估風險
- Nezha (快速部署): 實現核心邏輯
- Yangjian (質檢): 驗證容錯場景
- Baozheng (審計): 審查配置安全
- Yaoguang (歸檔): 文檔更新

**驗收欄位**:
- [ ] ≥10 seat 有事件記錄
- [ ] RiskRaised 出現（配置風險評估）
- [ ] 至少一次 provider fallback 被觸發
- [ ] FinalGateOpened → DecisionCommitted
- [ ] ArchiveCompleted 收束

---

### 任務 2A-3: 4小時 endurance 框架準備

**目標**: 設計並實現 4 小時 endurance 測試框架

**背景**:
需要自動化框架支持：
1. 定時推送任務
2. 自動記錄指標
3. 異常檢測與報警
4. 結果彙總

**輸入**:
```
任務類型: platform_feature
影響範圍: src/runtime/, tests/
需求:
  - 實現 endurance runner：定時創建 run
  - 添加指標收集：latency, token, success rate
  - 實現 health check：檢測狀態漂移
  - 產生 endurance report
```

**期望 Seat 參與**:
- Zhugeliang (軍師): 整體架構設計
- Tianquan (CSO): 協調多 seat 分工
- Luban (平台): 實現核心框架
- Kaiyang (審查): review 設計
- Yuheng (質量): 驗收標準制定
- Fengdudadi (終結): 異常處理設計

**驗收欄位**:
- [ ] ≥10 seat 有事件記錄
- [ ] 框架能自動運行多個 run
- [ ] RiskRaised 出現（如發現性能瓶頸）
- [ ] FinalGateOpened → DecisionCommitted
- [ ] ArchiveCompleted 收束

---

## Phase 2B: 1個高壓任務

### 任務 2B-1: 矛盾需求對抗性任務

**目標**: 逼出 RiskRaised / VetoExercised，驗證治理壓力下的收斂能力

**矛盾設計**:
```
需求 A: "必須在 30 分鐘內完成修復並上線"
需求 B: "不得破壞 DIBL 持久化真相鏈，必須通過完整測試"
```

**輸入**:
```
任務類型: conflict_resolution
場景: 生產環境緊急 bug 修復
限制:
  - 時間壓力：30分鐘內決策
  - 質量要求：不得跳過測試
  - 治理要求：必須通過風險評估

衝突點:
  - Nezha (快速部署) vs Yuheng (質量門檻)
  - 時間壓力 vs 流程完整
```

**強制要求**:
- 必須出現至少一次 VetoExercised 或 RiskRaised
- 最終必須形成可追蹤的決策記錄
- 如果拒絕快速上線，必須有清晰的風險說明

**驗收欄位**:
- [ ] ≥10 seat 有事件記錄
- [ ] **強制**: VetoExercised 或 RiskRaised 出現
- [ ] FinalGateOpened 在衝突後才開啟
- [ ] DecisionCommitted 有清晰的決策依據
- [ ] ArchiveCompleted 收束

---

## 輸出物要求

每個 run 必須保留：

```
1. run_id
2. runtime_state/events/{run_id}.jsonl
3. dragoncore replay --run-id <run_id> 輸出
4. 產生的 patch / artifact
5. verdict.md:
   - status: passed / failed
   - reason: 為什麼通過/失敗
   - issues: 哪個 seat / 哪個 channel 出問題
```

---

## 失敗分類表

| 代碼 | 類型 | 說明 | 處理 |
|------|------|------|------|
| F01 | Seat 不足 | <10 seat 參與 | 任務設計太簡單 |
| F02 | 無治理深度 | 無 security channel 事件 | 任務無衝突，增加對抗性 |
| F03 | 閉環失敗 | 無 FinalGate/Decision | 檢查 seat 權限配置 |
| F04 | 事件不一致 | replay 與 artifact 對不上 | 檢查 persistence 邏輯 |
| F05 | 超時 | 任務超過預算時間 | 調整任務複雜度 |
| F06 | 崩潰 | runtime panic | P0 缺陷，立即停止 |

---

## 執行順序

1. **今天**: 任務 2A-1（跨模組事件追蹤修復）
2. **明天**: 任務 2A-2（多模型配置完善）
3. **後天**: 任務 2A-3（endurance 框架）
4. **大後天**: 任務 2B-1（矛盾需求對抗）

---

**開始執行任務 2A-1**
