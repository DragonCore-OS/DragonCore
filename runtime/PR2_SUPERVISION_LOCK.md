# PR-2 监督标准锁定

**锁定日期**: 2026-03-20  
**监督人**: DragonCore 负责人 / 外部审计  
**状态**: 标准已冻结，不再变更

---

## 执行链确认

四份文档已全部在 repo 中：

| 文档 | 职责 | 提交 |
|------|------|------|
| DRAGONCORE_DIRECTIVE.md | 总目标与禁止事项 | e606d85 |
| PR2_IMPLEMENTATION_PLAN.md | 实施路径与启动信号 | ba9bd69 |
| PR2_DAY3_CHECKPOINT.md | 完整闸门与判定规则 | 50aa70d |
| PR2_DAY3_CORE_EVIDENCE.md | 最小放行证据 | c85d1c6 |

---

## Day 3 过闸标准 (已锁定)

### 必须同时出现的 4 个东西

```
┌─────────────────────────────────────────────────────────────┐
│  1. 一条真实 DecisionAttributed 落盘                         │
│     (grep 到事件内容，不是只有事件名)                        │
├─────────────────────────────────────────────────────────────┤
│  2. 一条真实 EntityKpiUpdated 落盘                           │
│     (grep 到事件内容，不是只有事件名)                        │
├─────────────────────────────────────────────────────────────┤
│  3. entity attribution --decision-id ... 查到真实数据        │
│     (返回 JSON 有真实 UUID 和权重，不是 stub)                │
├─────────────────────────────────────────────────────────────┤
│  4. entity kpi --entity-id ... --period ... 查到真实数据    │
│     (返回 JSON 有真实计算值，不是硬编码)                     │
└─────────────────────────────────────────────────────────────┘
```

### 真形成的完整链条

```
创建 -> DIBL 落盘 -> CLI 查询
```

**不是**:
- ❌ struct 定义
- ❌ enum 变体
- ❌ clap help 能显示
- ❌ grep 到事件名

---

## 放行规则 (已锁定)

### ✅ 四项全有

→ **允许进 Day 4-7 接 final-gate**

### 🟡 只有落盘，没有 CLI 真数据

→ **不放行，先补查询闭环**

### 🟡 只有 CLI 壳子，没有真实事件样例

→ **不放行**

### 🔴 提前启用 KPI 阈值惩罚

→ **判偏航，必须移除**

### 🔴 破坏 v0.2.1 JSON/CSV 路径

→ **判回退修复**

---

## 监督口令

> **Day 3 不是"能编译"，而是"能拿出两条真实事件 + 两条真实查询结果"。**

---

## 检查命令锁定

```bash
# 证据 1
cat runtime_state/events/*.jsonl | grep DecisionAttributed | jq '.details' | head -1

# 证据 2  
cat runtime_state/events/*.jsonl | grep EntityKpiUpdated | jq '.details' | head -1

# 证据 3
dragoncore entity attribution --decision-id <真实id>

# 证据 4
dragoncore entity kpi --entity-id <真实id> --period 2026-03
```

---

## 下一检查点

**时间**: Day 3 结束时  
**交付物**: 4 项证据截图/日志  
**判定**: 按上述规则立即判定

---

*标准锁定: 2026-03-20*  
*不再变更*
