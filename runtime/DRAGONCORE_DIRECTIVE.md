# DragonCore 小组完整指令

**版本**: v0.1  
**日期**: 2026-03-20  
**状态**: 执行中

---

## 总目标

把现有大模型的潜在能力，用 19 席治理、会议协议、责任制度、工具权限、验证闭环，尽可能稳定地逼出来。

**不是**去研究模型本体上限，**而是**把"已有知识与能力"通过组织化、制度化、可追责的方式，压成更高质量的行动结果。

---

## 当前已知底座

| 组件 | 状态 |
|------|------|
| 19 席 | 正式治理席位，有 authority、conflict、faction、communication style |
| 运行时核心治理链 | 正式动作链：init / run / execute / veto / final-gate / archive / terminate |
| DIBL | 事件结构、4 个 channel、persist-first 边界已固定 |
| Meeting Protocol v0.1 | P0-P3 已完成，19 项测试通过 |
| AI Entity Responsibility PR-1 | ✅ 已完成（身份与状态机） |

---

## 当前阶段定位

继续把下面三层做实：

| 层级 | 内容 |
|------|------|
| **治理层** | 19 席 + Meeting Protocol |
| **责任层** | AI Entity Responsibility |
| **执行层** | 工具权限、回滚、审计、长时稳定 |

---

## 当前优先级

### P0: AI Entity Responsibility v0.1 实现闭环

#### PR-2: 责任归因与 KPI

**必须实现：**
- [ ] DecisionAttribution
- [ ] 三层 KPI:
  - [ ] DecisionQuality (45%)
  - [ ] GovernanceConduct (30%)
  - [ ] OrganizationalDuty (25%)
- [ ] 责任权重计算
- [ ] 决策后回写
- [ ] CLI: `entity kpi` / `entity attribution`

**DIBL 事件扩展：**
- EntityKpiUpdated
- DecisionAttributed
- EntityWarned
- EntityLimited
- EntityUnderReview

**最低验收：**
- 每个关键治理动作都可归因到主体
- KPI 可回放
- KPI 变化与事件链一致
- 不允许无归因的关键决策通过 final gate

#### PR-3: 提醒与考核

**必须实现：**
- [ ] PreDecisionReminder
- [ ] PostDecisionWriteback
- [ ] MonthlyAssessmentReminder
- [ ] 周期考核输出
- [ ] 基于 KPI 的建议性状态迁移:
  - active → limited
  - active → under_review
  - limited → demoted

**最低验收：**
- 关键决策前能注入责任提醒
- 决策后能回写主体档案
- 月度考核能生成结构化结果
- 可以产生"建议升降级"，但**先不要自动执行最终惩罚**

---

### 并行任务: Meeting Protocol 联合 endurance

**当前状态**: Meeting Protocol v0.1 已完成 P0-P3

**目标**: 重新跑一轮正确接入会议层的长时联合验证

**必须回答的唯一问题：**
> Meeting Protocol v0.1 上线后，是否破坏了 DragonCore 原有长时稳定性？

**验证要求：**
- [ ] 会议协议层真实被调用
- [ ] ≥4h 运行
- [ ] 无人工干预
- [ ] 无死锁
- [ ] provider 稳定
- [ ] replay 可用
- [ ] 最好会议层 DIBL 事件也接通

**结论标准：**
- 会议功能层通过但会议层 DIBL 未接通 → **CONDITIONAL PASS**
- 会议层 DIBL 也接通 → **PASS**

---

## 研究题目

### 题目 A: 如何把模型潜在能力逼出来

**比较维度：**
- 单轮问答
- 线性 seat 执行
- Meeting Protocol
- Meeting Protocol + KPI/责任制度
- Meeting Protocol + 工具权限/回滚

**评估指标：**
- 任务正确率
- 风险命中率
- 回滚率
- 交付质量
- 长时稳定性

### 题目 B: 责任制度会不会改善治理质量

**比较维度：**
- 无 KPI / 无责任归因
- 有 KPI / 有责任归因
- 有升降级建议
- 有提醒与考核

**评估指标：**
- 低价值发言是否下降
- 漏报风险是否下降
- 关键席位失职是否减少
- 共识形成速度是否改善

### 题目 C: 工具与权限如何提升上限

**研究内容：**
- 给多少权限最有效
- 哪些工具缺失限制了上限
- 哪些工具必须自研
- 如何保证可回滚、可审计、可隔离

---

## 禁止事项

当前阶段**不要做**：
- ❌ 大规模 UI
- ❌ 多节点分布式
- ❌ 过度扩展 v0.2 会议新特性
- ❌ 为了"更像人"去做表演型人格
- ❌ 在未验证前把系统描述成"全面生产就绪"

---

## 交付要求

后续所有交付必须带：

| 项 | 说明 |
|----|------|
| 功能说明 | 实现了什么 |
| 验收标准 | 怎么算通过 |
| DIBL 事件影响 | 新增/修改了哪些事件 |
| replay 影响 | 对回放的影响 |
| 测试清单 | 哪些测试覆盖 |
| 边界说明 | 限制和约束 |

---

## 一句话定调

> **DragonCore 负责把能力逼出来，把组织做实，把后果绑上去。**

---

*指令发布: 2026-03-20*  
*执行状态: PR-2 准备中*
