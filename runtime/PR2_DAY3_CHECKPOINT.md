# PR-2 Day 3 检查口径

**检查日期**: Day 3 结束时 (实施启动后 72 小时)  
**检查人**: DragonCore 负责人 / 外部审计  
**通过标准**: 6 条必须同时满足

---

## 核心原则

> **Day 3 通过 ≠ "文件存在" 或 "能编译一次"**
>
> **Day 3 通过 = 归因数据结构、事件类型、最小查询路径已经形成可验证闭环**

只有过了这个闸门，Day 4-7 去接 final-gate 才是稳的。

---

## 6 条通过标准

### 1. attribution.rs 已落地 + 三类权重测试通过

**必须覆盖的测试**:

```rust
// 测试 1: 标准分配总和为 100%
#[test]
fn test_standard_weight_distribution() {
    // primary: 40%, approver: 30%, 2 supporters: 各 15%
    // 40 + 30 + 15 + 15 = 100
}

// 测试 2: 多角色累加
#[test]
fn test_multiple_roles_accumulation() {
    // 同一实体担任 primary (40%) + approver (30%) + supporter (15%) = 85%
    // 明确标注: "主责+拍板+2支持者之一"
}

// 测试 3: 无支持者时剩余 30% 作废
#[test]
fn test_no_supporter_weight_forfeited() {
    // primary: 40%, approver: 30%, 无支持者
    // 总和 = 70%，剩余 30% 不归入任何方
}
```

**检查方式**:
```bash
cargo test entity::attribution --lib
# 必须全部通过
```

---

### 2. kpi.rs 已落地 + total_score() 可测 + 三层权重不得漂移

**权重分配** (已冻结，不得更改):
- DecisionQuality: **45%**
- GovernanceConduct: **30%**
- OrganizationalDuty: **25%**

**必须通过的测试**:

```rust
#[test]
fn test_kpi_total_score_calculation() {
    let kpi = PeriodKPI {
        decision_quality: 80.0,
        governance_conduct: 70.0,
        organizational_duty: 60.0,
    };
    
    // 80*0.45 + 70*0.30 + 60*0.25 = 36 + 21 + 15 = 72
    assert!((kpi.total_score() - 72.0).abs() < 0.1);
}
```

**禁止**: 只有 struct 占位，没有计算方法。

---

### 3. 5 个新事件已进入正式事件枚举

**必须存在的事件类型**:

```rust
pub enum GovernanceEventType {
    // ... 现有事件
    
    // PR-2 新增 (5个)
    EntityKpiUpdated,       // ✅ 必须存在
    DecisionAttributed,     // ✅ 必须存在
    EntityWarned,           // ✅ 必须存在
    EntityLimited,          // ✅ 必须存在
    EntityUnderReview,      // ✅ 必须存在
}
```

**检查方式**:
```bash
grep -E "EntityKpiUpdated|DecisionAttributed|EntityWarned|EntityLimited|EntityUnderReview" \
  src/events/mod.rs
# 5个都必须找到
```

---

### 4. 查询闭环至少有一半已经通

**最低要求** (不是 final-gate 已接线):

> **"事件能落盘 + CLI 能读回"**

**必须实现**:

```rust
// 事件落盘 (通过 DIBL)
impl AttributionStorage {
    pub async fn save_attribution(&self, attr: &DecisionAttribution) -> Result<()> {
        // 必须实际写入存储
    }
}

// CLI 能读回
#[derive(Subcommand, Debug)]
enum EntityCommand {
    // ...
    /// 查询决策归因
    Attribution {
        #[arg(short, long)]
        decision_id: String,
    },
    /// 查询实体 KPI
    Kpi {
        #[arg(short, long)]
        entity_id: String,
        #[arg(short, long)]
        period: Option<String>,
    },
}

// CLI 实现必须能读到真实数据，不是 stub
Commands::Entity { command: EntityCommand::Attribution { decision_id } } => {
    let attr = manager.get_attribution(&decision_id).await?;
    println!("{}", serde_json::to_string_pretty(&attr)?);
}
```

**禁止**: 只加 clap 命令定义，没有真实数据路径。

---

### 5. 不得碰 KPI 阈值惩罚逻辑

**Day 3 禁止出现**:

```rust
// 这是 Day 4-7 才做的事
if kpi.total_score() < MIN_THRESHOLD {
    return Err(FinalGateError::LowKPI); // ❌ Day 3 禁止
}
```

**允许**: 错误类型预留但未启用

```rust
pub enum FinalGateError {
    // ...
    LowKPI(Uuid, f32), // ✅ 可以预留类型，但不得真正拦截
}
```

**偏航判断**: Day 3 若出现 LowKPI 真正生效 → 判定为偏航。

---

### 6. 不得破坏 v0.2.1 单节点 JSON/CSV 持久化路径

**必须保持**:
- `runtime_state/` 目录结构不变
- JSON/CSV 格式不变
- 现有 CLI 命令 (`run`, `veto`, `final-gate`, `archive`, `terminate`) 全部可用

**检查方式**:
```bash
# 回归测试
cargo test --lib
# 必须全部通过

cargo test integration
# 不得出现与 persistence 相关的失败
```

---

## Day 3 报告固定格式

提交时必须按以下 6 项报告:

### 1. 已实现文件
```
- src/entity/attribution.rs (行数: XXX)
- src/entity/kpi.rs (行数: XXX)
- src/events/mod.rs (修改行数: XXX)
- src/entity/cli.rs (新增/修改)
```

### 2. 编译结果
```
cargo build --release
# 结果: ✅ 通过 / ❌ 失败 (附错误日志)
```

### 3. 新增测试及通过情况
```
test_standard_weight_distribution ........... ✅
test_multiple_roles_accumulation ............ ✅
test_no_supporter_weight_forfeited .......... ✅
test_kpi_total_score_calculation ............ ✅
test_event_types_exist ...................... ✅
test_attribution_roundtrip .................. ✅/⏳
```

### 4. 事件落盘样例
```json
{
  "event_type": "DecisionAttributed",
  "decision_id": "...",
  "primary_owner": "...",
  "approving_authority": "...",
  "supporting": ["..."],
  "timestamp": "2026-03-23T..."
}
```

### 5. CLI 查询样例
```bash
$ dragoncore entity attribution --decision-id <id>
{
  "decision_id": "...",
  "primary_owner": "...",
  "responsibility_weights": {
    "...": 0.40,
    "...": 0.30,
    "...": 0.15,
    "...": 0.15
  }
}
```

### 6. 对 v0.2.1 路径的影响
```
- 现有 JSON/CSV 路径: ✅ 未受影响
- 现有 CLI 命令: ✅ 全部可用
- 新增依赖: async-trait, chrono (已存在)
```

### 7. 未完成项与阻塞项
```
⏳ 待完成:
- final-gate 接线 (Day 4-7)
- replay 重建 (Day 8-10)

🚫 阻塞项:
- 无
```

---

## 多角色测试口径说明

计划写了两种多角色测试场景，**两者不冲突，但必须标注清楚前提**:

| 场景 | 前提 | 预期权重 |
|------|------|----------|
| 85% 场景 | 主责 + 拍板 + **2支持者之一** | 40% + 30% + 15% = 85% |
| 100% 场景 | 主责 + 拍板 + **唯一支持者** | 40% + 30% + 30% = 100% |

**Day 3 报告必须写明**:
```rust
// 测试: 主责+拍板+2支持者之一 = 85%
// 2 supporters: each gets 15%, so one supporter = 15%
```

---

## 检查流程

```bash
# 1. 拉取 Day 3 提交
git fetch
git checkout <day3-commit>

# 2. 编译检查
cargo build --release

# 3. 单元测试
cargo test entity::attribution entity::kpi --lib

# 4. 事件类型检查
grep -E "EntityKpiUpdated|DecisionAttributed|EntityWarned|EntityLimited|EntityUnderReview" \
  src/events/mod.rs | wc -l
# 期望: 5

# 5. 集成测试
cargo test --test integration 2>&1 | grep -E "(test.*attribution|test.*kpi)"

# 6. CLI 检查
cargo run -- entity attribution --help
cargo run -- entity kpi --help

# 7. 回归测试
cargo test --lib 2>&1 | tail -5
# 期望: all passed
```

---

## 通过 / 不通过判定

### ✅ 通过 (全部满足)

- 6 条标准全部满足
- 报告 7 项完整
- 无阻塞项

**结果**: 可进入 Day 4-7 (final-gate 接线)

### 🟡 条件通过 (部分满足)

- 核心数据结构完成
- 但查询闭环未完全打通
- 或测试覆盖率不足

**结果**: Day 4 开始前补足缺口

### ❌ 不通过 (关键缺口)

- 权重计算错误
- 或事件类型缺失
- 或破坏 v0.2.1 路径

**结果**: 修复后重新检查

---

## 关键提醒

> **"可编译最小闭环" ≠ "文件存在且编译通过"**
>
> **= 数据结构 + 事件落盘 + CLI 查询 形成完整数据流**

**Day 3 的真正闸门**:
- [ ] 创建归因 → 落盘 → CLI 读回 (端到端)
- [ ] 创建 KPI → 落盘 → CLI 读回 (端到端)
- [ ] 5 个新事件类型可发射、可存储、可查询

---

*检查口径发布: 2026-03-20*  
*生效日期: PR-2 Day 3 结束时*  
*版本: v1.0*
