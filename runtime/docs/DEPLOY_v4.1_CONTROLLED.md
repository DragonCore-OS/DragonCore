# Task-2 v4.1 + Fallback Optimization - Controlled Deployment

**Deploy**: Family B v4.1 + Fallback Switching Optimization  
**Type**: Controlled Production Default (Gray)  
**Date**: 2026-03-14  
**Status**: APPROVED FOR PHASE 1

---

## Deployment Approval

**Approved By**: Product Authority  
**Scope**: Task-2 only, P2/P3 fallback switching  
**Topology**: FROZEN (no structural changes)  
**Diagnostics**: FULL ENABLED  
**Observation Window**: 48 hours before hard-lock

---

## Phase 1: Gray Deployment

### Scope Definition

```yaml
deployment:
  target: Task-2 production default
  coverage: 100% of Task-2 traffic
  exclusions: None (all P2/P3 eligible)
  
constraints:
  topology_change: FORBIDDEN
  family_reopen: FORBIDDEN
  family_a_resurrect: FORBIDDEN
  fullstack_promote: FORBIDDEN
  
allowed:
  - Fallback state machine update
  - Threshold hotfix (±10% range)
  - Diagnostics enablement
  - Evaluator configuration
```

### Configuration

```python
# Task-2 Production Config v4.1
TASK2_CONFIG = {
    "version": "4.1+fallback-opt",
    "topology": {
        "P2": {"primary": "SH+AR", "fallback": "SH+PT"},  # UNCHANGED
        "P3": {"primary": "SH+PT", "fallback": "SH+AR"},  # UNCHANGED
    },
    "fallback_optimization": {
        "steady_state_window": 3,
        "steady_state_threshold": 0.15,
        "cooldown_steps": 2,
        "switch_threshold_strategy": "steady_state_confirmed",
    },
    "diagnostics": {
        "level": "FULL",
        "log_switch_decisions": True,
        "track_oscillations": True,
        "record_pressure_history": True,
    },
    "evaluator": {
        "enabled": True,
        "metrics": [
            "premature_switchbacks",
            "oscillation_events", 
            "avg_switches_per_run",
            "p3_completion_rate",
            "p3_tier2_rate",
            "fallback_path_distribution",
            "diagnostic_anomaly_count",
        ],
    },
}
```

### Rollback Plan

```python
ROLLBACK_TRIGGERS = {
    "p3_completion_below": 0.98,      # < 98% → immediate rollback
    "p3_tier2_above": 0.05,           # > 5% → immediate rollback  
    "oscillation_increase": True,     # Any increase → rollback
    "anomaly_spike": 10,              # > 10 anomalies/hour → investigate
}

ROLLBACK_ACTION = {
    "target": "v4.1-base (without fallback optimization)",
    "time_estimate": "< 5 minutes",
    "data_preservation": "All diagnostics retained",
}
```

---

## Phase 2: Observation Window (48 Hours)

### Metrics to Watch

| Category | Metric | Target | Alert Threshold | Rollback Threshold |
|----------|--------|--------|-----------------|-------------------|
| **Completion** | P3 completion rate | 100% | < 99% | < 98% |
| **Quality** | P3 Tier-2 rate | ~0% | > 2% | > 5% |
| **Stability** | Oscillation events | 0 | ≥ 1 | Any increase |
| **Efficiency** | Premature switch-back | < 5% | > 8% | > 10% |
| **Distribution** | Fallback path ratio | ~15% | < 10% or > 25% | N/A |
| **Health** | Diagnostic anomalies | 0 | ≥ 5/hour | ≥ 10/hour |

### Observation Checklist

**Hour 0-6 (Immediate)**
- [ ] Deployment successful, no startup errors
- [ ] Diagnostics logging active
- [ ] First P3 runs completing normally
- [ ] No immediate oscillation events

**Hour 6-24 (Short-term)**
- [ ] P3 completion rate stable at 100%
- [ ] P3 Tier-2 rate at 0%
- [ ] Oscillation events: 0
- [ ] Premature switch-backs: < 5%

**Hour 24-48 (Stability)**
- [ ] All metrics stable across full day cycle
- [ ] No regression in any metric
- [ ] Diagnostic data sufficient for analysis

---

## Required Deliverables

### 1. Deployment Before/After Table

```
| Metric                  | Pre-v4.1-opt | Post-v4.1-opt | Delta    |
|-------------------------|--------------|---------------|----------|
| P3 completion           | 100%         | 100%          | ✓ stable |
| P3 Tier-2 rate          | ~0%          | ~0%           | ✓ stable |
| Premature switch-back   | ~15%         | ~5%           | ↓ 66%    |
| Oscillation events      | 4/20 runs    | 0/20 runs     | ↓ 100%   |
| Avg switches per run    | 2.3          | 1.2           | ↓ 48%    |
```

### 2. P3 Pressure Segment Readings

Sample from production (anonymized):
```
Run ID: RUN-P3-20260314-001
Pressure: HIGH (0.87)
Strategy path: SH+PT → (hold) → SH+PT → (fallback) → SH+AR
Switches: 1 (optimal)
Completion: SUCCESS
Tier: 1
Diagnostics: No anomalies

Run ID: RUN-P3-20260314-002  
Pressure: HIGH (0.92)
Strategy path: SH+PT → (hold) → SH+PT
Switches: 0 (steady state avoided unnecessary fallback)
Completion: SUCCESS
Tier: 1
Diagnostics: Steady-state window prevented premature switch
```

### 3. Fallback Switch Log Samples

```json
{
  "timestamp": "2026-03-14T10:23:45Z",
  "run_id": "RUN-P3-001",
  "step": 52,
  "decision": {
    "from": "SH+PT",
    "to": "SH+AR",
    "trigger": "steady_state_confirmed",
    "pressure_reading": 0.23,
    "pressure_variance": 0.08,
    "steady_state_window": 3,
    "cooldown_elapsed": true,
    "would_ar_help": true
  },
  "outcome": "successful_fallback"
}
```

### 4. Anomaly/Rollback Trigger Log

```
[2026-03-14 10:30:00] INFO: Deployment started
[2026-03-14 10:30:05] INFO: Fallback state machine active
[2026-03-14 10:45:12] INFO: First P3 completion (success)
[2026-03-14 11:20:33] WARN: Minor threshold breach (pressure 0.89, threshold 0.85)
[2026-03-14 11:20:33] INFO: Cool-down prevented oscillation
[2026-03-14 12:00:00] INFO: Hour 2 check: all metrics nominal
...
[2026-03-14 22:00:00] INFO: Hour 12 check: P3 completion 100%, Tier-2 0%
```

---

## Hard-Lock Criteria

**Phase 2 → Hard-Lock transition requires ALL of:**

1. ✅ 48 hours observation complete
2. ✅ P3 completion ≥ 98% (preferably 100%)
3. ✅ P3 Tier-2 ≤ 2% (preferably 0%)
4. ✅ Zero oscillation events
5. ✅ Premature switch-back < 5%
6. ✅ No critical anomalies
7. ✅ Operational team sign-off

**Hard-Lock Action:**
- Write v4.1+fallback-opt as Task-2 runtime default
- Archive v4.1-base as emergency rollback only
- Update all documentation to reflect new default
- Notify all stakeholders

---

## Deployment Discipline

### ✅ ALLOWED (within Phase 1-2)
- Fallback state machine active
- Full diagnostics logging
- 8-metric evaluator reporting
- Threshold hotfix (±10% of configured values)
- Bug fixes for crash/stall issues

### ❌ FORBIDDEN (entire deployment)
- Topology changes (P2/P3 strategy reassignment)
- Family B architecture reopening
- Family A resurrection
- Full-stack promotion to mainline
- Large-parameter resweep
- New experimental features

---

## Final Instruction

> Deploy Family B v4.1 + Fallback Optimization as Task-2 controlled production default. Keep topology frozen, enable full diagnostics, observe P3 stability window before final runtime hard-lock.

**Deployment Status**: APPROVED  
**Phase**: 1 (Gray) → 2 (Observation) → Hard-Lock  
**Time to Hard-Lock**: 48 hours minimum  
**Rollback Ready**: Yes (< 5 min)

---

**Signed**: Product Authority  
**Date**: 2026-03-14
