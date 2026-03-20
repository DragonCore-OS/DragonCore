# DragonCore Meeting Protocol v0.1 - P2 驗收報告

**版本**: v0.1-p2  
**日期**: 2026-03-20  
**狀態**: ✅ **P2 智能主持驗收通過**

---

## P2 目標回顧

**P2 目標**: 讓下一位發言人選擇，從「人工指定/簡單排序」升級成「覆蓋驅動的主持調度」

---

## P2 必做 4 件事驗收

### 1. ✅ Conflict Coverage 檢查

**已實現**:
- `RoleCategory` 枚舉：8 種角色類型（Risk, Execution, Resource, Audit, Strategy, Stability, Review, Innovation）
- `CoverageChecker::check_coverage()`：檢查當前討論的角色覆蓋
- `criticality()`：角色關鍵程度評分（Risk=10 最高）
- `CoverageReport`：完整覆蓋報告（已覆蓋/缺失角色、覆蓋率、建議）

**可識別缺失**:
- ✅ 風險席有沒有說話
- ✅ 執行席有沒有說話
- ✅ 資源席有沒有評估
- ✅ 審計席有沒有確認
- ✅ 戰略席有沒有規劃

### 2. ✅ Speaker Selection 升級（6因素模型）

**已實現因素**:
```rust
pub struct SpeakerSelectionFactors {
    pub relevance: f32,              // 議題相關性 (25%)
    pub urgency: f32,                // 緊急程度 (20%)
    pub novelty: f32,                // 新信息量 (20%)
    pub conflict_coverage: f32,      // 衝突覆蓋貢獻 (20%)
    pub underrepresented_role: f32,  // 角色代表性不足補償 (15%)
    pub repeated_dominance_penalty: f32, // 連續發言懲罰
}
```

**SmartModerator::select_next_speaker()**:
- 計算6因素分數
- 綜合排序
- 生成可解釋的理由
- 更新發言次數追踪

### 3. ✅ 自動 Challenge Window 觸發

**觸發條件**:
```rust
pub fn should_auto_challenge_window(...) -> Option<String> {
    // 條件1: 高共識但缺少風險視角
    if coverage_ratio > 0.8 && missing_roles.contains(Risk) {
        return Some("High consensus but missing risk perspective");
    }
    
    // 條件2: 執行席已發言但風險席未發言
    if exec_spoken && risk_missing {
        return Some("Execution present but risk missing");
    }
    
    // 條件3: 最終決議前反方聲音不足
    if phase == ResolutionDraft && challenges_count < 1 {
        return Some("Insufficient challenge before resolution");
    }
}
```

### 4. ✅ Dead Floor 處理

**處理策略**:
```rust
pub enum DeadFloorAction {
    ForceSpeak(String),   // 自動點名缺位關鍵席
    SuggestResolution,    // 建議進入 resolution
    Wait,                 // 繼續等待
}
```

**處理邏輯**:
- 如果還有缺位的關鍵席 → `ForceSpeak`
- 如果覆蓋足夠且無新信息 → `SuggestResolution`
- 否則 → `Wait`

---

## P2 驗收標準檢查

| # | 標準 | 驗證 | 結果 |
|---|------|------|------|
| 1 | 系統能識別缺失的關鍵角色覆蓋 | `test_coverage_check_identifies_missing_roles` | ✅ |
| 2 | 不會連續偏向同一類席位 | `test_no_continuous_bias_to_same_role` | ✅ |
| 3 | 高共識但低挑戰場景下主動補 challenge coverage | `test_auto_challenge_window_high_consensus_low_challenge` | ✅ |
| 4 | 無人申請發言時不會死鎖 | `test_dead_floor_no_deadlock` | ✅ |
| 5 | 自動排序結果可解釋 | `test_ranking_is_explainable` | ✅ |

---

## P2 集成測試

### ✅ Test 1: 覆蓋檢查識別缺失角色
```
場景: 只有執行席發言
檢查: 應識別缺少風險視角
結果: ✅ PASS
```

### ✅ Test 2: 不連續偏向同一角色
```
場景: 多個執行席請求發言
檢查: 第二個執行席應有懲罰分數
結果: ✅ PASS
```

### ✅ Test 3: 自動 challenge window
```
場景: 高共識但只有執行席
檢查: 自動建議打開 challenge window
結果: ✅ PASS
```

### ✅ Test 4: Dead floor 不鎖死
```
場景: 無人申請發言
檢查: 自動點名缺位關鍵席
結果: ✅ PASS
```

### ✅ Test 5: 排序可解釋
```
場景: 發言請求排序
檢查: 每個結果有解釋，6因素可追踪
結果: ✅ PASS
```

---

## P2 新增代碼統計

| 組件 | 行數 | 說明 |
|------|------|------|
| `RoleCategory` | ~40 行 | 8 種角色類型 |
| `CoverageChecker` | ~60 行 | 覆蓋檢查 |
| `SpeakerSelectionFactors` | ~20 行 | 6因素模型 |
| `SmartModerator` | ~150 行 | 智能主持核心 |
| `ExplainableSpeakerRanking` | ~10 行 | 可解釋排序結果 |
| P2 測試 | ~100 行 | 5 個集成測試 |
| **P2 總計** | **~380 行** | |

---

## Meeting Protocol v0.1 總結

| 階段 | 狀態 | 核心交付 |
|------|------|----------|
| **P0** | ✅ 完成 | 會議基礎結構、13個 CLI 命令 |
| **P1** | ✅ 完成 | 立場系統：可追踪、可回放、可收束 |
| **P2** | ✅ **完成** | **智能主持：Coverage-aware scheduler** |
| **P3** | 待續 | 行為人格（BiasStyle、偏好、謹慎度） |

---

## 當前狀態評估

### 可以說的話 ✅

> "DragonCore 已從線性 seat 執行器，演進為具備會議協議層的治理操作系統原型。"

**已具備**:
- ✅ 會議能開（P0）
- ✅ 發言能申請（P0）
- ✅ 立場能變（P1）
- ✅ 衝突能收束（P1）
- ✅ **誰該下一位發言有根據（P2）**
- ✅ **自動補充缺失視角（P2）**
- ✅ **不會死鎖（P2）**

### 還不能說的話 ❌

> "完整會議操作系統已成熟"

**還差**:
- ❌ P3 行為人格（不同席位行為風格差異）

---

## 給 Kimi CLI 的指令

**開始 P3，但不要做聊天式 AI 主持人；先做行為人格：**

1. `BiasStyle` 影響 `speak_request` 概率
2. `caution_level` 影響 `raise risk` 頻率
3. `challenge_tendency` 影響反駁概率
4. `verbosity_preference` 影響發言長度

**先做行為人格，不做表演人格。**

---

## 結論

**Meeting Protocol v0.1 P2 驗收通過！**

系統已從「可交互會議殼」（P0）、「可追踪協商系統」（P1），升級為：

> **「會智能調度的會議操作系統」（P2）**

下一步 P3：行為人格，讓不同席位有差異化的行為風格。

---

*報告生成時間: 2026-03-20*
