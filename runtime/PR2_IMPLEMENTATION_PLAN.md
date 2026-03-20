# PR-2: 责任归因与 KPI - 最小实现方案

**状态**: 待实施  
**依赖**: PR-1 (已完成)  
**目标**: 把责任归因接进 final-gate 和 replay

---

## 1. PR-2 最小实现切片

### 1.1 数据结构

```rust
// src/entity/attribution.rs

/// 决策归因记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionAttribution {
    /// 决策唯一ID
    pub decision_id: Uuid,
    
    /// 决策类型
    pub decision_type: DecisionType,
    
    /// 主要责任人 (提出方案者)
    pub primary_owner: Uuid,
    
    /// 支持者
    pub supporting: Vec<Uuid>,
    
    /// 反对者
    pub challenging: Vec<Uuid>,
    
    /// 最终拍板者
    pub approving_authority: Uuid,
    
    /// 执行实体
    pub execution_entities: Vec<Uuid>,
    
    /// 决策时间
    pub decided_at: DateTime<Utc>,
    
    /// 决策结果
    pub outcome: DecisionOutcome,
    
    /// 后续影响 (后填写)
    pub impact: Option<DecisionImpact>,
}

/// 决策类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DecisionType {
    Proposal,       // 提案
    RiskAssessment, // 风险评估
    Veto,           // 否决
    FinalGate,      // 最终门控
    Termination,    // 终止
    Archive,        // 归档
    ResourceAllocation, // 资源分配
}

/// 决策结果
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DecisionOutcome {
    Approved,
    Rejected,
    Modified,
    Deferred,
}

/// 决策影响 (决策后评估)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionImpact {
    /// 是否成功
    pub success: bool,
    
    /// 引入的新问题数
    pub new_issues: u32,
    
    /// 是否触发回滚
    pub triggered_rollback: bool,
    
    /// 评估时间
    pub assessed_at: DateTime<Utc>,
}

/// 责任权重计算
impl DecisionAttribution {
    /// 计算每个参与者的责任权重
    pub fn calculate_responsibility(&self) -> HashMap<Uuid, f32> {
        let mut weights = HashMap::new();
        
        // 主要责任人: 40%
        weights.insert(self.primary_owner, 0.40);
        
        // 支持者: 各 10% (均分)
        if !self.supporting.is_empty() {
            let supporter_weight = 0.10 / self.supporting.len() as f32;
            for supporter in &self.supporting {
                weights.insert(*supporter, supporter_weight);
            }
        }
        
        // 拍板者: 30%
        weights.insert(self.approving_authority, 0.30);
        
        // 反对者不担责，但有记录
        
        weights
    }
}
```

```rust
// src/entity/kpi.rs

/// 周期 KPI 记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodKPI {
    /// 周期标识 (如 "2026-03")
    pub period: String,
    
    /// 实体ID
    pub entity_id: Uuid,
    
    /// 决策质量 (权重 45%)
    pub decision_quality: f32,
    
    /// 治理行为 (权重 30%)
    pub governance_conduct: f32,
    
    /// 组织责任 (权重 25%)
    pub organizational_duty: f32,
    
    /// 详细指标
    pub details: KPIDetails,
    
    /// 计算时间
    pub calculated_at: DateTime<Utc>,
}

/// KPI 详细指标
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KPIDetails {
    // DecisionQuality 指标
    pub proposal_count: u32,
    pub adopted_count: u32,
    pub success_count: u32,
    pub rollback_count: u32,
    pub risk_warnings: u32,
    pub risk_hits: u32,
    
    // GovernanceConduct 指标
    pub speak_count: u32,
    pub silence_count: u32,
    pub challenge_count: u32,
    pub valid_challenges: u32,
    pub repetitive_count: u32,
    
    // OrganizationalDuty 指标
    pub assigned_tasks: u32,
    pub completed_tasks: u32,
    pub coverage_rate: f32,
    pub convergence_support: u32,
}

impl PeriodKPI {
    /// 总分计算
    pub fn total_score(&self) -> f32 {
        let score = self.decision_quality * 0.45
                  + self.governance_conduct * 0.30
                  + self.organizational_duty * 0.25;
        score.clamp(0.0, 100.0)
    }
    
    /// 根据事件计算 KPI
    pub fn calculate_from_events(
        entity_id: Uuid,
        period: &str,
        events: &[GovernanceEvent],
    ) -> Self {
        let mut details = KPIDetails::default();
        
        for event in events {
            // 只统计该实体相关事件
            if event.actor != entity_id.to_string() {
                continue;
            }
            
            match event.event_type {
                GovernanceEventType::DecisionCommitted => {
                    details.proposal_count += 1;
                }
                GovernanceEventType::RollbackTriggered => {
                    details.rollback_count += 1;
                }
                GovernanceEventType::RiskRaised => {
                    details.risk_warnings += 1;
                }
                // ... 其他事件统计
                _ => {}
            }
        }
        
        // 计算各维度得分
        let decision_quality = calculate_decision_quality(&details);
        let governance_conduct = calculate_governance_conduct(&details);
        let organizational_duty = calculate_organizational_duty(&details);
        
        Self {
            period: period.to_string(),
            entity_id,
            decision_quality,
            governance_conduct,
            organizational_duty,
            details,
            calculated_at: Utc::now(),
        }
    }
}

fn calculate_decision_quality(details: &KPIDetails) -> f32 {
    if details.proposal_count == 0 {
        return 50.0; // 中性基线
    }
    
    let adoption_rate = details.adopted_count as f32 / details.proposal_count as f32;
    let success_rate = if details.adopted_count > 0 {
        details.success_count as f32 / details.adopted_count as f32
    } else {
        0.0
    };
    let rollback_penalty = (details.rollback_count as f32 * 10.0).min(30.0);
    
    (adoption_rate * 30.0 + success_rate * 50.0 - rollback_penalty).clamp(0.0, 100.0)
}

fn calculate_governance_conduct(details: &KPIDetails) -> f32 {
    // 实现治理行为评分逻辑
    50.0 // 占位
}

fn calculate_organizational_duty(details: &KPIDetails) -> f32 {
    // 实现组织责任评分逻辑
    50.0 // 占位
}
```

### 1.2 事件定义

```rust
// src/events/mod.rs (扩展)

pub enum GovernanceEventType {
    // ... 现有事件
    
    // PR-2 新增事件
    EntityKpiUpdated,       // KPI 更新
    DecisionAttributed,     // 决策归因记录
    EntityWarned,           // 实体警告
    EntityLimited,          // 实体受限
    EntityUnderReview,      // 实体审查中
}
```

### 1.3 CLI 入口

```rust
// src/main.rs (entity 子命令扩展)

#[derive(Subcommand, Debug)]
enum EntityCommand {
    // ... 现有命令
    
    /// 查询实体 KPI
    Kpi {
        /// 实体ID
        #[arg(short, long)]
        entity_id: String,
        
        /// 周期 (如 "2026-03", 默认当前周期)
        #[arg(short, long)]
        period: Option<String>,
    },
    
    /// 查询决策归因
    Attribution {
        /// 决策ID
        #[arg(short, long)]
        decision_id: String,
    },
    
    /// 列出实体所有决策
    Decisions {
        /// 实体ID
        #[arg(short, long)]
        entity_id: String,
        
        /// 起始日期
        #[arg(short, long)]
        from: Option<String>,
        
        /// 结束日期
        #[arg(short, long)]
        to: Option<String>,
    },
}
```

### 1.4 Final-Gate 拦截点

```rust
// src/governance/final_gate.rs (修改)

pub struct FinalGate {
    attribution_required: bool,
    min_kpi_threshold: f32,
}

impl FinalGate {
    pub async fn execute(&self, decision: &Decision) -> Result<(), FinalGateError> {
        // 1. 检查归因
        if self.attribution_required {
            if decision.attribution.is_none() {
                return Err(FinalGateError::MissingAttribution);
            }
            
            let attribution = decision.attribution.as_ref().unwrap();
            
            // 验证所有参与者都是活跃实体
            for entity_id in attribution.all_participants() {
                let entity = self.entity_manager.get_entity(entity_id).await?;
                if !entity.is_governance_active() {
                    return Err(FinalGateError::InactiveParticipant(entity_id));
                }
            }
        }
        
        // 2. 检查 KPI (如果启用)
        if self.min_kpi_threshold > 0.0 {
            if let Some(ref attribution) = decision.attribution {
                let primary = attribution.primary_owner;
                let kpi = self.kpi_calculator.get_latest(primary).await?;
                if kpi.total_score() < self.min_kpi_threshold {
                    return Err(FinalGateError::LowKPI(primary, kpi.total_score()));
                }
            }
        }
        
        // 3. 执行 final gate
        // ...
        
        // 4. 记录归因到 DIBL
        if let Some(ref attribution) = decision.attribution {
            self.emit_attribution_event(attribution).await?;
        }
        
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum FinalGateError {
    #[error("决策缺少归因信息")]
    MissingAttribution,
    
    #[error("参与者 {0} 不是活跃治理实体")]
    InactiveParticipant(Uuid),
    
    #[error("主要责任人 {0} KPI 过低: {1}")]
    LowKPI(Uuid, f32),
}
```

### 1.5 Replay 影响

```rust
// src/events/replay.rs (扩展)

impl Replay {
    /// 回放时重建决策归因链
    pub fn rebuild_attribution_chain(&self, run_id: &str) -> Vec<DecisionAttribution> {
        let events = self.load_events(run_id);
        let mut attributions = Vec::new();
        
        for event in events {
            if let GovernanceEventType::DecisionAttributed = event.event_type {
                if let Ok(attr) = serde_json::from_str::<DecisionAttribution>(&event.details) {
                    attributions.push(attr);
                }
            }
        }
        
        attributions
    }
    
    /// 回放时重建 KPI 变化轨迹
    pub fn rebuild_kpi_trajectory(&self, entity_id: Uuid) -> Vec<PeriodKPI> {
        let events = self.load_all_events();
        let mut kpis = Vec::new();
        
        for event in events {
            if let GovernanceEventType::EntityKpiUpdated = event.event_type {
                if event.actor == entity_id.to_string() {
                    if let Ok(kpi) = serde_json::from_str::<PeriodKPI>(&event.details) {
                        kpis.push(kpi);
                    }
                }
            }
        }
        
        kpis.sort_by(|a, b| a.period.cmp(&b.period));
        kpis
    }
}
```

---

## 2. PR-2 文件级改动清单

### 2.1 新增文件

| 文件 | 职责 |
|------|------|
| `src/entity/attribution.rs` | DecisionAttribution, 责任权重计算 |
| `src/entity/kpi.rs` | PeriodKPI, 三层 KPI 计算 |
| `src/entity/kpi_calculator.rs` | 从事件流计算 KPI |
| `src/governance/final_gate.rs` | FinalGate 拦截点 (修改现有) |

### 2.2 修改文件

| 文件 | 修改内容 |
|------|----------|
| `src/entity/manager.rs` | 添加归因记录、KPI 查询方法 |
| `src/entity/mod.rs` | 导出 attribution, kpi 模块 |
| `src/events/mod.rs` | 新增 5 个 DIBL 事件类型 |
| `src/main.rs` | CLI 添加 kpi, attribution, decisions 命令 |
| `src/governance/mod.rs` | 集成 final-gate 拦截 |

---

## 3. PR-2 最小验收测试

### 3.1 单元测试

```rust
// src/entity/attribution.rs (tests)

#[test]
fn test_responsibility_calculation() {
    let attr = DecisionAttribution {
        decision_id: Uuid::new_v4(),
        primary_owner: Uuid::new_v4(),
        supporting: vec![Uuid::new_v4(), Uuid::new_v4()],
        challenging: vec![Uuid::new_v4()],
        approving_authority: Uuid::new_v4(),
        // ...
    };
    
    let weights = attr.calculate_responsibility();
    
    // 主要责任人 40%
    assert_eq!(weights.get(&attr.primary_owner), Some(&0.40));
    
    // 拍板者 30%
    assert_eq!(weights.get(&attr.approving_authority), Some(&0.30));
    
    // 支持者各 5%
    for supporter in &attr.supporting {
        assert_eq!(weights.get(supporter), Some(&0.05));
    }
    
    // 反对者不在权重表中
    for challenger in &attr.challenging {
        assert!(!weights.contains_key(challenger));
    }
}

#[test]
fn test_kpi_total_score() {
    let kpi = PeriodKPI {
        decision_quality: 80.0,
        governance_conduct: 70.0,
        organizational_duty: 60.0,
        // ...
    };
    
    // 80*0.45 + 70*0.30 + 60*0.25 = 36 + 21 + 15 = 72
    assert!((kpi.total_score() - 72.0).abs() < 0.1);
}
```

### 3.2 集成测试

```rust
// tests/attribution_integration.rs

#[tokio::test]
async fn test_final_gate_blocks_unattributed_decision() {
    let runtime = setup_test_runtime().await;
    
    // 创建一个无归因的决策
    let decision = Decision {
        attribution: None,
        // ...
    };
    
    // Final gate 应该拒绝
    let result = runtime.final_gate(&decision).await;
    assert!(matches!(result, Err(FinalGateError::MissingAttribution)));
}

#[tokio::test]
async fn test_kpi_calculation_from_events() {
    let runtime = setup_test_runtime().await;
    let entity = runtime.create_test_entity().await;
    
    // 模拟一系列决策事件
    runtime.simulate_decision_events(&entity, 5).await;
    
    // 计算 KPI
    let kpi = runtime.calculate_kpi(entity.entity_id, "2026-03").await;
    
    assert!(kpi.decision_quality > 0.0);
    assert!(kpi.total_score() > 0.0);
}
```

### 3.3 端到端治理回放样例

```bash
# 1. 创建实体
dragoncore entity create --name "Yuheng-Test" --role Yuheng --department Audit

# 2. 创建决策 (带归因)
dragoncore decision create --primary-owner <id> --approving-authority <id> \
  --supporting <id1>,<id2> --type Proposal

# 3. 尝试通过 final gate (应该成功)
dragoncore final-gate --decision-id <id> --approve

# 4. 查询归因
dragoncore entity attribution --decision-id <id>

# 5. 查询 KPI
dragoncore entity kpi --entity-id <id> --period 2026-03

# 6. 回放验证
dragoncore replay --run-id <run-id>
# 应该能看到：
# - DecisionAttributed 事件
# - EntityKpiUpdated 事件
```

---

## 4. Meeting Protocol 4h Endurance 最小验证方案

### 4.1 调度方式

```bash
# 使用已修改的验证脚本
nohup ./scripts/meeting_endurance_test_v2.sh > /tmp/endurance_4h.log 2>&1 &

# 监控
watch -n 30 'tail -50 /tmp/endurance_4h.log'
```

### 4.2 指标记录

| 指标 | 记录方式 | 目标 |
|------|----------|------|
| 运行时长 | 脚本自动记录 | >= 240 分钟 |
| 会议完成数 | 日志计数 | > 50 轮 |
| 成功率 | 失败计数 | 100% |
| 内存使用 | 系统监控 | 稳定增长 <50% |
| DIBL 事件数 | 事件文件行数 | > 0 (会议层) |
| stance_updates | metrics.csv | > 0 |

### 4.3 PASS / CONDITIONAL PASS 判定

```bash
# 验证脚本结束后的判定逻辑

if [ $TOTAL_MIN -ge 240 ] && [ $FAILURE_COUNT -eq 0 ]; then
    # 检查会议层 DIBL 事件
    MEETING_EVENTS=$(grep -c "MeetingTurnPublished\|StanceUpdated" /path/to/events/*.jsonl 2>/dev/null || echo "0")
    
    if [ $MEETING_EVENTS -gt 0 ]; then
        echo "✅ PASS: 会议功能稳定，会议层 DIBL 已接通"
    else
        echo "🟡 CONDITIONAL PASS: 会议功能稳定，但会议层 DIBL 未接通"
    fi
else
    echo "❌ FAIL: 运行中断或失败"
fi
```

---

## 5. 实施优先级

### 立即开始 (Day 1-3)
1. `src/entity/attribution.rs` - DecisionAttribution 数据结构
2. `src/entity/kpi.rs` - PeriodKPI 数据结构
3. `src/events/mod.rs` - 新增 5 个事件类型

### 第 4-7 天
4. `src/governance/final_gate.rs` - 拦截点实现
5. `src/entity/manager.rs` - 归因记录方法
6. CLI 命令实现

### 第 8-10 天
7. 集成测试
8. Endurance 验证
9. 文档更新

---

*计划制定: 2026-03-20*  
*状态: 等待实施启动*
