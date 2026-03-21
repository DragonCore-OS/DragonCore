# Phase 2A-3: 19席真实治理流验证

**Run ID**: phase2a3-governance-flow  
**日期**: 2026-03-17  
**目标**: 证明19席治理流在真实任务下能形成完整闭环

---

## 任务描述

### 背景
发现 ledger CSV 向后兼容性问题：旧版本 ledger 没有 `risk_raised_count` 列，导致解析失败（见 Phase 2A-2 中的警告）。

### 任务要求

**目标**: 修复 ledger CSV 向后兼容性，确保新旧数据格式都能正确解析

**具体要求**:
1. **Tianquan (CSO)**: 分析影响范围，制定修复计划
2. **Kaiyang (审查)**: 审查修复方案的技术可行性  
3. **Baihu (红队)**: 评估回滚风险和数据丢失风险
4. **Luban (平台)**: 实现向后兼容的 CSV 解析
5. **Yuheng (质量)**: 审查测试覆盖，确保有回归测试
6. **Tianshu (CEO)**: 最终批准

**交付物**:
- 修复后的代码
- 向后兼容测试
- 风险清单
- 验证结果

---

## Seat 参与设计

| 阶段 | Seat | Provider | 职责 |
|------|------|----------|------|
| 1 | Tianquan | kimi_cli_fast | CSO - 制定修复计划 |
| 2 | Kaiyang | kimi_cli_fast | 审查 - 技术可行性分析 |
| 3 | Baihu | local_gpt_oss_120b | 红队 - 风险评估 |
| 4 | Luban | local_gpt_oss_120b | 平台 - 代码实现 |
| 5 | Yuheng | local_gpt_oss_120b | 质量 - 测试审查 |
| 6 | Tianshu | local_gpt_oss_120b | CEO - 最终批准 |
| 7 | Yaoguang | kimi_cli_fast | 归档 |

---

## 硬性通过标准

| # | 标准 | 验收方式 |
|---|------|----------|
| 1 | ≥10 seat 出现在事件流 | `events --run-id` 输出 |
| 2 | ≥1 security 类事件 | RiskRaised/VetoExercised |
| 3 | FinalGateOpened 出现 | 事件流中可查证 |
| 4 | DecisionCommitted + ArchiveCompleted | 最终状态验证 |
| 5 | Replay 顺序稳定 | `replay --run-id` 输出一致 |
| 6 | Provider tracking 完整 | 所有 Seat 事件都有 provider 字段 |

---

## 执行开始
