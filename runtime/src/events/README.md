# DIBL - DragonCore Internal Broadcast Layer

DIBL 是 DragonCore 的內部事件廣播層，用於治理運行時的可觀測性和審計。

## 核心原則

1. **JSON/Ledger 是真相源** - 廣播只是派生視圖
2. **先持久化、後廣播** - 事件必須先寫入 JSONL 存儲才能發送
3. **分層可見性** - `internal` | `operator_visible` | `exportable`

## 事件結構 (與 AXI 對齊)

```rust
pub struct GovernanceEvent {
    pub event_id: Uuid,
    pub run_id: String,
    pub seat_id: Option<String>,
    pub channel: EventChannel,      // Control/Ops/Security/Research
    pub event_type: GovernanceEventType,
    pub scope: EventScope,          // Internal/OperatorVisible/Exportable
    pub severity: Severity,         // Info/Warn/Critical
    pub summary: String,
    pub details_ref: Option<String>,
    pub artifact_refs: Vec<String>,
    pub created_at: DateTime<Utc>,
    // Correlation context (flattened)
    pub correlation_id: Option<String>,
    pub parent_event_id: Option<Uuid>,
    pub actor: String,              // 原 triggered_by
    pub trigger_context: Option<String>,
}
```

## 事件通道

| 通道 | 用途 | 事件類型 |
|------|------|----------|
| `Control` | 治理控制流 | RunCreated, SeatStarted, FinalGateOpened, DecisionCommitted |
| `Ops` | 運行時操作 | ArchiveCompleted |
| `Security` | 安全與風險 | VetoExercised, TerminateTriggered, RollbackTriggered |
| `Research` | 研究輸出 | SeatCompleted, RiskRaised |

## 事件發射點

已在以下關鍵操作點接入事件發射：

| 操作 | 事件 | Channel | Scope | Actor |
|------|------|---------|-------|-------|
| `init_run()` | RunCreated | Control | OperatorVisible | system |
| `execute_seat()` | SeatStarted | Control | Internal | {seat} |
| `execute_seat()` | SeatCompleted | Research | Internal | {seat} |
| `exercise_veto()` | VetoExercised | Security | OperatorVisible | {seat} |
| `final_gate()` | FinalGateOpened | Control | OperatorVisible | Tianshu |
| `final_gate()` | DecisionCommitted | Control | Exportable | Tianshu |
| `archive_run()` | ArchiveCompleted | Ops | OperatorVisible | {seat} |
| `terminate_run()` | TerminateTriggered | Security | OperatorVisible | {seat} |

## 存儲格式

```
runtime_state/events/{run_id}.jsonl
```

每行一個 JSON 事件對象，可追加、可回放、可審計。

樣本文件: `test_vectors/dragoncore_sample.jsonl`

## Operator Projection

最小摘要視圖，不暴露內部細節：

```rust
pub struct OperatorProjection {
    pub run_id: String,
    pub current_phase: String,
    pub current_seat: Option<String>,
    pub last_significant_event: Option<String>,
    pub open_risks: Vec<String>,
    pub veto_count: u32,
    pub terminate_flag: bool,
    pub final_outcome: Option<String>,
}
```

## 與 AXI 的關係

DIBL 與 AXI 完全對齊：

| 項目 | 狀態 |
|------|------|
| Scope 三層 | ✅ Internal/OperatorVisible/Exportable |
| Channel 四通道 | ✅ Control/Ops/Security/Research |
| 存儲格式 | ✅ JSONL |
| 文件命名 | ✅ {run_id}.jsonl |
| CorrelationContext | ✅ actor, correlation_id, parent_event_id |

DIBL **不是** 聊天室產品，而是治理運行時的事件層。
