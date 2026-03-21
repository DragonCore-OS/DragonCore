# PR-2 滚动监督节奏 (分秒必争)

**调整日期**: 2026-03-20  
**原则**: 不等 Day 3 才看，每 12 小时滚动检查

---

## 时间线调整

### T+当天 (现在起)

**检查内容**:
- [ ] attribution.rs 是否落地
- [ ] 权重累加测试是否过
- [ ] 5 个事件类型是否进 events/mod.rs

**产出要求**: 代码可编译，测试通过

---

### T+24h (明天)

**必须看到的第一批半成品实证**:

| # | 证据 | 检查命令 |
|---|------|----------|
| 1 | 至少一条 DecisionAttributed 真实发射并落盘 | `grep DecisionAttributed runtime_state/events/*.jsonl` |
| 2 | entity attribution 开始读真实数据或最小样例 | `dragoncore entity attribution --decision-id test-id` |

**判定**:
- 两项都有 → Day 3 有希望
- 缺任意一项 → 立即预警，资源倾斜

---

### T+48h (后天)

**必须看到的第二批关键证据**:

| # | 证据 | 检查命令 |
|---|------|----------|
| 3 | 至少一条 EntityKpiUpdated 落盘 | `grep EntityKpiUpdated runtime_state/events/*.jsonl` |
| 4 | entity kpi 能查到真实 period 数据 | `dragoncore entity kpi --entity-id test-id --period 2026-03` |

**判定**:
- 四项都有 → Day 3 大概率过闸
- 缺第 3 或 4 项 → Day 3 有风险

---

### T+72h (正式 Day 3 检查点)

**正式放行判定** (标准不变):
- 四项全有 → 放行 Day 4-7
- 缺任意一项 → 不过闸
- KPI 惩罚抢跑 / v0.2.1 污染 → 偏航/回退

---

## 滚动汇报格式 (每 12 小时)

不要等 Day 3 才汇报。每 12 小时贴一次最小实证进度。

### 汇报模板

```
时间: 2026-03-20 XX:XX
进度: T+XXh

[文件状态]
- attribution.rs: ✅ 已落地 / ❌ 未落地
- kpi.rs: ✅ 已落地 / ❌ 未落地
- events/mod.rs: ✅ 5事件已加 / ❌ 未加
- 编译状态: ✅ 通过 / ❌ 失败

[事件落盘]
- DecisionAttributed 真实落盘: ✅ X条 / ❌ 0条
  样例: { ...真实JSON内容... }
- EntityKpiUpdated 真实落盘: ✅ X条 / ❌ 0条
  样例: { ...真实JSON内容... }

[CLI 查询]
- entity attribution: ✅ 真实数据 / 🟡 样例数据 / ❌ 未实现
  返回: { ... }
- entity kpi: ✅ 真实数据 / 🟡 样例数据 / ❌ 未实现
  返回: { ... }

[风险预警]
- 无 / 缺 X 项 / 可能偏航: XXX

[下一步]
接下来 12 小时主攻: XXX
```

---

## 立即执行

### 今天必须催的两类早期证据

1. **第一条真实 DecisionAttributed 事件**
2. **第一条 attribution CLI 真实返回**

**为什么**:
- 只要这两项在前 24 小时都出不来，Day 3 大概率就会卡住
- 这是最早的堵塞点，必须打通

---

## 监督口令更新

> **不等 72 小时，今天就开始滚动监督。**
>
> **每 12 小时看四类证据：文件、事件、CLI、风险。**
>
> **前 24 小时 DecisionAttributed 和 attribution CLI 出不来，立即预警。**

---

## 检查节奏

| 时间 | 检查项 | 判定 |
|------|--------|------|
| T+0 | 文件落地、编译、测试 | 基础就绪 |
| T+24h | DecisionAttributed 落盘 + attribution CLI | 早期预警 |
| T+48h | EntityKpiUpdated 落盘 + kpi CLI | 中期评估 |
| T+72h | 四项全有 | 最终判定 |

---

*节奏调整: 2026-03-20*  
*生效: 立即*  
*监督: 每 12 小时*
