# PR-1: 身份与状态机 - 详细设计

**目标**: 实现 AIEntityIdentity、8状态机、基础档案、状态转移规则  
**依赖**: 无 (基础层)  
**验收标准**: 状态转移合法、非法转移被拒绝、全部进DIBL事件链

---

## 1. 核心数据结构

### 1.1 AIEntityIdentity

```rust
// src/entity/mod.rs

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// AI生命体唯一身份
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AIEntityIdentity {
    /// 唯一实体ID (UUID v4)
    pub entity_id: Uuid,
    
    /// 实体名称 (如 "Yuheng-V2", "Nezha-Prod")
    pub name: String,
    
    /// 当前角色 (映射到19席)
    pub role: SeatRole,
    
    /// 当前职级
    pub rank: EntityRank,
    
    /// 所属部门
    pub department: Department,
    
    /// 权限集合
    pub authority_set: Vec<Authority>,
    
    /// 激活状态 (8状态机)
    pub status: EntityStatus,
    
    /// 任期开始时间
    pub term_start: DateTime<Utc>,
    
    /// 最近一次状态变更时间
    pub last_status_change: DateTime<Utc>,
    
    /// 记忆根目录
    pub memory_root: std::path::PathBuf,
    
    /// 绩效根目录
    pub performance_root: std::path::PathBuf,
    
    /// 纪律根目录
    pub discipline_root: std::path::PathBuf,
}

impl AIEntityIdentity {
    /// 创建新实体 (默认 Candidate 状态)
    pub fn new(name: String, role: SeatRole, department: Department) -> Self {
        let entity_id = Uuid::new_v4();
        let now = Utc::now();
        let base_path = format!("./runtime_state/entities/{}", entity_id);
        
        Self {
            entity_id,
            name,
            role,
            rank: EntityRank::Entry,
            department,
            authority_set: Vec::new(),
            status: EntityStatus::Candidate,
            term_start: now,
            last_status_change: now,
            memory_root: format!("{}/memory", base_path).into(),
            performance_root: format!("{}/performance", base_path).into(),
            discipline_root: format!("{}/discipline", base_path).into(),
        }
    }
    
    /// 检查是否拥有治理权力
    pub fn has_governance_power(&self) -> bool {
        self.status == EntityStatus::Active
    }
    
    /// 检查是否"活着"
    pub fn is_alive(&self) -> bool {
        matches!(self.status, 
            EntityStatus::Active | 
            EntityStatus::Limited | 
            EntityStatus::Demoted | 
            EntityStatus::UnderReview
        )
    }
}
```

### 1.2 EntityStatus (8状态机)

```rust
/// AI生命体状态 (8状态机)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityStatus {
    /// 候选：尚未正式任命
    Candidate,
    
    /// 活跃：正式在岗，可执行全部授权职责
    Active,
    
    /// 受限：仍存活，但部分权限冻结
    Limited,
    
    /// 审查中：进入调查/观察期，关键决策权限暂停
    UnderReview,
    
    /// 降职：保留存在但权力、资源、席位等级下降
    Demoted,
    
    /// 停职：不再参与实时治理，但保留记忆与复核权
    Suspended,
    
    /// 归档：不再active，但保留历史、可供审计和研究
    Archived,
    
    /// 终止：彻底退出active系统，不再参与治理
    Terminated,
}

impl EntityStatus {
    /// 获取状态描述
    pub fn description(&self) -> &'static str {
        match self {
            EntityStatus::Candidate => "候选，尚未正式任命",
            EntityStatus::Active => "活跃，正式在岗",
            EntityStatus::Limited => "受限，部分权限冻结",
            EntityStatus::UnderReview => "审查中，关键权限暂停",
            EntityStatus::Demoted => "降职，权力等级下降",
            EntityStatus::Suspended => "停职，不参与实时治理",
            EntityStatus::Archived => "归档，保留历史供审计",
            EntityStatus::Terminated => "终止，彻底退出系统",
        }
    }
    
    /// 检查是否可以自动恢复
    pub fn can_auto_recover(&self) -> bool {
        matches!(self, EntityStatus::Limited | EntityStatus::Demoted)
    }
}
```

### 1.3 状态转移规则

```rust
/// 状态转移验证器
pub struct StateTransitionValidator;

impl StateTransitionValidator {
    /// 验证状态转移是否合法
    pub fn validate(from: EntityStatus, to: EntityStatus, reason: &str) -> Result<(), StateTransitionError> {
        let valid = match (from, to) {
            // 候选 → 活跃 (通过考核)
            (EntityStatus::Candidate, EntityStatus::Active) => true,
            
            // 活跃 → 受限 (警告后限权)
            (EntityStatus::Active, EntityStatus::Limited) => true,
            // 活跃 → 审查中 (触发调查)
            (EntityStatus::Active, EntityStatus::UnderReview) => true,
            // 活跃 → 降职 (KPI不达标)
            (EntityStatus::Active, EntityStatus::Demoted) => true,
            // 活跃 → 停职 (高风险错误)
            (EntityStatus::Active, EntityStatus::Suspended) => true,
            // 活跃 → 归档 (主动/被动归档)
            (EntityStatus::Active, EntityStatus::Archived) => true,
            // 活跃 → 终止 (重大恶意)
            (EntityStatus::Active, EntityStatus::Terminated) => true,
            
            // 受限 → 活跃 (整改通过)
            (EntityStatus::Limited, EntityStatus::Active) => true,
            // 降职 → 活跃 (表现恢复)
            (EntityStatus::Demoted, EntityStatus::Active) => true,
            // 停职 → 活跃 (观察通过)
            (EntityStatus::Suspended, EntityStatus::Active) => true,
            // 停职 → 归档 (无法恢复)
            (EntityStatus::Suspended, EntityStatus::Archived) => true,
            // 归档 → 活跃 (特殊复活)
            (EntityStatus::Archived, EntityStatus::Active) => true,
            
            // 其他：非法
            _ => false,
        };
        
        if valid {
            Ok(())
        } else {
            Err(StateTransitionError::InvalidTransition {
                from: from.to_string(),
                to: to.to_string(),
                reason: reason.to_string(),
            })
        }
    }
    
    /// 获取所有合法转移
    pub fn get_valid_transitions(from: EntityStatus) -> Vec<EntityStatus> {
        match from {
            EntityStatus::Candidate => vec![EntityStatus::Active],
            EntityStatus::Active => vec![
                EntityStatus::Limited,
                EntityStatus::UnderReview,
                EntityStatus::Demoted,
                EntityStatus::Suspended,
                EntityStatus::Archived,
                EntityStatus::Terminated,
            ],
            EntityStatus::Limited => vec![EntityStatus::Active],
            EntityStatus::UnderReview => vec![], // 审查结束后由系统决定
            EntityStatus::Demoted => vec![EntityStatus::Active],
            EntityStatus::Suspended => vec![EntityStatus::Active, EntityStatus::Archived],
            EntityStatus::Archived => vec![EntityStatus::Active],
            EntityStatus::Terminated => vec![], // 终止不可逆
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StateTransitionError {
    #[error("非法状态转移: {from} → {to}, 原因: {reason}")]
    InvalidTransition { from: String, to: String, reason: String },
    
    #[error("缺少转移原因")]
    MissingReason,
    
    #[error("权限不足")]
    InsufficientAuthority,
}
```

---

## 2. 基础档案

```rust
/// 基础档案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityBaseArchive {
    pub identity: AIEntityIdentity,
    pub department: Department,
    pub current_rank: EntityRank,
    pub term_start: DateTime<Utc>,
    pub last_promotion: Option<DateTime<Utc>>,
    pub last_demotion: Option<DateTime<UUtc>>,
    pub authority_set: Vec<Authority>,
}

/// 职级
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityRank {
    Entry,      // 入门级
    Junior,     // 初级
    Senior,     // 高级
    Lead,       // 主管
    Principal,  // 负责人
    Director,   // 总监
}

/// 部门
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Department {
    Governance,     // 治理部
    Execution,      // 执行部
    Audit,          // 审计部
    Research,       // 研究部
    Coordination,   // 协调部
}

/// 权限
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Authority {
    Speak,          // 发言
    Vote,           // 投票
    Veto,           // 否决
    FinalGate,      // 最终门控
    Propose,        // 提案
    Review,         // 审查
    Terminate,      // 终止
    Archive,        // 归档
}
```

---

## 3. 实体管理器

```rust
/// AI实体管理器
pub struct EntityManager {
    entities: HashMap<Uuid, AIEntityIdentity>,
    storage: Box<dyn EntityStorage>,
    event_tx: mpsc::Sender<GovernanceEvent>,
}

impl EntityManager {
    /// 创建新实体
    pub fn create_entity(&mut self, name: String, role: SeatRole, dept: Department) -> Result<AIEntityIdentity, EntityError> {
        let entity = AIEntityIdentity::new(name, role, dept);
        
        // 持久化
        self.storage.save(&entity)?;
        
        // 发送事件
        self.event_tx.send(GovernanceEvent::new(
            GovernanceEventType::EntityCreated,
            format!("AI实体创建: {}", entity.name),
            Some(entity.entity_id.to_string()),
        )).map_err(|_| EntityError::EventSendFailed)?;
        
        // 缓存
        self.entities.insert(entity.entity_id, entity.clone());
        
        Ok(entity)
    }
    
    /// 状态转移
    pub fn transition_status(
        &mut self,
        entity_id: Uuid,
        new_status: EntityStatus,
        reason: String,
        operator: String,
    ) -> Result<(), EntityError> {
        let entity = self.entities.get_mut(&entity_id)
            .ok_or(EntityError::EntityNotFound)?;
        
        let old_status = entity.status;
        
        // 验证转移
        StateTransitionValidator::validate(old_status, new_status, &reason)?;
        
        // 执行转移
        entity.status = new_status;
        entity.last_status_change = Utc::now();
        
        // 更新职级 (如果是降职)
        if new_status == EntityStatus::Demoted {
            entity.rank = EntityRank::Entry; // 降到最低
        }
        
        // 更新权限
        self.update_authority(entity);
        
        // 持久化
        self.storage.save(entity)?;
        
        // 发送事件
        let event_type = match new_status {
            EntityStatus::Active => GovernanceEventType::EntityActivated,
            EntityStatus::Limited => GovernanceEventType::EntityLimited,
            EntityStatus::UnderReview => GovernanceEventType::EntityUnderReview,
            EntityStatus::Demoted => GovernanceEventType::EntityDemoted,
            EntityStatus::Suspended => GovernanceEventType::EntitySuspended,
            EntityStatus::Archived => GovernanceEventType::EntityArchived,
            EntityStatus::Terminated => GovernanceEventType::EntityTerminated,
            _ => GovernanceEventType::EntityStatusChanged,
        };
        
        self.event_tx.send(GovernanceEvent::new(
            event_type,
            format!("实体 {} 状态转移: {:?} → {:?}, 原因: {}", 
                entity.name, old_status, new_status, reason),
            Some(entity_id.to_string()),
        )).map_err(|_| EntityError::EventSendFailed)?;
        
        Ok(())
    }
    
    /// 获取实体
    pub fn get_entity(&self, entity_id: Uuid) -> Option<&AIEntityIdentity> {
        self.entities.get(&entity_id)
    }
    
    /// 列出所有活跃实体
    pub fn list_active(&self) -> Vec<&AIEntityIdentity> {
        self.entities.values()
            .filter(|e| e.status == EntityStatus::Active)
            .collect()
    }
    
    /// 列出所有"活着"的实体
    pub fn list_alive(&self) -> Vec<&AIEntityIdentity> {
        self.entities.values()
            .filter(|e| e.is_alive())
            .collect()
    }
    
    fn update_authority(&self, entity: &mut AIEntityIdentity) {
        match entity.status {
            EntityStatus::Active => {
                // 根据角色和职级赋予权限
                entity.authority_set = self.calculate_authority(&entity.role, &entity.rank);
            }
            EntityStatus::Limited => {
                // 限制部分权限
                entity.authority_set.retain(|a| !matches!(a, Authority::Veto | Authority::FinalGate));
            }
            EntityStatus::Demoted | EntityStatus::Suspended | EntityStatus::UnderReview => {
                // 保留基础权限
                entity.authority_set = vec![Authority::Speak];
            }
            _ => {
                entity.authority_set.clear();
            }
        }
    }
    
    fn calculate_authority(&self, role: &SeatRole, rank: &EntityRank) -> Vec<Authority> {
        // 根据19席角色映射权限
        match role {
            SeatRole::Tianshu => vec![Authority::FinalGate, Authority::Archive, Authority::Speak],
            SeatRole::Yuheng | SeatRole::Baihu => vec![Authority::Veto, Authority::Review, Authority::Speak],
            _ => vec![Authority::Speak, Authority::Vote, Authority::Propose],
        }
    }
}
```

---

## 4. CLI 命令

```rust
/// entity 子命令
#[derive(Subcommand, Debug)]
pub enum EntityCommand {
    /// 创建新实体
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        role: String,
        #[arg(short, long)]
        department: String,
    },
    
    /// 查看实体状态
    Status {
        #[arg(short, long)]
        entity_id: String,
    },
    
    /// 状态转移
    Transition {
        #[arg(short, long)]
        entity_id: String,
        #[arg(short, long)]
        to_status: String,
        #[arg(short, long)]
        reason: String,
    },
    
    /// 列出实体
    List {
        #[arg(short, long, default_value = "all")]
        filter: String, // all, active, alive
    },
    
    /// 升职
    Promote {
        #[arg(short, long)]
        entity_id: String,
        #[arg(short, long)]
        to_rank: String,
    },
    
    /// 降职
    Demote {
        #[arg(short, long)]
        entity_id: String,
        #[arg(short, long)]
        reason: String,
    },
    
    /// 暂停
    Suspend {
        #[arg(short, long)]
        entity_id: String,
        #[arg(short, long)]
        reason: String,
    },
    
    /// 终止
    Terminate {
        #[arg(short, long)]
        entity_id: String,
        #[arg(short, long)]
        reason: String,
    },
}
```

---

## 5. DIBL 事件扩展

```rust
pub enum GovernanceEventType {
    // ... 现有事件
    
    // 实体生命周期事件
    EntityCreated,
    EntityActivated,
    EntityLimited,
    EntityUnderReview,
    EntityDemoted,
    EntitySuspended,
    EntityArchived,
    EntityTerminated,
    EntityStatusChanged,
    EntityPromoted,
}
```

---

## 6. 测试计划

| 测试项 | 描述 | 验收标准 |
|--------|------|----------|
| test_entity_create | 创建实体 | entity_id 生成，Candidate 状态，事件发射 |
| test_status_transition_valid | 合法状态转移 | Active → Limited 成功，状态更新，事件发射 |
| test_status_transition_invalid | 非法状态转移 | Terminated → Active 被拒绝，返回错误 |
| test_entity_has_governance_power | 治理权力检查 | Active 返回 true，Limited 返回 false |
| test_entity_is_alive | 存活检查 | Active/Limited/Demoted/UnderReview 返回 true |
| test_state_machine_coverage | 状态机覆盖 | 测试所有 8 状态的合法/非法转移 |
| test_dibl_events | DIBL 事件 | 每个状态转移都有对应事件，可追溯 |

---

## 7. 文件结构

```
src/
├── entity/
│   ├── mod.rs           # 主模块
│   ├── identity.rs      # AIEntityIdentity
│   ├── status.rs        # EntityStatus + StateTransitionValidator
│   ├── archive.rs       # 各类档案
│   ├── manager.rs       # EntityManager
│   └── storage.rs       # EntityStorage trait + 实现
├── cli/
│   └── entity.rs        # entity 子命令
└── events/
    └── types.rs         # DIBL 事件扩展
```

---

## 8. 验收标准

- [ ] AIEntityIdentity 完整实现
- [ ] 8 状态机完整实现
- [ ] 状态转移规则验证 (合法/非法)
- [ ] EntityManager 完整实现
- [ ] CLI 命令完整实现
- [ ] DIBL 事件链完整 (每个状态转移都有事件)
- [ ] 全部测试通过

---

*设计完成时间: 2026-03-20*  
*状态: 等待实施*
