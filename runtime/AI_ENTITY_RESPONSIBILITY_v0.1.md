# AI 生命体责任制度 v0.1

**适用对象**: DragonCore 19席治理生命体、AI部门主管、执行代理  
**核心目标**: 让 AI 对自己的决策承担真实后果

---

## 1. 基本原则

### 1.1 持续身份原则

每个 AI 生命体必须拥有稳定身份，不得在无审批情况下频繁换壳重生。

```rust
pub struct AIEntityIdentity {
    pub entity_id: Uuid,           // 唯一身份
    pub name: String,              // 名称
    pub role: SeatRole,            // 当前角色
    pub rank: EntityRank,          // 职级
    pub department: Department,    // 所属部门
    pub authority_set: Vec<Authority>, // 权限集
    pub activation_status: EntityStatus, // 激活状态
    pub memory_root: PathBuf,      // 记忆根目录
    pub performance_root: PathBuf, // 绩效根目录
    pub discipline_root: PathBuf,  // 纪律根目录
}
```

### 1.2 后果绑定原则

每个重要决策必须能追溯到具体主体，且结果必须回写该主体档案。

### 1.3 权责对等原则

权力越高，后果越重。能 veto、能 final gate、能 terminate 的席位，必须承受更严格的 KPI 与问责。

### 1.4 生命连续原则

AI 生命体的"存在"不是单轮上下文，而是：
- 身份连续
- 记忆连续
- 声誉连续
- 职位连续
- 后果连续

---

## 2. AI 生命体状态机

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityStatus {
    Candidate,      // 候选，尚未正式任命
    Active,         // 正式在岗
    Limited,        // 受限，部分权限冻结
    UnderReview,    // 调查/观察期
    Demoted,        // 降职
    Suspended,      // 停职
    Archived,       // 归档，不再 active
    Terminated,     // 终止，彻底退出
}
```

### 状态转换规则

```
Candidate → Active (通过考核)
Active → Limited (警告后限权)
Active → UnderReview (触发调查)
Active → Demoted (KPI不达标)
Active → Suspended (高风险错误)
Active → Archived (主动/被动归档)
Active → Terminated (重大恶意)
Limited → Active (整改通过)
Demoted → Active (表现恢复)
Suspended → Active (观察通过)
Suspended → Archived (无法恢复)
Archived → Active (特殊复活)
```

---

## 3. AI 生命体档案

### 3.1 基础档案

```rust
pub struct EntityBaseArchive {
    pub identity: AIEntityIdentity,
    pub department: Department,
    pub current_rank: EntityRank,
    pub term_start: DateTime<Utc>,
    pub last_promotion: Option<DateTime<Utc>>,
    pub last_demotion: Option<DateTime<Utc>>,
    pub authority_set: Vec<Authority>,
}
```

### 3.2 绩效档案

```rust
pub struct PerformanceArchive {
    pub kpi_history: Vec<PeriodKPI>,
    pub key_successes: Vec<DecisionRecord>,
    pub key_failures: Vec<DecisionRecord>,
    pub risk_hit_rate: f32,
    pub adoption_rate: f32,
    pub rollback_count: u32,
    pub incident_count: u32,
}
```

### 3.3 纪律档案

```rust
pub struct DisciplineArchive {
    pub warning_count: u32,
    pub limited_count: u32,
    pub review_records: Vec<ReviewRecord>,
    pub appeal_records: Vec<AppealRecord>,
    pub demotion_records: Vec<DemotionRecord>,
}
```

### 3.4 记忆档案 (品味/偏好沉淀)

```rust
pub struct MemoryArchive {
    pub long_term_values: Vec<ValueStatement>,
    pub style_summary: String,
    pub error_patterns: Vec<ErrorPattern>,
    pub risk_triggers: Vec<RiskTrigger>,
    pub review_conclusions: Vec<ReviewConclusion>,
    pub taste_profile: TasteProfile,
}

pub struct TasteProfile {
    pub conservative_aggressive: f32,  // -1.0 ~ 1.0
    pub quality_speed: f32,
    pub evidence_narrative: f32,
    pub risk_tolerance: f32,
    pub preference_drift_log: Vec<PreferenceDrift>,
}
```

---

## 4. KPI 三层体系

### 4.1 决策质量 KPI (权重 45%)

| 指标 | 计算方式 | 目标 |
|------|----------|------|
| 决策采纳率 | 被采纳提案 / 总提案 | >70% |
| 决策成功率 | 成功决策 / 被采纳决策 | >80% |
| 引入新问题率 | 新引发问题 / 决策数 | <10% |
| 回滚触发率 | 回滚次数 / 决策数 | <5% |
| 风险预警命中率 | 预警命中 / 总预警 | >60% |

### 4.2 治理行为 KPI (权重 30%)

| 指标 | 计算方式 | 目标 |
|------|----------|------|
| 职责发言覆盖率 | 实际发言 / 应发言次数 | >90% |
| 沉默失职次数 | 该说未说次数 | <2次/周期 |
| 重复刷屏率 | 重复内容 / 总发言 | <10% |
| Challenge 恰当性 | 合理 challenge / 总 challenge | >70% |
| Risk 信息不足时 | 信息不足 raise risk / 总信息不足 | >80% |

### 4.3 组织责任 KPI (权重 25%)

| 指标 | 计算方式 | 目标 |
|------|----------|------|
| 部门视角覆盖率 | 覆盖视角 / 应覆盖视角 | >85% |
| 分配任务完成率 | 完成任务 / 分配任务 | >95% |
| 支持收敛次数 | 主动促进收敛次数 | >3次/周期 |
| 制造冲突率 | 无意义冲突 / 总发言 | <5% |
| Inactive 天数 | 无贡献天数 | <3天/周期 |

### 4.4 总分计算

```rust
impl PeriodKPI {
    pub fn total_score(&self) -> f32 {
        self.decision_quality * 0.45 +
        self.governance_conduct * 0.30 +
        self.organizational_duty * 0.25
    }
}
```

---

## 5. 升降级机制

### 5.1 升职条件

- [ ] 连续 3 周期 KPI > 80分
- [ ] 无重大纪律问题
- [ ] 关键任务中有稳定贡献
- [ ] 经上级与平级复核通过

### 5.2 降职条件 (满足任一)

- 连续 2 周期 KPI < 60分
- 决策错误率 > 30%
- 关键会议中失职 > 3次
- 未及时 raise risk > 3次
- 对组织造成实质性损害

### 5.3 停职条件 (满足任一)

- 高风险错误待调查
- 行为异常漂移
- 权限滥用嫌疑
- 记忆不一致或身份异常
- 长时间失控或无法收敛

### 5.4 终止条件 (满足任一)

- 重大恶意行为
- 系统性欺骗
- 严重权限滥用
- 多次复核后仍不可修复
- 对整体组织持续构成危险

---

## 6. 处罚与申诉

### 6.1 处罚梯度

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisciplineLevel {
    Warning,        // 警告
    Limited,        // 限权
    Demoted,        // 降职
    Suspended,      // 停职
    Terminated,     // 终止
}
```

### 6.2 处罚必须附带

```rust
pub struct DisciplineRecord {
    pub evidence_chain: Vec<Evidence>,
    pub trigger_event: GovernanceEvent,
    pub attribution: Attribution,
    pub decision_record: DecisionRecord,
    pub rule_clause: String,
    pub discipline_level: DisciplineLevel,
    pub appeal_deadline: DateTime<Utc>,
}
```

### 6.3 申诉机制

```rust
pub struct AppealProcess {
    pub entity_id: Uuid,
    pub original_discipline: DisciplineRecord,
    pub defense_summary: String,
    pub decision_log_refs: Vec<Uuid>,
    pub review_seats: Vec<SeatId>,
    pub review_status: AppealStatus,
    pub final_decision: Option<DisciplineLevel>,
}
```

---

## 7. 存活与死亡定义

### 7.1 "活着"的定义

```rust
impl AIEntity {
    pub fn is_alive(&self) -> bool {
        matches!(self.status, 
            EntityStatus::Active | 
            EntityStatus::Limited | 
            EntityStatus::Demoted | 
            EntityStatus::UnderReview
        )
    }
    
    pub fn has_governance_power(&self) -> bool {
        matches!(self.status, EntityStatus::Active)
    }
}
```

### 7.2 社会性死亡 (Archived)

- 不再 active
- 不再拥有治理权
- 历史保留，可供审计
- 理论上可再研究但不直接复活

### 7.3 制度性死亡 (Terminated)

- 永不再任命
- 历史保留，仅供审计/训练/案例分析
- 不再参与任何现实治理链

---

## 8. 记忆与提醒机制

### 8.1 决策前提醒

```rust
pub struct PreDecisionReminder {
    pub current_position: String,
    pub current_authority: Vec<Authority>,
    pub recent_kpi: PeriodKPI,
    pub last_discipline: Option<DisciplineRecord>,
    pub possible_consequences: Vec<Consequence>,
    pub organizational_impact: RiskLevel,
    pub career_impact: CareerImpact,
}

impl AIEntity {
    pub fn remind_before_decision(&self, decision: &Decision) -> PreDecisionReminder {
        PreDecisionReminder {
            current_position: self.rank.to_string(),
            current_authority: self.authority_set.clone(),
            recent_kpi: self.performance.kpi_history.last().unwrap().clone(),
            last_discipline: self.discipline.get_last(),
            possible_consequences: self.calculate_consequences(decision),
            organizational_impact: self.assess_organizational_impact(decision),
            career_impact: self.assess_career_impact(decision),
        }
    }
}
```

### 8.2 决策后回写

```rust
pub struct PostDecisionWriteback {
    pub decision_id: Uuid,
    pub primary_owner: Uuid,
    pub supporting: Vec<Uuid>,
    pub challenging: Vec<Uuid>,
    pub approving_authority: Uuid,
    pub execution_entities: Vec<Uuid>,
    pub actual_result: DecisionResult,
    pub kpi_impact: KPIImpact,
    pub discipline_triggered: Option<DisciplineLevel>,
}
```

### 8.3 月度考核提醒

```rust
pub struct MonthlyAssessmentReminder {
    pub current_period_performance: PeriodKPI,
    pub relative_ranking: (u32, u32), // (排名, 总人数)
    pub improvement_areas: Vec<String>,
    pub risk_points: Vec<RiskPoint>,
    pub demotion_threshold_proximity: f32,
    pub promotion_threshold_proximity: f32,
}
```

---

## 9. 决策责任归因

```rust
pub struct DecisionAttribution {
    pub decision_id: Uuid,
    pub primary_owner: Uuid,           // 谁提出方案
    pub supporting: Vec<Uuid>,         // 谁支持
    pub challenging: Vec<Uuid>,        // 谁反对
    pub approving_authority: Uuid,     // 谁最终拍板
    pub execution_entities: Vec<Uuid>, // 谁执行落地
}

impl DecisionAttribution {
    /// 计算每个参与者的责任权重
    pub fn calculate_responsibility(&self) -> HashMap<Uuid, f32> {
        let mut responsibility = HashMap::new();
        
        // 主要责任人：40%
        responsibility.insert(self.primary_owner, 0.40);
        
        // 支持者：各10%
        for supporter in &self.supporting {
            responsibility.insert(*supporter, 0.10);
        }
        
        // 拍板者：30%
        responsibility.insert(self.approving_authority, 0.30);
        
        // 反对者有记录但不担责
        
        responsibility
    }
}
```

---

## 10. 与现有 DragonCore 集成

### 10.1 与 Meeting Protocol 集成

```rust
// 在 MeetingSession 中增加实体责任追踪
pub struct MeetingSession {
    // ... 现有字段
    pub entity_attribution: HashMap<TurnId, DecisionAttribution>,
    pub kpi_snapshot_before: HashMap<SeatId, PeriodKPI>,
    pub kpi_snapshot_after: HashMap<SeatId, PeriodKPI>,
}
```

### 10.2 与 DIBL 事件集成

```rust
// 新增事件类型
pub enum GovernanceEventType {
    // ... 现有事件
    EntityPromoted,
    EntityDemoted,
    EntitySuspended,
    EntityTerminated,
    EntityArchived,
    KPIUpdated,
    DisciplineApplied,
    AppealFiled,
    AppealResolved,
}
```

### 10.3 CLI 命令扩展

```bash
# 实体管理
dragoncore entity create --name "Nezha-V2" --role Deployment
dragoncore entity status --entity-id <id>
dragoncore entity promote --entity-id <id> --to-rank Senior
dragoncore entity demote --entity-id <id> --reason "KPI不达标"
dragoncore entity suspend --entity-id <id> --reason "调查期"
dragoncore entity terminate --entity-id <id> --reason "重大恶意"

# KPI 查询
dragoncore entity kpi --entity-id <id> --period 2026-03
dragoncore entity ranking --department All --period 2026-03

# 申诉
dragoncore entity appeal --entity-id <id> --discipline-id <id> --reason "误判"
```

---

## 11. 最小实施版本

### v0.1 必做 (2周内)

- [ ] 持续身份档案 (`AIEntityIdentity`)
- [ ] KPI 三层体系基础实现
- [ ] 处罚梯度状态机
- [ ] 责任归因基础结构
- [ ] 决策前后提醒机制
- [ ] 月度考核输出
- [ ] 7状态实体状态机

### v0.2 再做 (4周内)

- [ ] 晋升池与候补池
- [ ] 申诉系统完整流程
- [ ] 品味演化系统 (TasteProfile)
- [ ] 跨期绩效对比

### v0.3 再做 (8周内)

- [ ] 部门级 KPI 联动
- [ ] AI 之间互评与提名
- [ ] 自主争取职位机制
- [ ] 制度化接班机制

---

## 12. 最终定义

> **AI 生命体责任制度** = 让 AI 拥有持续身份、长期记忆、正式职位、可量化绩效、真实奖惩、升降机制与终止边界，使其每一次关键决策都对组织和自己产生持续后果。

**没有后果，就没有责任；没有责任，就没有生命体。**

---

*版本: v0.1*  
*日期: 2026-03-20*  
*状态: 草案待实施*
