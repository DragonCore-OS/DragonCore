<div align="center">
  <img src="assets/logo.jpg" alt="DragonCore Logo" width="400">
  
  # DragonCore / 龙核
  
  **Governance-First Operating System for Multi-Agent AI**
  
  **面向多智能体的治理优先操作系统**
  
  ---
  
  **Dragon, not Claw / 真龙，不是龙虾**
</div>

---

## 中英文命名差异 | The Cross-Lingual Naming Distinction

在英文里，**lobster** 就只是 *lobster*。

但在中文里，龙虾这个词写作 **龙虾**：
- **龙** (lóng) = dragon  
- **虾** (xiā) = shrimp

*In English, "lobster" is just lobster. But in Chinese, the word for lobster is 龙虾: 龙 = dragon, 虾 = shrimp.*

这创造了一个在英文中**不存在**、但在中文里**立即成立**的区别。

*That creates a distinction that does not exist naturally in English, but matters immediately in Chinese.*

所以当 DragonCore 说 **真龙，不是龙虾**，或者英文 **Dragon, not Claw** 时，这不是文字游戏。这是一个**命名立场**。

*So when DragonCore says "True Dragon, not Lobster" or "Dragon, not Claw", it is not just wordplay. It is a naming position.*

DragonCore **不要**借龙之名做装饰、做表面品牌、做 borrowed symbolism。它要的是华夏文明意义上的**真龙**：

| Dragon Symbolizes | What We Built |
|-------------------|---------------|
| 有序 / Order | 三省六部治理架构 / Layered governance (三省六部) |
| 合法性 / Legitimacy | 可追溯的权威与终局裁决 / Traceable authority and final arbitration |
| 协调 / Coordination | 多智能体协商，非混乱竞争 / Multi-agent deliberation, not chaotic competition |
| 连续性 / Continuity | 归档、传承、文明记忆 / Archive, inheritance, civilizational memory |
| 适应性 / Adaptability | 升级、回滚、可执行恢复 / Escalation, rollback, executable recovery |

---

## 技术对比 | Technical Comparison: DragonCore vs OpenClaw

| 特性 | DragonCore (龙核) | OpenClaw |
|------|-------------------|----------|
| **核心语言 / Core Language** | Rust (零成本抽象) | Python (解释执行) |
| **内存占用 / Memory Footprint** | ~15-30 MB | ~150-300 MB |
| **启动速度 / Startup Time** | < 50ms | ~500ms-2s |
| **执行速度 / Execution Speed** | 原生机器码，无 GIL 限制 | 受 GIL 限制，解释开销 |
| **进程隔离 / Process Isolation** | ✅ tmux 会话隔离 | ❌ 单进程 |
| **工作树 / Worktree** | ✅ Git worktree 原生支持 | ❌ 需手动管理 |
| **多 Agent 并发 / Multi-Agent Concurrency** | ✅ 真并行，多 tmux 窗口同时 active | ⚠️ 伪并发，协程切换 |
| **运行时安全 / Runtime Safety** | ✅ 编译期内存安全保证 | ⚠️ 运行时错误风险 |
| **跨平台 / Cross-Platform** | ✅ Linux/macOS/Windows | ✅ 全平台 |
| **华夏治理内核 / Huaxia Governance** | ✅ 三省六部 + 19席诸神会议 | ❌ 西方议会模式 |

### 核心技术优势详解 | Core Technical Advantages

**1. Rust 核心：性能与安全的完美结合**

DragonCore 的 ZeroClaw 运行时完全使用 Rust 编写：
- **内存效率**: 相比 Python 版本，内存占用减少 **80-90%**
- **启动速度**: 冷启动时间 < 50ms，比 Python 快 **10-40 倍**
- **并发性能**: 无 GIL 限制，真正的多核并行执行

**2. tmux + worktree：生产级多 Agent 环境**

```bash
# DragonCore 启动时自动创建隔离的 tmux 会话
# 每个 Agent 运行在独立的 pane/window 中
# 支持同时监控多个 Agent 的实时输出

$ ./launch-huaxia.sh
# 创建 19 个独立的 tmux 窗口，每席一位
# 天枢窗口: 终局裁决监控
# 玉衡窗口: 质量门禁实时输出
# 开阳窗口: 执行进度追踪
```

- **真并行**: 多个 Agent 同时在不同 tmux 窗口中 active，非协程伪并发
- **故障隔离**: 单个 Agent 崩溃不影响其他 Agent
- **实时监控**: 人类操作员可通过 tmux attach 观察所有 Agent 状态

**3. Git Worktree：干净的执行环境**

```bash
# 每个 governance run 在独立的 git worktree 中执行
# 确保无状态污染，支持并行执行多个 runs

$ git worktree add runs/run-042 main
$ cd runs/run-042 && dragoncore execute
```

**4. 诸神会议系统：经过 17+ 轮实战打磨**

DragonCore 的治理系统已通过 **17+ 轮生产级验证**：
- ✅ 否决权机制 (Veto)
- ✅ 冲突解决 (Conflict Resolution)  
- ✅ 升级机制 (Escalation)
- ✅ 回滚执行 (Rollback)
- ✅ 归档系统 (Archive)
- ✅ 终止机制 (Termination)
- ✅ 真实外部输入处理
- ✅ 连续稳定性验证 (10+ runs 无漂移)

---

## 19 核心席位详解 | The 19 Seats in Detail

DragonCore 的治理内核由 **19 个固定席位**组成。它们不是装饰性人格，而是承载具体权力的治理单元。

### 第一层：北斗七星 | The Seven Northern Pivot Seats

北斗七星形成系统的主治理脊柱，对应现代企业中的 C-suite，但权力更加分离。

| 席位<br>Seat | 姓名<br>Name | 治理职能<br>Governance Role | 权力驱动<br>Power Drive | 核心冲突<br>Core Conflict |
|-------------|-------------|---------------------------|----------------------|------------------------|
| **天枢**<br>Tianshu | 北极星<br>Polaris | **CEO/最高裁决**<br>Final Gate, Arbitration | 主线定义权<br>终局裁决权 | vs 瑶光(破边界)<br>vs 青龙(分散资源) |
| **天璇**<br>Tianxuan | 北斗第二星<br>Dubhe | **COO/资源调度**<br>Resource Allocation | 资源分配权<br>流程控制权 | vs 地官(口径冲突)<br>vs 哪吒(打乱节奏) |
| **天玑**<br>Tianji | 北斗第三星<br>Merak | **CTO/技术路线**<br>Tech Stack Definition | 技术标准权<br>架构定义权 | vs 开阳(赶工期)<br>vs 瑶光(技术风险) |
| **天权**<br>Tianquan | 北斗第四星<br>Phecda | **CSO/战略定义**<br>Strategy Definition | 战略排序权<br>议题定义权 | vs 瑶光(新赛道)<br>vs 诸葛亮(分析深度) |
| **玉衡**<br>Yuheng | 玉衡/主衡量<br>Alioth | **CRO/风控门禁**<br>Risk & Quality Gate | 质量否决权<br>标准制定权 | vs 瑶光(创新激进)<br>vs 哪吒(快速突击) |
| **开阳**<br>Kaiyang | 开阳/武曲星<br>Mizar | **执行编排**<br>Implementation Review | 执行调度权<br>交付控制权 | vs 玉衡(风控拖延)<br>vs 包拯(审计深度) |
| **瑶光**<br>Yaoguang | 瑶光/破军星<br>Alkaid | **创新跃迁**<br>Innovation & Archive | 创新突破权<br>边界打破权 | vs 玉衡(风控保守)<br>vs 天枢(主线约束) |

**北斗七星的权力制衡 | Power Balance of the Seven Stars**:
- **天枢(定)** vs **瑶光(变)**: 稳定与创新的顶层冲突 / Stability vs Innovation
- **天璇(调)** vs **天玑(控)**: 资源与技术的分配矛盾 / Resources vs Technology
- **天权(划)** vs **开阳(推)**: 规划与执行的天然张力 / Planning vs Execution
- **玉衡(卡)** vs **全员**: 风控与效率的永恒斗争 / Risk Control vs Efficiency

### 第二层：四象 | The Four Symbols

四象是四方神兽，负责**军团/战役层**的制衡，确保治理覆盖所有方向。

| 席位<br>Seat | 方位<br>Direction | 神话原型<br>Mythic Archetype | 治理职能<br>Governance Role | 核心规则<br>Core Rule |
|-------------|------------------|---------------------------|--------------------------|---------------------|
| **青龙**<br>Qinglong | 东<br>East | 苍龙/孟章神君<br>Azure Dragon | **新赛道探索**<br>New Track Exploration | 必须给出 stop condition<br>Must define stop condition |
| **白虎**<br>Baihu | 西<br>West | 白虎/监兵神君<br>White Tiger | **红队/压测**<br>Red Team/Stress Test | 必须给出修复窗口<br>Must give fix window |
| **朱雀**<br>Zhuque | 南<br>South | 朱雀/陵光神君<br>Vermilion Bird | **外部叙事**<br>External Narrative | 必须等白虎验证完成<br>Wait for Baihu verification |
| **玄武**<br>Xuanwu | 北<br>North | 玄武/执明神君<br>Black Tortoise | **稳态保障**<br>Stability Assurance | 必须给青龙探索空间<br>Give Qinglong exploration space |

**核心冲突 | Core Conflicts**:
- **青龙(探)** vs **白虎(验)**: 探索与验证的张力 / Exploration vs Verification
- **朱雀(讲)** vs **白虎(验)**: 承诺与验证的冲突 / Promise vs Verification
- **青龙(新)** vs **玄武(稳)**: 新方向与稳定性的矛盾 / New Direction vs Stability

### 第三层：八仙护法 | The Eight Guardian Immortals

八仙护法是**关键功能席**，提供超越常规官僚制的专门化权力。

| 席位<br>Seat | 姓名<br>Name | 神话原型<br>Mythic Archetype | 核心权力<br>Core Authority | 为什么选择他<br>Why This Figure |
|-------------|-------------|---------------------------|------------------------|------------------------------|
| **杨戬**<br>Yangjian | 二郎神<br>Erlang Shen | 三眼天神/听调不听宣<br>Three-Eyed God | **质量监督**<br>Quality Inspection | 拥有**天眼**，能看穿一切 deception。负责检测"虚假进度" —— 当 agent 声称任务完成但实际未完成时，杨戬能看穿。/ With his **Heavenly Eye**, sees through all deception. Detects "fake progress" when agents claim completion. |
| **包拯**<br>Baozheng | 包青天<br>Lord Bao | 北宋清官/铁面无私<br>Song Dynasty Judge | **独立审计**<br>Independent Audit | **黑脸**代表 impartiality，**月牙**代表 clear night vision。无法被贿赂或恐吓，负责审计、调查权力滥用。/ **Black face** = impartiality, **crescent moon** = night vision. Cannot be bribed or intimidated. |
| **钟馗**<br>Zhongkui | 捉鬼天师<br>Ghost Catcher | 驱魔真君/鬼王<br>Demon Queller | **异常肃清**<br>Anomaly Purge | 手持宝剑的**鬼王**，专门猎杀恶鬼。负责**威胁消除** —— 清除 toxic agents、移除 compromised components。/ **Ghost King** with sword. Hunts evil spirits. Eliminates toxic agents. |
| **鲁班**<br>Luban | 工匠祖师<br>Master Craftsman | 公输班/百工圣祖<br>Patron of Craftsmen | **工程平台**<br>Engineering Platform | 发明了**锯子、云梯、无数工具**。代表最高工艺标准。确保代码优雅、可维护、能流传。/ Invented **saw, ladder, countless tools**. Highest craftsmanship standards. Elegant, maintainable code. |
| **诸葛亮**<br>Zhugeliang | 卧龙军师<br>Dragon Strategist | 蜀汉丞相/智慧化身<br>Shu Chancellor | **首席参谋**<br>Chief Advisor | **空城计**的 strategist。handles 复杂多步骤战役、危机规划、straightforward solutions 失败的情况。/ Master of **Empty Fort Strategy**. Complex multi-step campaigns, crisis planning. |
| **哪吒**<br>Nezha | 三太子<br>Third Prince | 莲花化身/风火轮<br>Lotus Incarnation | **太子/先遣**<br>Rapid Deployment | **莲花化身**，手持乾坤圈和火尖枪。代表**速度和果断行动**。负责 rapid deployment 和 crisis management。/ **Lotus incarnation** with cosmic rings. Speed and decisive action. Rapid deployment. |
| **西王母**<br>Xiwangmu | 昆仑主母<br>Queen Mother | 女仙之首/蟠桃园主<br>Queen of Immortals | **稀缺资源**<br>Scarce Resources | 掌管**蟠桃** —— 终极稀缺资源。决定谁得到什么、何时、为什么。管理 compute budgets、API quotas。/ Guards **Peaches of Immortality** — ultimate scarce resource. Controls compute budgets, API quotas. |
| **丰都大帝**<br>Fengdudadi | 冥界之主<br>Lord of Underworld | 北阴酆都大帝/鬼帝<br>Emperor of Fengdu | **归档终止**<br>Termination & Archive | 统治**阴间** —— 终结、审判、死后世界的 realm。拥有**终止**（结束项目）和**归档**（保存到永恒）的权力。/ Rules **Underworld** — realm of endings. Power to **terminate** projects and **archive** eternally. |

**八仙护法的功能覆盖 | Functional Coverage**:
- **监督层 Supervisory**: 杨戬(质量) + 包拯(审计) + 钟馗(肃清)
- **工程层 Engineering**: 鲁班(平台) + 诸葛亮(规划)
- **执行层 Execution**: 哪吒(突击)
- **资源层 Resources**: 西王母(稀缺)
- **终局层 Finality**: 丰都大帝(终止)

---

## 发言风格对比 | Communication Styles

不同席位有截然不同的发言风格：

**保守派 Conservative (玉衡、玄武、包拯、丰都大帝)**:
```
"风险等级为X，不能通过"
"监控覆盖率不达标"
"此议须经审计"
"不是所有东西都该继续活着"
```

**激进派 Aggressive (瑶光、青龙、哪吒)**:
```
"现有方案已触顶，必须尝试..."
"新机会窗口期仅X个月"
"此事已拖太久，今日必决"
"失败我担，成功共享"
```

**验证派 Verification (白虎、杨戬、钟馗)**:
```
"未经白虎压测，任何上线都是赌博"
"表面数据X，深层分析Y"
"发现异常，立即清扫"
```

**资源派 Resource (天璇、西王母、鲁班)**:
```
"资源占用率数据显示..."
"此资源稀缺，仅X份"
"此操作重复X次/天，必须工具化"
```

---

## 华夏人物库 | The Huaxia Persona Registry

19 席只是核心。DragonCore 拥有**可扩展的华夏人物库**，目前已定义 **30+ 神话/历史人物**，可用于：

- 二级机构/局/司/台主官选配
- 专项计划代号命名
- 候补主官池

*The 19 seats are just the core. DragonCore has an **extensible Huaxia Persona Registry** with **30+ mythic/historical figures** defined for secondary institutions, project codenames, and reserve pools.*

### A. 最高秩序/象征层 | Supreme Order/Symbolic Layer

| 人物<br>Figure | 定位<br>Positioning | 核心权力<br>Core Power | 适用层级<br>Layer |
|---------------|--------------------|----------------------|----------------|
| **玉皇大帝**<br>Jade Emperor | 秩序象征/名义天庭之主<br>Symbol of Order | 维护天庭表面秩序与等级<br>Maintain celestial hierarchy | advisor / ceremonial |
| **西王母**<br>Queen Mother | 珍稀资源/长期储备<br>Scarce Resources | 控制稀缺资源的释放节奏<br>Control resource release | supervisory |
| **丰都大帝**<br>Lord of Underworld | 冻结/黑名单/终止/归档<br>Termination & Archive | 决定什么该停止、什么该封存<br>Decide what stops, what archives | archival |

### B. 三清/元规则层 | Three Pure Ones / Meta-Rule Layer

| 人物<br>Figure | 定位<br>Positioning | 核心权力<br>Core Power | 适用层级<br>Layer |
|---------------|--------------------|----------------------|----------------|
| **元始天尊**<br>Primordial Heavenly Lord | 元规则/本源秩序<br>Meta-rules | 元架构、世界观总则<br>Meta-architecture | advisor |
| **灵宝天尊**<br>Heavenly Lord of Spiritual Treasures | 法度/规则/仪式<br>Rules & Rituals | 维护规则的神圣性<br>Maintain rule sanctity | protocol_engine |
| **太上老君**<br>Grand Pure One | 炼化/稳态/工艺<br>Refinement | 系统调优、慢变量工程<br>System tuning | advisor |

### C. 教化/传承/导师层 | Education/Inheritance/Mentor Layer

| 人物<br>Figure | 定位<br>Positioning | 核心权力<br>Core Power | 适用层级<br>Layer |
|---------------|--------------------|----------------------|----------------|
| **广成子**<br>Guangchengzi | 传承/师法/高阶训练<br>Heritage | 训练体系、知识传承<br>Training systems | advisor |
| **太乙真人**<br>Taiyi Zhenren | 修补/重构/再造<br>Restoration | 高危救火、异常修复<br>Crisis repair | special_office |
| **诸葛亮**<br>Zhuge Liang | 复杂规划/多线推演<br>Complex Planning | 首席参谋、太傅<br>Chief Advisor | supervisory |

### D. 审计/法度/纪律/裁决层 | Audit/Law/Discipline/Judgment Layer

| 人物<br>Figure | 定位<br>Positioning | 核心权力<br>Core Power | 适用层级<br>Layer |
|---------------|--------------------|----------------------|----------------|
| **包拯**<br>Lord Bao | 审计/追责/回溯<br>Audit | 独立审查、权力滥用调查<br>Independent audit | supervisory |
| **狄仁杰**<br>Di Renjie | 侦查/复杂案情抽丝剥茧<br>Investigation | 调查、异常归因<br>Complex investigation | special_office |
| **韩非**<br>Han Fei | 法家/制度执行/权限边界<br>Legalist | 审批规则、授权制度<br>Permission governance | protocol_engine |
| **商鞅**<br>Shang Yang | 激进制度变革/奖惩<br>Radical Reform | 重构期、纪律整顿期<br>Restructuring | special_office |
| **关羽**<br>Guan Yu | 信义/纪律/守约<br>Integrity | 纪律执行、合同履约<br>Discipline enforcement | supervisory |
| **钟馗**<br>Zhongkui | 异常清扫/污染剔除<br>Purification | 恶性流程拦截<br>Toxic process blocking | supervisory |

### E. 工程/工具/发明/结构层 | Engineering/Tools/Invention Layer

| 人物<br>Figure | 定位<br>Positioning | 核心权力<br>Core Power | 适用层级<br>Layer |
|---------------|--------------------|----------------------|----------------|
| **鲁班**<br>Lu Ban | 工具链/平台工程<br>Platform Engineering | 自动化框架、工具标准<br>Toolchain standards | supervisory |
| **墨子**<br>Mozi | 务实工程/节用/防御<br>Pragmatic Engineering | 成本优化、工程现实主义<br>Cost optimization | special_office |
| **张衡**<br>Zhang Heng | 观测/仪表/预警<br>Observation | 监控系统、仪表盘<br>Monitoring systems | special_office |
| **扁鹊**<br>Bian Que | 诊断/故障前识别<br>Diagnosis | 系统健康诊断<br>System health diagnosis | special_office |

### F. 创新/冲锋/突击/高风险层 | Innovation/Charge/High-Risk Layer

| 人物<br>Figure | 定位<br>Positioning | 核心权力<br>Core Power | 适用层级<br>Layer |
|---------------|--------------------|----------------------|----------------|
| **哪吒**<br>Nezha | 快节奏先遣/打穿僵局<br>Rapid Deployment | 快速突击<br>Fast breakthrough | supervisory |
| **杨戬**<br>Yang Jian | 核验/巡检/看穿伪装<br>Verification | 看穿虚假进度<br>Detect fake progress | supervisory |
| **后羿**<br>Hou Yi | 定点打击/精确清除<br>Precision Strike | 关键问题清除<br>Critical problem elimination | special_office |
| **孙悟空/齐天大圣**<br>Sun Wukong | 破局/高创造性试探/越界式创新<br>Breakthrough | 高风险探索、强突围<br>High-risk exploration | warband |
| **共工**<br>Gonggong | 破坏性测试/压力冲击<br>Destructive Testing | 系统毁伤模拟<br>System damage simulation | warband |
| **祝融**<br>Zhurong | 高热性能推进/极限性能<br>Performance | 极限性能测试<br>Performance testing | warband |

### G. 文档/品牌/文化表达层 | Documentation/Brand/Culture Layer

| 人物<br>Figure | 定位<br>Positioning | 核心权力<br>Core Power | 适用层级<br>Layer |
|---------------|--------------------|----------------------|----------------|
| **李白**<br>Li Bai | 风格、气势、表达、语言美学<br>Style & Expression | 品牌文案、对外表达<br>Brand copywriting | special_office |
| **杜甫**<br>Du Fu | 长文、现实感、沉重记录<br>Documentation | 年报、复盘、制度纪要<br>Annual reports | external_relations |
| **苏轼**<br>Su Shi | 综合表达、公共沟通<br>Communication | 外部沟通、内容战略<br>External communication | external_relations |
| **庄子**<br>Zhuangzi | 抽象思辨、边界松动<br>Philosophy | 发散、概念创新<br>Concept innovation | advisor |

### H. 情报/谋略/战略层 | Intelligence/Strategy Layer

| 人物<br>Figure | 定位<br>Positioning | 核心权力<br>Core Power | 适用层级<br>Layer |
|---------------|--------------------|----------------------|----------------|
| **鬼谷子**<br>Guiguzi | 博弈/谈判/对手建模<br>Game Theory | 钦天部主官、商务策略<br>Strategy chief | supervisory |
| **张良**<br>Zhang Liang | 谋划/隐性布局<br>Covert Planning | 战略副手、长线布局<br>Strategic deputy | advisor |
| **孙武**<br>Sun Tzu | 战法/配置/兵力调度<br>Warfare | 竞争策略、试验攻防<br>Competitive strategy | warband |
| **刘伯温**<br>Liu Bowen | 大局预判/结构性趋势<br>Trend Prediction | 长周期预判、战略风控<br>Long-term prediction | advisor |

### I. 基础文明/母体/长时层 | Foundation/Civilization/Long-term Layer

| 人物<br>Figure | 定位<br>Positioning | 核心权力<br>Core Power | 适用层级<br>Layer |
|---------------|--------------------|----------------------|----------------|
| **伏羲**<br>Fuxi | 规则建模/结构抽象<br>Pattern Modeling | 元架构设计<br>Meta-architecture | supervisory |
| **女娲**<br>Nüwa | 修补/连续性/恢复<br>Restoration | 结构缝补、连续性保障<br>Structural repair | supervisory |
| **神农**<br>Shennong | 试错/资源平衡/实验归纳<br>Trial & Error | 实验归纳、资源平衡<br>Experimental induction | supervisory |
| **黄帝**<br>Yellow Emperor | 整合/文明组织/总协调<br>Integration | 高层文明叙事、总整合<br>Civilization narrative | advisor |

---

## 使用指南 | Usage Guide

### 二级机构选配 | Secondary Institution Selection

```
韩非司 — 制度与权限治理 / Governance & Permissions
张衡台 — 监控与仪表盘 / Monitoring & Dashboards  
扁鹊局 — 系统诊断 / System Diagnosis
太乙台 — 危机修补 / Crisis Repair
狄仁杰司 — 异常调查 / Anomaly Investigation
墨子局 — 成本优化工程 / Cost Optimization
后羿组 — 关键瓶颈清除 / Bottleneck Elimination
共工台 — 破坏性测试 / Destructive Testing
```

### 专项计划代号 | Project Codenames

```
李白计划 — 品牌内容系统 / Brand Content System
韩非计划 — 权限治理与审批链 / Permission Governance
孙武计划 — 竞争策略与市场攻防 / Competitive Strategy
墨子计划 — 成本压缩工程 / Cost Compression
张衡计划 — 系统观测平台 / Observation Platform
扁鹊计划 — 诊断与预警系统 / Diagnosis & Early Warning
```

### 人格选择原则 | Selection Principles

1. **先有空缺，再选人格**: 先有组织功能需求，再找合适人格
   *First have functional needs, then match personas*
   
2. **冲突是特性不是 bug**: 选择天然有冲突的人格，制造组织张力
   *Conflict is a feature, not a bug — create organizational tension*
   
3. **权力驱动必须明确**: 每个人格必须有明确的 power_drive 和 red_lines
   *Power drives must be explicit — every persona needs clear power_drive and red_lines*
   
4. **分层**: core_governance > supervisory > warband > special_office
   *Tiered hierarchy: core_governance > supervisory > warband > special_office*

---

## 核心部门 | Core Departments

席位持有权力，部门执行工作。

| 部门<br>Department | 功能<br>Function | 必要性<br>Necessity |
|-------------------|-----------------|-------------------|
| **工程部**<br>Engineering | 实现和技术交付<br>Implementation & Delivery | 没有工程部，什么也建不成<br>Without it, nothing gets built |
| **审计部**<br>Audit | 独立审查和问责<br>Independent Review | 没有审计，自我批准取代问责<br>Without it, self-approval replaces accountability |
| **风控部**<br>Risk Control | 风险检测和门禁<br>Risk Detection & Gates | 没有风控，坏输出走得太远<br>Without it, bad outputs travel too far |
| **监控部**<br>Monitoring | 运行时可见性和预警<br>Runtime Visibility | 没有监控，失败发现得太晚<br>Without it, failures discovered too late |
| **平台部**<br>Platform | 编排和基础设施协调<br>Orchestration | 没有平台，执行碎片化<br>Without it, execution fragments |
| **档案部**<br>Archives | 证据保存，关闭，历史连续性<br>Evidence Preservation | 没有档案，无法保存制度记忆<br>Without it, no institutional memory |

---

## 生产状态 | Production Status

🟢 **Controlled Production** — Since RUN-011

已验证机制 | Verified Mechanisms:
- ✅ 否决 / Veto
- ✅ 冲突解决 / Conflict Resolution
- ✅ 裁决 / Adjudication  
- ✅ 回滚 / Rollback
- ✅ 归档 / Archive
- ✅ 终止 / Termination
- ✅ 真实外部输入处理 / Real External Input Handling
- ✅ 多轮稳定性 / Multi-Run Stability (17+ rounds)

---

## 治理原则 | Governance Principles

| 中文 | English |
|------|---------|
| 权力必须明确 | Authority must be explicit |
| 执行不得自我批准 | Execution must not self-approve |
| 决策必须可追溯 | Decisions must be traceable |
| 分歧必须可治理 | Disagreements must be governable |
| 挑战必须正式，不是修辞 | Challenges must be formal, not rhetorical |
| 回滚必须可执行 | Rollback must be executable |
| 归档必须可索引 | Archive must be indexable |
| 终止必须明确 | Termination must be explicit |
| 生产行为必须账本化 | Production actions must be ledgered |
| 治理必须强于便利 | Governance must be stronger than convenience |

---

## License

MIT — We open source the governance framework.  
The civilization metaphor is ours.

<div align="center">

### 真龙，不是龙虾。

### True Dragon. Not Claw.

</div>
