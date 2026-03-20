# DragonCore Meeting Protocol v0.1 冻结声明

**冻结日期**: 2026-03-20  
**冻结版本**: v0.1  
**预计解冻条件**: 联合 Endurance 验证完成 (PASS 或 CONDITIONAL PASS)

---

## 冻结范围

### 🚫 禁止事项

在联合 Endurance 验证完成前，**禁止**以下操作：

- 新增会议协议功能
- 修改会议状态机核心逻辑
- 调整 Stance 收敛规则
- 修改 SmartModerator 调度算法
- 调整 BiasStyle 行为参数
- 新增 CLI 命令
- 修改 DIBL 事件结构

### ✅ 允许事项

以下修改**无需解冻**：

| 类型 | 示例 |
|------|------|
| 缺陷修复 | 修复已发现的 bug |
| 文档纠正 | 修正文档错误、补充说明 |
| 观测增强 | 增加日志、metrics、监控点 |
| 测试增强 | 增加测试覆盖、修复 flaky test |
| 配置调整 | 验证相关的配置参数 |

---

## 冻结原因

**核心目标**: 验证会议协议层是否破坏原有长时稳定性

在跑联合 Endurance 前，任何功能增加都会：
1. 引入新的不稳定因素
2. 模糊验证结论（问题来自新功能还是原有代码？）
3. 浪费验证资源（需要重新跑 4 小时）

**冻结是为了保护验证口径的纯净性。**

---

## 验证计划

### 验证目标

Meeting Protocol + 19 席 + 双 provider + DIBL + 4h 无人工干预

### 成功标准

#### 🟢 PASS（全部满足）

- [ ] ≥4h 无崩溃运行
- [ ] 0 次人工干预
- [ ] 无 state/event/ledger 脱节
- [ ] 会议能持续收敛
- [ ] 行为人格无异常漂移
- [ ] provider 路由稳定

#### 🟡 CONDITIONAL PASS（满足大部分）

- [ ] ≥4h 运行但有可定位的小缺口
- [ ] 个别轮次收敛过慢
- [ ] replay 可读性下降
- [ ] 某类席位过度沉默/过度活跃

#### 🔴 FAIL（任一出现）

- [ ] 死锁
- [ ] 失控循环
- [ ] authority 被绕过
- [ ] event/ledger 丢失
- [ ] provider 漂移
- [ ] 长时运行崩溃

---

## 解冻条件

### 自动解冻

| 验证结果 | 解冻方式 |
|----------|----------|
| **PASS** | 自动解冻，可开始 v0.2 规划 |
| **CONDITIONAL PASS** | 评估缺口影响，决定是否先修复再解冻 |
| **FAIL** | 修复问题后重新冻结，再次验证 |

### 解冻审批

**审批人**: 用户 (Human)  
**解冻触发**: 验证完成 + 结论明确

---

## 当前状态

```
Meeting Protocol v0.1: 功能级验证通过 ✅
代码冻结: 已生效 🚫
联合验证: 待执行 🔜
```

---

## 相关文档

- [MEETING_PROTOCOL_ENDURANCE_PLAN.md](./MEETING_PROTOCOL_ENDURANCE_PLAN.md) - 联合验证详细计划
- [STATUS.md](./STATUS.md) - 系统状态
- [VERIFICATION_RESULTS.md](./VERIFICATION_RESULTS.md) - 验证结果

---

*冻结声明生效时间: 2026-03-20*  
*声明版本: v0.1-freeze*
