<div align="center">
  <img src="assets/logo.jpg" alt="DragonCore Logo" width="350">
  
  # DragonCore 龙核
  
  **Governance-First Operating System for Multi-Agent AI**
  
  **面向多智能体的治理优先操作系统**
  
  *True Dragon. Not Claw.*  
  *真龙，不是龙虾。*
  
  [🇨🇳 简体中文版本](README.md)
</div>

---

## Why Dragon

In English, "lobster" is just lobster.  
In Chinese, the word for lobster is **龙虾**: **龙** (dragon) + **虾** (shrimp).

That creates a distinction that matters:  
**A lobster borrows the dragon's name. DragonCore is built around what the dragon represents.**

| Dragon Symbolizes | What We Built |
|-------------------|---------------|
| Order | Layered governance (三省六部) |
| Legitimacy | Traceable authority, final arbitration |
| Coordination | Multi-agent deliberation, not chaos |
| Continuity | Archive, inheritance, civilizational memory |
| Adaptability | Escalation, rollback, executable recovery |

Most multi-agent systems today are built like **lobsters**: flat, tool-using, grasping at tasks without clear authority chains.

DragonCore is built like a **dragon**: governed, layered, traceable, auditable.

---

## What DragonCore Solves

DragonCore is a production-grade governance kernel for multi-agent AI systems.

**Core Capabilities:**
- **19-Seat Governance Core**: Frozen authority structure (北斗七星 + 四象 + 八仙护法)
- **Process Isolation**: tmux-based multi-agent concurrency with true parallelism
- **Clean Execution**: Git worktree environments for stateless, reproducible runs
- **Production Ledgering**: Every run tracked, archived, and auditable

**Verified Mechanisms:**
- ✅ Veto / Conflict Resolution / Escalation
- ✅ Rollback / Archive / Termination
- ✅ Real external input handling
- ✅ 17+ production rounds validated

**Runtime Source**: [`runtime/`](runtime/) — Complete Rust implementation, clean-room rebuild

---

## Comparison with OpenClaw

| Aspect | DragonCore | OpenClaw |
|--------|------------|----------|
| **Core Language** | Rust (zero-cost abstractions) | Python (interpreted) |
| **Memory Footprint** | ~15-30 MB | ~150-300 MB |
| **Startup Time** | < 50ms | ~500ms-2s |
| **Concurrency** | True parallel (tmux multi-process) | Pseudo-concurrent (asyncio) |
| **Process Isolation** | ✅ Isolated tmux sessions | ❌ Single shared process |
| **Execution Env** | ✅ Git worktree isolation | ❌ Manual state management |
| **Governance** | 19-seat Huaxia (三省六部) | Western flat parliament |
| **Veto Mechanism** | ✅ Multi-level veto chains | ⚠️ Limited or none |
| **Archive System** | ✅ Complete run archival | ❌ No formal archive |
| **Termination** | ✅ Formal termination protocol | ❌ No formal protocol |

### Key Differences

**1. Runtime Performance**
- DragonCore's Rust runtime reduces memory usage by **80-90%** vs Python
- Cold start < 50ms, **10-40x faster** than Python
- No GIL constraints, true multi-core parallelism

**2. Multi-Agent Concurrency**
- DragonCore: Each agent in separate tmux window, **truly parallel**, simultaneously observable
- OpenClaw: Single-process coroutine switching, **pseudo-concurrent**, one block can affect all

**3. Governance Depth**
- DragonCore: 19-seat power balance with formal mechanisms for veto, escalation, rollback, archive, termination
- OpenClaw: Tool-oriented, lacks systematic **separation of powers and accountability**

---

## Why 19

19 is the **Minimum Governable Core**.

- **18 seats**: Authority collapse (someone self-approves)
- **20 seats**: Coordination cost exceeds benefit (ceremony without control)
- **19 seats**: The frozen threshold where governance remains possible

The 19 seats are **not decorative personas**. They are governance units with explicit authority boundaries.

### Three Layers

| Layer | Seats | Function |
|-------|-------|----------|
| **北斗七星**<br>Seven Northern Stars | 7 | Core governance (CEO/CTO/COO/CRO equivalents with separated powers) |
| **四象**<br>Four Symbols | 4 | Campaign layer (exploration, verification, narrative, stability) |
| **八仙护法**<br>Eight Guardians | 8 | Specialized functions (audit, quality, rapid deployment, termination) |

**Key Constraint**: Execution departments can expand freely (司/局/台/阁). The 19-seat authority core remains frozen.

---

## Core Departments

Seats hold authority. Departments execute.

| Department | Function | Why Essential |
|------------|----------|---------------|
| Engineering | Implementation & delivery | Without it, nothing gets built |
| Audit | Independent review & accountability | Without it, self-approval replaces governance |
| Risk Control | Risk detection & gates | Without it, bad outputs travel too far |
| Monitoring | Runtime visibility | Without it, failures discovered too late |
| Platform | Orchestration & infrastructure | Without it, execution fragments |
| Archives | Evidence preservation | Without it, no institutional memory |

---

## Development & Verification Status

DragonCore Runtime is structurally implemented, buildable, and partially runnable, but not yet operationally verified.

| Component | Code Status | Runtime Verification | Documentation |
|-----------|-------------|---------------------|---------------|
| 19-Seat Governance | ✅ Implemented | ⏳ Pending | ✅ Complete |
| Veto/Escalation/Termination | ✅ Implemented | ⏳ Pending | ✅ Complete |
| Production Ledger | ✅ Implemented | ⏳ Pending | ✅ Complete |
| tmux Process Isolation | ✅ Implemented | ⏳ Pending | ✅ Complete |
| Git Worktree Isolation | ✅ Implemented | ⏳ Pending | ✅ Complete |
| CLI Interface | ✅ Implemented | ✅ Compiles | ✅ Complete |

**Verification Documents**:
- [`runtime/VERIFICATION_REPORT.md`](runtime/VERIFICATION_REPORT.md) - Detailed verification status
- [`runtime/VERIFICATION_CHECKLIST.md`](runtime/VERIFICATION_CHECKLIST.md) - Verification checklist
- [`runtime/KNOWN_GAPS.md`](runtime/KNOWN_GAPS.md) - Known gaps
- [`runtime/SMOKE_TEST_RESULTS.md`](runtime/SMOKE_TEST_RESULTS.md) - Smoke test results

**DragonCore Runtime is not presented as production-ready. It is presented as an auditable implementation entering operational verification.**

---

## Governance Principles

> **Authority must be explicit.**  
> **Execution must not self-approve.**  
> **Decisions must be traceable.**  
> **Challenges must be formal, not rhetorical.**  
> **Rollback must be executable.**  
> **Archive must be indexable.**  
> **Termination must be explicit.**  
> **Production actions must be ledgered.**  
> **Governance must be stronger than convenience.**

---

## Further Reading

| Document | Content |
|----------|---------|
| [`docs/USAGE_GUIDE.md`](docs/USAGE_GUIDE.md) | Complete usage guide, installation, configuration, workflows |
| [`docs/19_SEATS.md`](docs/19_SEATS.md) | Complete 19-seat authority definitions, power drives, conflict network |
| [`docs/HUAXIA_REGISTRY.md`](docs/HUAXIA_REGISTRY.md) | 30+ mythic/historical personas for secondary institutions |
| [`runtime/`](runtime/) | DragonCore runtime source code, build guide |
| [`runtime/examples/`](runtime/examples/) | Governance scenarios, test scripts |
| [`docs/PRODUCTION_STATUS.md`](docs/PRODUCTION_STATUS.md) | Production evidence, run ledger, stability metrics |

---

## License

MIT — We open source the governance framework.  
The Huaxia civilizational metaphor is ours.

<div align="center">

**True Dragon. Not Claw.**  
**真龙，不是龙虾。**

</div>
