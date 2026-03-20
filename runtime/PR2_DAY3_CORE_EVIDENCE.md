# PR-2 Day 3 核心证据标准

**发布日期**: 2026-03-20  
**优先级**: P0 (最高)  
**原则**: 不被"文件数量"分散注意力，只看三项核心证据

---

## 核心原则

> **Day 3 真正过闸的标准不是"编译通过"或"文件存在"，而是：**
>
> **归因/KPI 事件已进入可查询数据流**

---

## 三项核心证据 (必须全部呈现)

### 证据 1: DecisionAttributed 真实落盘样例

**要求**: 必须是真实写入存储的事件，不是 mock 或单元测试数据。

**呈现格式**:
```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "event_type": "DecisionAttributed",
  "timestamp": "2026-03-23T14:32:10Z",
  "actor": "entity_system",
  "run_id": "decision-1773999999",
  "details": {
    "decision_id": "decision-1773999999",
    "primary_owner": "entity-uuid-1",
    "approving_authority": "entity-uuid-2",
    "supporting": ["entity-uuid-3", "entity-uuid-4"],
    "challenging": [],
    "responsibility_weights": {
      "entity-uuid-1": 0.40,
      "entity-uuid-2": 0.30,
      "entity-uuid-3": 0.15,
      "entity-uuid-4": 0.15
    }
  }
}
```

**验证命令**:
```bash
# 必须能在事件存储中找到
cat runtime_state/events/*.jsonl | grep DecisionAttributed | head -1
```

---

### 证据 2: EntityKpiUpdated 真实落盘样例

**要求**: 必须是基于真实事件流计算的 KPI，不是硬编码值。

**呈现格式**:
```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440001",
  "event_type": "EntityKpiUpdated",
  "timestamp": "2026-03-23T14:35:22Z",
  "actor": "entity-uuid-1",
  "details": {
    "entity_id": "entity-uuid-1",
    "period": "2026-03",
    "decision_quality": 75.0,
    "governance_conduct": 68.0,
    "organizational_duty": 82.0,
    "total_score": 74.35,
    "details": {
      "proposal_count": 5,
      "adopted_count": 4,
      "success_count": 3,
      "rollback_count": 1
    }
  }
}
```

**验证命令**:
```bash
cat runtime_state/events/*.jsonl | grep EntityKpiUpdated | head -1
```

---

### 证据 3: CLI 实查结果 (两条命令)

#### 命令 A: entity attribution

**执行**:
```bash
dragoncore entity attribution --decision-id <真实decision-id>
```

**必须返回**:
```json
{
  "decision_id": "decision-1773999999",
  "primary_owner": "entity-uuid-1",
  "approving_authority": "entity-uuid-2",
  "supporting": ["entity-uuid-3", "entity-uuid-4"],
  "responsibility_weights": {
    "entity-uuid-1": 0.40,
    "entity-uuid-2": 0.30,
    "entity-uuid-3": 0.15,
    "entity-uuid-4": 0.15
  },
  "total_weight": 1.0
}
```

**禁止**: 返回 "not found" 或空 stub。

---

#### 命令 B: entity kpi

**执行**:
```bash
dragoncore entity kpi --entity-id <真实entity-id> --period 2026-03
```

**必须返回**:
```json
{
  "entity_id": "entity-uuid-1",
  "period": "2026-03",
  "decision_quality": 75.0,
  "governance_conduct": 68.0,
  "organizational_duty": 82.0,
  "total_score": 74.35,
  "calculated_at": "2026-03-23T14:35:22Z"
}
```

**禁止**: 返回 "not found" 或硬编码值。

---

## Day 4-7 启动前提

### 允许开始接 final-gate 的条件

**必须同时满足**:
1. ✅ 三项核心证据全部呈现
2. ✅ 事件可查询 (不是只能 grep 文件，CLI 能读到)
3. ✅ 数据流完整: 创建 → 落盘 → 查询

### 禁止抢跑

**如果三项证据有任意一项缺失**:
- ❌ 不准开始 Day 4-7
- ❌ 不准接 final-gate
- ✅ 必须先补查询闭环

---

## 快速检查脚本

```bash
#!/bin/bash
# Day 3 核心证据检查脚本

echo "=== Day 3 核心证据检查 ==="

# 检查 1: DecisionAttributed 事件
ATTR_COUNT=$(cat runtime_state/events/*.jsonl 2>/dev/null | grep -c DecisionAttributed || echo "0")
echo "[1/3] DecisionAttributed 事件数: $ATTR_COUNT"
if [ "$ATTR_COUNT" -eq 0 ]; then
    echo "  ❌ FAIL: 无 DecisionAttributed 事件"
    exit 1
fi

# 检查 2: EntityKpiUpdated 事件
KPI_COUNT=$(cat runtime_state/events/*.jsonl 2>/dev/null | grep -c EntityKpiUpdated || echo "0")
echo "[2/3] EntityKpiUpdated 事件数: $KPI_COUNT"
if [ "$KPI_COUNT" -eq 0 ]; then
    echo "  ❌ FAIL: 无 EntityKpiUpdated 事件"
    exit 1
fi

# 检查 3: CLI 可用 (需要实际运行)
echo "[3/3] CLI 命令可用性"
echo "  请手动执行:"
echo "    dragoncore entity attribution --decision-id <id>"
echo "    dragoncore entity kpi --entity-id <id> --period 2026-03"
echo "  并确认返回真实数据而非 stub"

echo ""
echo "=== 检查结果 ==="
echo "✅ 证据 1: DecisionAttributed 落盘 - 通过 ($ATTR_COUNT 条)"
echo "✅ 证据 2: EntityKpiUpdated 落盘 - 通过 ($KPI_COUNT 条)"
echo "⏳ 证据 3: CLI 实查 - 需手动验证"
```

---

## 与 PR2_DAY3_CHECKPOINT.md 的关系

| 文档 | 作用 |
|------|------|
| `PR2_DAY3_CHECKPOINT.md` | 完整检查清单 (6 条标准 + 7 项报告格式) |
| `PR2_DAY3_CORE_EVIDENCE.md` | **核心浓缩**: 只看三项证据，不被分散注意力 |

**使用建议**:
- **快速判断**: 看本文档 (三项证据)
- **详细审计**: 看 CHECKPOINT 文档 (完整 6 条)

---

## 一句话总结

> **Day 3 过闸 = 能展示一条真实的 DecisionAttributed + 一条真实的 EntityKpiUpdated + 两条 CLI 能查到真实数据。**
>
> **少任何一项，都不准进入 Day 4-7。**

---

*标准发布: 2026-03-20*  
*生效: Day 3 检查时刻*  
*监督: DragonCore 负责人 / 外部审计*
