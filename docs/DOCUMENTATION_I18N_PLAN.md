# DragonCore 文档国际化计划 (i18n Plan)

**目标**: 所有重要文档提供中英文双版本  
**优先级**: P0 (核心) → P1 (重要) → P2 (可选)

---

## 优先级矩阵

### P0 - 核心文档 (必须)
| 中文文档 | 英文版本 | 状态 | 重要性 |
|---------|---------|------|--------|
| `README.md` | `README_EN.md` | ✅ 已有 | 项目门面 |
| `docs/19_SEATS.md` | `docs/19_SEATS_EN.md` | ❌ 待创建 | 核心治理架构 |
| `docs/USAGE_GUIDE.md` | `docs/USAGE_GUIDE_EN.md` | ❌ 待创建 | 用户使用指南 |
| `runtime/STATUS.md` | `runtime/STATUS_EN.md` | ❌ 待创建 | 项目状态 |

### P1 - 重要文档 (应该)
| 中文文档 | 英文版本 | 状态 | 重要性 |
|---------|---------|------|--------|
| `runtime/RELEASE_NOTES_v0.2.1.md` | `runtime/RELEASE_NOTES_v0.2.1_EN.md` | ❌ 待创建 | 发布说明 |
| `runtime/docs/VERIFICATION_REPORT.md` | `runtime/docs/VERIFICATION_REPORT_EN.md` | ❌ 待创建 | 验证报告 |
| `runtime/docs/WINDOWS_RUNTIME_ARCHITECTURE.md` | `..._EN.md` | ❌ 待创建 | Windows架构 |
| `runtime/docs/WINDOWS_V1_SCOPE.md` | `..._EN.md` | ❌ 待创建 | Windows范围 |
| `runtime/docs/KNOWN_GAPS.md` | `runtime/docs/KNOWN_GAPS_EN.md` | ❌ 待创建 | 已知缺陷 |

### P2 - 可选文档 (可以)
| 中文文档 | 英文版本 | 状态 | 重要性 |
|---------|---------|------|--------|
| `docs/WHY_DRAGON.md` | `docs/WHY_DRAGON_EN.md` | ❌ 待创建 | 品牌故事 |
| `docs/HUAXIA_REGISTRY.md` | `docs/HUAXIA_REGISTRY_EN.md` | ❌ 待创建 | 人物注册表 |
| `runtime/docs/PERSISTENCE_DESIGN.md` | `..._EN.md` | ❌ 待创建 | 技术设计 |
| `runtime/NEXT_MILESTONE.md` | `runtime/NEXT_MILESTONE_EN.md` | ❌ 待创建 | 里程碑规划 |

---

## 实施建议

### 方案 A: 机器翻译 + 人工校对 (快速)
1. 使用 Kimi/Claude 进行高质量翻译
2. 保留关键术语英文 (如 "19-Seat Governance", "Veto Chain")
3. 人工校对技术准确性

### 方案 B: 双语同步编写 (精确)
1. 新文档直接编写双语版本
2. 旧文档逐步补齐
3. 每次更新同步维护两个版本

**推荐**: 方案 A 快速补齐 P0/P1，后续采用方案 B 维护。

---

## 关键术语对照表 (Glossary)

| 中文 | 英文 | 备注 |
|-----|------|------|
| 龙核 | DragonCore | 产品名，不翻译 |
| 19席治理 | 19-Seat Governance | 核心概念 |
| 北斗七星 | Seven Northern Stars | 可保留中文+英文 |
| 四象 | Four Symbols | 可保留中文+英文 |
| 八仙护法 | Eight Guardians | 可保留中文+英文 |
| 否决链 | Veto Chain | 核心机制 |
| 终局裁决 | Final Gate | 保留英文术语 |
| 归档 | Archive | 通用术语 |
| 生产账本 | Production Ledger | 特定概念 |
| 华夏隐喻 | Huaxia Metaphor | 文化概念 |

---

## 下一步行动

请指定优先级：
1. **立即开始 P0** - 补齐核心文档英文版
2. **P0 + P1 同步** - 核心+重要文档一起
3. **完整方案** - P0+P1+P2 全部
