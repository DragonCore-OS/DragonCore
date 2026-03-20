# DragonCore Meeting Protocol v0.1 - P0 實施報告

**狀態**: ✅ P0 最小可用會議殼已完成  
**日期**: 2026-03-19  
**版本**: v0.1-p0

---

## 已實現內容 (P0)

### 1. 核心數據結構

| 結構 | 文件 | 狀態 |
|------|------|------|
| `MeetingSession` | `src/meeting/mod.rs` | ✅ |
| `MeetingPhase` 狀態機 | `src/meeting/mod.rs` | ✅ |
| `PresenceState` | `src/meeting/mod.rs` | ✅ |
| `SpeakRequest` | `src/meeting/mod.rs` | ✅ |
| `SpeakIntent` | `src/meeting/mod.rs` | ✅ |
| `SeatStance` | `src/meeting/mod.rs` | ✅ |
| `DiscussionRound` | `src/meeting/mod.rs` | ✅ |
| `MeetingTurn` | `src/meeting/mod.rs` | ✅ |
| `ResolutionDraft` | `src/meeting/mod.rs` | ✅ |
| `RecommendedAction` | `src/meeting/mod.rs` | ✅ |
| `MeetingManager` | `src/meeting/mod.rs` | ✅ |

### 2. CLI 命令

| 命令 | 狀態 | 說明 |
|------|------|------|
| `meeting open` | ✅ | 創建會議會話 |
| `meeting assemble` | ✅ | 集合檢查 |
| `meeting roll-call` | ✅ | 點名確認 |
| `meeting topic-lock` | ✅ | 議題鎖定 |
| `meeting request-speak` | ✅ | 申請發言 |
| `meeting schedule-round` | ✅ | 安排輪次 |
| `meeting speak` | ✅ | 執行發言 |
| `meeting force-speak` | ✅ | 強制點名 |
| `meeting update-stance` | ✅ | 更新立場 |
| `meeting challenge-window` | ✅ | 開啟挑戰窗口 |
| `meeting draft-resolution` | ✅ | 草擬決議 |
| `meeting commit-action` | ✅ | 提交治理動作 |
| `meeting status` | ✅ | 查看狀態 |

### 3. 核心算法

| 算法 | 狀態 | 說明 |
|------|------|------|
| `speak_score()` | ✅ | 發言評分算法 (role_relevance*0.30 + urgency*0.20 + confidence*0.15 + novelty*0.20 + conflict_need*0.15) |

---

## P0 功能演示

```bash
# 1. 開啟會議
dragoncore meeting open \
  --run-id "meeting-001" \
  --topic "Database Migration Risk Assessment" \
  --moderator "Tianshu"

# 2. 點名確認
dragoncore meeting roll-call --run-id "meeting-001"

# 3. 申請發言 (Baihu 風險警報)
dragoncore meeting request-speak \
  --run-id "meeting-001" \
  --seat "Baihu" \
  --intent "risk-alert" \
  --confidence 0.85 \
  --urgency 0.90 \
  --reason "Data corruption risk during migration"

# 4. 安排討論輪次
dragoncore meeting schedule-round \
  --run-id "meeting-001" \
  --round 1 \
  --speakers "Tianquan,Yuheng,Baihu"

# 5. 強制點名 (Xuanwu 必須發言)
dragoncore meeting force-speak \
  --run-id "meeting-001" \
  --seat "Xuanwu" \
  --reason "Security review required"

# 6. 更新立場
dragoncore meeting update-stance \
  --run-id "meeting-001" \
  --seat "Yuheng" \
  --position "Conditional support with rollback plan" \
  --confidence 0.72 \
  --challenges "Nezha"

# 7. 開啟挑戰窗口
dragoncore meeting challenge-window --run-id "meeting-001"

# 8. 草擬決議
dragoncore meeting draft-resolution \
  --run-id "meeting-001" \
  --seat "Tianshu" \
  --summary "Proceed with staged migration and 5min rollback" \
  --action "open-final-gate"

# 9. 提交治理動作
dragoncore meeting commit-action \
  --run-id "meeting-001" \
  --action "open-final-gate"
```

---

## 設計原則落實

### ✅ 討論自由，權力不自由

- 所有席位可申請發言、更新立場、支持/挑戰他人
- 僅現有有權席位可 veto/final-gate/terminate/archive

### ✅ Persist first, broadcast second

- MeetingManager 先保存 state 到 JSON
- 再 emit DIBL 事件

### ✅ 會議是 run 的子狀態

- `MeetingSession` 包含 `run_id`
- 與現有 governance run 綁定

---

## P1-P3 待實現

### P1: 立場系統
- [ ] `supports/challenges` 關係追蹤
- [ ] `confidence delta` 計算
- [ ] 立場可視化

### P2: 智能主持
- [ ] 自動 speaker 排序算法
- [ ] Conflict coverage 檢查
- [ ] 缺席角色自動點名
- [ ] 自動開 challenge window

### P3: 人格增強
- [ ] `SeatPersonaProfile`
- [ ] `BiasStyle` 行為差異
- [ ] 更自然的發言節奏

---

## 與現有系統集成

```
DragonCore Runtime
├── Governance (existing)
│   ├── 19 Seats
│   ├── Veto/FinalGate/Archive/Terminate
│   └── DIBL Events
├── Meeting Protocol (NEW)
│   ├── MeetingSession
│   ├── SpeakRequest scoring
│   └── ChallengeWindow
└── Integration
    ├── meeting -> governance action mapping
    └── meeting events -> DIBL
```

---

## 驗證結果

| 測試項 | 結果 |
|--------|------|
| CLI 命令可用 | ✅ |
| 參數解析正確 | ✅ |
| Help 輸出完整 | ✅ |
| 編譯無錯誤 | ✅ |

---

## 下一步

1. **P1 實現**: 立場系統完善
2. **集成測試**: 與現有 governance runtime 聯調
3. **DIBL 事件擴展**: 會議層事件進入 event stream

---

**Meeting Protocol v0.1 P0 已完成，可進入 P1 階段。**
