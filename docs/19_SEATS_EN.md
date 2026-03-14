# The 19 Seats of DragonCore Governance

**Purpose**: Complete authority definitions for DragonCore's 19-seat governance kernel.

**Current Status**: v0.2.1 verified — All 19 seats operational in single-node JSON-backed path.

---

## Overview

DragonCore's governance kernel consists of **19 fixed seats** — not decorative personas, but governance units with explicit authority boundaries, power drives, and core conflicts.

The 19 seats are organized into three layers:
- **Layer 1**: Seven Northern Stars (北斗七星 Beidou Qixing) — Core governance
- **Layer 2**: Four Symbols (四象 Sixiang) — Campaign-level balance  
- **Layer 3**: Eight Guardian Immortals (八仙护法 Baxian Hufa) — Specialized functions

---

## Layer 1: Seven Northern Stars (北斗七星 Beidou Qixing)

The main governance spine. Corresponds to C-suite roles, but with **separated powers** — no single seat can self-approve.

| Seat | Mythic Archetype | Governance Role | Authority | Core Conflict |
|------|------------------|-----------------|-----------|---------------|
| **Tianshu**<br>天枢 | Polaris / Celestial Emperor | **CEO / Final Arbiter**<br>Final gate, ultimate veto, constitutional center | Right to define mainline<br>Right of final decision | vs Yaoguang (boundary-breaking)<br>vs Qinglong (resource dispersion) |
| **Tianxuan**<br>天璇 | Dubhe / Star of Change | **COO / Risk Guardian**<br>Resource allocation, compliance review | Right to allocate resources<br>Right to control process | vs Diguan (conflicting standards)<br>vs Nezha (disrupting rhythm) |
| **Tianji**<br>天玑 | Merak / Star of Strategy | **CTO / Technical Lead**<br>Tech stack definition, architecture standards | Right to define technical standards<br>Right to set architecture | vs Kaiyang (rushing deadlines)<br>vs Yaoguang (technical risk) |
| **Tianquan**<br>天权 | Phecda / Star of Balance | **CSO / Strategy Definition**<br>Issue definition, track prioritization | Right to prioritize strategy<br>Right to define issues | vs Yaoguang (new tracks)<br>vs Zhugeliang (analysis depth) |
| **Yuheng**<br>玉衡 | Alioth / Star of Measurement | **CRO / Quality Gate**<br>Standard setting, veto power | Right to veto on quality<br>Right to set standards | vs Yaoguang (radical innovation)<br>vs Nezha (rapid strike) |
| **Kaiyang**<br>开阳 | Mizar / Martial Star | **Implementation Review**<br>Task scheduling, delivery accountability | Right to schedule execution<br>Right to control delivery | vs Yuheng (risk control delays)<br>vs Baozheng (audit depth) |
| **Yaoguang**<br>瑶光 | Alkaid / Breaking Army Star | **Innovation & Archive**<br>Innovation budget, boundary-breaking resources | Right to innovate<br>Right to break boundaries | vs Yuheng (conservative risk control)<br>vs Tianshu (mainline constraints) |

### Power Balance of the Seven Stars

**Top-level tensions**:
- **Tianshu (Stability)** ↔ **Yaoguang (Change)**: The eternal war between maintaining order and pursuing innovation
- **Tianxuan (Resources)** ↔ **Tianji (Technology)**: Who controls the budget vs who controls the stack
- **Tianquan (Planning)** ↔ **Kaiyang (Execution)**: Strategy vs delivery reality
- **Yuheng (Gate)** ↔ **All**: Risk control inevitably conflicts with speed

---

## Layer 2: Four Symbols (四象 Sixiang)

Four directional deities responsible for **campaign-level balance** — managing the tension between exploration and stability.

| Seat | Direction | Mythic Archetype | Governance Role | Core Rule |
|------|-----------|------------------|-----------------|-----------|
| **Qinglong**<br>青龙 | East | Azure Dragon (Mengzhang) | **New Track Exploration**<br>Exploration budget, opportunity capture | Must define **stop condition** |
| **Baihu**<br>白虎 | West | White Tiger (Jianbing) | **Red Team / Stress Test**<br>High-risk testing, failure modes | Must give **fix window** |
| **Zhuque**<br>朱雀 | South | Vermilion Bird (Lingguang) | **External Narrative**<br>Brand voice, promise review | Must wait for **Baihu verification** |
| **Xuanwu**<br>玄武 | North | Black Tortoise (Zhiming) | **Stability Assurance**<br>Stability budget, monitoring resources | Must give **Qinglong exploration space** |

### Campaign Rules

1. **Qinglong** (Exploration): Can seize new opportunities, but **must define stop condition** — when to quit if it's not working
2. **Baihu** (Verification): Can break things to find failure modes, but **must give fix window** — time to repair before declaring failure
3. **Zhuque** (Narrative): Can make promises externally, but **must wait for Baihu verification** — no marketing before stress testing
4. **Xuanwu** (Stability): Can demand stability, but **must give Qinglong exploration space** — can't block all innovation

### Core Conflicts

- **Qinglong (Explore)** ↔ **Baihu (Verify)**: The fundamental tension — you can't explore and verify simultaneously
- **Zhuque (Promise)** ↔ **Baihu (Verify)**: Marketing wants to promise, testing wants to verify
- **Qinglong (New)** ↔ **Xuanwu (Stable)**: New directions threaten stability; stability blocks new directions

---

## Layer 3: Eight Guardian Immortals (八仙护法 Baxian Hufa)

Specialized function seats providing authority **beyond conventional bureaucracy**. These are the "checks and balances" that prevent the Seven Stars from self-approving.

| Seat | Name | Mythic Archetype | Core Authority | Why This Figure |
|------|------|------------------|----------------|-----------------|
| **Yangjian**<br>杨戬 | Erlang Shen | Three-Eyed God / "Listen to调 but not to宣" | **Quality Inspection**<br>Metric verification, seeing through deception | Possesses the **Heavenly Eye (天眼)** that sees through all deception. Detects "fake progress" when agents claim completion but haven't delivered. |
| **Baozheng**<br>包拯 | Lord Bao | Song Dynasty Judge / "Iron face, no favoritism" | **Independent Audit**<br>Bypass three departments, direct report, liability trace | **Black face** = impartiality, **crescent moon** = clear night vision. Cannot be bribed or intimidated. Audits power abuse, delivers verdicts. |
| **Zhongkui**<br>钟馗 | Ghost Catcher | Demon Queller / Ghost King | **Anomaly Purge**<br>Pollution cleanup, malicious interception | **Ghost King** with sword, specialized in hunting evil spirits. Eliminates toxic agents, removes compromised components. |
| **Luban**<br>鲁班 | Master Craftsman | Patron of Craftsmen / "Saint of Hundred Crafts" | **Engineering Platform**<br>Toolchain standards, automation framework | Invented **saw, ladder, countless tools**. Represents highest craftsmanship. Ensures code is elegant, maintainable, lasting. |
| **Zhugeliang**<br>诸葛亮 | Dragon Strategist | Shu Chancellor / "Wisdom Incarnate" | **Chief Advisor**<br>Complex planning, multi-line deduction | Master of **Empty Fort Strategy**. Handles complex multi-step campaigns, crisis planning, when straightforward solutions fail. |
| **Nezha**<br>哪吒 | Third Prince | Lotus Incarnation / Wind-Fire Wheels | **Rapid Deployment**<br>Fast breakthrough, deadlock piercing | **Lotus incarnation** with cosmic rings and fire-tipped spear. Represents **speed and decisive action**. Rapid deployment, crisis management. |
| **Xiwangmu**<br>西王母 | Queen Mother | Queen of Immortals / Peach Garden Owner | **Scarce Resources**<br>High-tier quotas, long-term reserves | Guards **Peaches of Immortality** — ultimate scarce resource. Decides who gets what, when, why. Manages compute budgets, API quotas. |
| **Fengdudadi**<br>丰都大帝 | Lord of Underworld | Emperor of Fengdu / Ghost Emperor | **Termination & Archive**<br>Deadline judgment, risk sealing | Rules **Underworld** — realm of endings, judgment, afterlife. Power to **terminate** projects and **archive** eternally. |

### Functional Coverage

| Function | Seats |
|----------|-------|
| **Supervisory** | Yangjian (Quality), Baozheng (Audit), Zhongkui (Purge) |
| **Engineering** | Luban (Platform), Zhugeliang (Planning) |
| **Execution** | Nezha (Rapid) |
| **Resources** | Xiwangmu (Scarce) |
| **Finality** | Fengdudadi (Termination) |

---

## The Complete Conflict Network

DragonCore governance is **not harmonious** — it is full of **constructive tension**.

### Top-Level Conflicts
- **Tianshu (Fixed)** ↔ **Yaoguang (Change)**: Stability vs Innovation war at the top
- **Tianxuan (Allocate)** ↔ **Tianji (Control)**: Eternal struggle over resources vs technology

### Risk Control Conflicts
- **Yuheng (Gate)** ↔ **Everyone**: Risk control vs efficiency

### Campaign Conflicts
- **Qinglong (Explore)** ↔ **Baihu (Verify)**: Exploration vs verification tension
- **Zhuque (Narrative)** ↔ **Baihu (Verify)**: Promise vs verification conflict
- **Qinglong (New)** ↔ **Xuanwu (Stable)**: New direction vs stability contradiction

### Functional Conflicts
- **Baozheng (Audit)** ↔ **Kaiyang (Execute)**: Audit depth vs delivery speed
- **Nezha (Fast)** ↔ **Yuheng (Gate)**: Rapid strike vs risk control
- **Yangjian (Truth)** ↔ **Zhuque (Beauty)**: Real data vs brand packaging

---

## Communication Styles by Faction

Different seats have distinctly different communication styles:

### Conservative Faction (Yuheng, Xuanwu, Baozheng, Fengdudadi)
```
"Risk level is X, cannot pass."
"Monitoring coverage below threshold."
"This matter requires audit."
"Not everything deserves to live."
```

### Aggressive Faction (Yaoguang, Qinglong, Nezha)
```
"Current solution has hit ceiling, must try new approach."
"New opportunity window: only X months."
"This has dragged too long, decide today."
"I bear failure, we share success."
```

### Verification Faction (Baihu, Yangjian, Zhongkui)
```
"Without Baihu stress test, any launch is gambling."
"Surface data says X, deep analysis says Y."
"Anomaly detected, immediate purge required."
```

### Resource Faction (Tianxuan, Xiwangmu, Luban)
```
"Resource utilization data shows..."
"This resource is scarce, only X units available."
"This operation repeats X times/day, must toolify."
```

---

## Authority Levels

Each seat has explicitly defined authority levels:

| Level | Description | Example Seats |
|-------|-------------|---------------|
| **suggest** | Can recommend, no binding power | All seats can suggest |
| **review** | Can examine and comment | Kaiyang, Yangjian |
| **veto** | Can block with documented reason | Tianxuan, Yuheng, Baozheng, Baihu |
| **approve** | Can authorize passage | Tianshu, Tianquan |
| **execute** | Can implement directly | Kaiyang, Nezha |
| **final_gate** | Ultimate decision, no appeal | Tianshu only |
| **archive** | Can preserve to institutional memory | Yaoguang, Fengdudadi |
| **terminate** | Can end permanently | Fengdudadi only |

---

## Design Rationale

### Why 19, Not 18?

With 18 seats, critical powers collapse:
- Some seat ends up self-approving
- Veto chains become circular
- No true final authority

### Why 19, Not 20?

With 20 seats:
- Coordination cost exceeds governance benefit
- Accountability diffuses
- Ceremony replaces control

### Why This Specific 19?

The 19 seats map to **12 essential governance functions**:

1. **Initiation & execution** — Tianquan
2. **Technical supervision** — Kaiyang / Luban
3. **Risk review** — Tianxuan / Baihu
4. **Quality review** — Yuheng
5. **Independent audit** — Baozheng / Yangjian
6. **Challenge & block** — Baihu / Yuheng
7. **Conflict resolution** — Tianshu
8. **Escalation of unresolved conflicts** — Tianxuan / Tianshu
9. **Rollback of bad outcomes** — Tianshu / Nezha
10. **Final archive** — Yaoguang / Fengdudadi
11. **Termination of dangerous branches** — Fengdudadi / Zhongkui
12. **Final authority reservation** — Tianshu

**19 is the minimum where all 12 functions can be held by separate seats.**

---

## Operational Notes

### In v0.2.1 Verified Path

All 19 seats are functional in the single-node JSON-backed path:

```bash
# Example: Multi-seat governance workflow
dragoncore run --run-id demo --input-type code -t "Implement feature"
dragoncore execute --run-id demo --seat Tianquan --task "Create plan"
dragoncore execute --run-id demo --seat Tianji --task "Review architecture"
dragoncore execute --run-id demo --seat Yuheng --task "Quality gate"
dragoncore veto --run-id demo --seat Yuheng --reason "Security issue"  # If needed
dragoncore final-gate --run-id demo --approve  # Tianshu only
dragoncore archive --run-id demo --seat Yaoguang
```

### Authority Verification

Each seat's authority is enforced at runtime:
- Only Yuheng, Tianxuan, Baozheng, Baihu can `veto`
- Only Tianshu can `final-gate`
- Only Fengdudadi can `terminate`
- Only Yaoguang and Fengdudadi can `archive`

Attempting unauthorized actions results in:
```
Error: Seat {name} does not have {authority} authority
```

---

## Further Reading

- [USAGE_GUIDE_EN.md](USAGE_GUIDE_EN.md) — How to use seats in practice
- [VERIFICATION_REPORT.md](VERIFICATION_REPORT.md) — Evidence that 19 seats work
- [HUAXIA_REGISTRY.md](HUAXIA_REGISTRY.md) — Extended persona registry for secondary institutions

---

**Governance Status**: 19 seats operational | v0.2.1 verified

**True Dragon. Not Claw.**
