# Runtime Architecture | 运行时架构

DragonCore uses a custom-built runtime optimized for governance-first multi-agent execution. This document describes the technical design choices.

---

## Design Philosophy

DragonCore's runtime is designed around three core principles:

1. **True Parallelism**: Agents run concurrently, not sequentially
2. **Fault Isolation**: One agent's failure doesn't compromise others
3. **Stateless Execution**: Each run starts from a clean, reproducible state

---

## Core Components

### 1. Process Isolation Layer

DragonCore uses **tmux** (terminal multiplexer) for process isolation:

```bash
# Each governance seat gets its own tmux window
# All 19 seats can be active simultaneously
# Human operators can attach and observe in real-time

$ tmux new-session -d -s dragoncore
$ tmux new-window -t dragoncore:1 -n "tianshu"
$ tmux new-window -t dragoncore:2 -n "tianxuan"
# ... etc for all 19 seats
```

**Benefits:**
- True parallelism: Multiple agents execute simultaneously on separate CPU cores
- Observability: Human operators can `tmux attach` to watch any seat in real-time
- Fault isolation: One crashing agent doesn't affect others
- Resource limits: Per-pane resource constraints possible

### 2. Execution Environment

Each governance run executes in an isolated **Git worktree**:

```bash
# Create isolated execution environment
$ git worktree add runs/run-042 main
$ cd runs/run-042

# Run executes in this clean environment
$ dragoncore execute --run-id=run-042

# After completion, worktree preserves state for audit
```

**Benefits:**
- No state pollution between runs
- Full git history available for reference
- Runs are reproducible: same commit = same starting state
- Parallel runs possible: multiple worktrees active simultaneously

### 3. Agent Concurrency Model

Unlike systems using cooperative multitasking (async/await), DragonCore uses **preemptive multitasking**:

| Aspect | DragonCore | Traditional Python Agents |
|--------|-----------|---------------------------|
| Concurrency | True OS processes | Event loop (asyncio) |
| Parallelism | Multi-core | Single-core with context switching |
| Blocking | One blocks, others continue | Can block entire event loop |
| Memory | Process-isolated | Shared interpreter memory |

### 4. Governance Kernel

The runtime implements the 19-seat governance protocol:

```rust
// Simplified conceptual structure
struct GovernanceRun {
    run_id: String,
    seats: HashMap<SeatId, Seat>,
    state: RunState,
    ledger: Ledger,
}

struct Seat {
    authority: Vec<Authority>,
    boundary: RoleBoundary,
    handoff_rules: Vec<HandoffRule>,
}

enum Authority {
    Suggest,
    Review,
    Veto,
    Approve,
    Execute,
    FinalGate,
    Archive,
    Terminate,
}
```

---

## Language Choice: Rust

DragonCore's runtime is implemented in **Rust** for:

### Memory Efficiency
- No garbage collector pauses
- Predictable memory usage
- Zero-cost abstractions

### Performance
- Native machine code execution
- No interpreter overhead
- True multi-core parallelism (no GIL)

### Safety
- Compile-time memory safety guarantees
- No null pointer dereferences
- No data races in concurrent code

### Startup Speed
- Cold start in < 50ms
- Suitable for high-frequency governance checks
- Minimal overhead for short-lived runs

---

## Model Support

DragonCore is designed to work with **domestic Chinese models** by default:

- **Kimi** (Moonshot AI)
- **DeepSeek** (DeepSeek AI)
- **Qwen** (Alibaba Cloud)

Overseas models can be enabled via configuration, but domestic models are the default and primary support target.

---

## Skill Compatibility

DragonCore maintains compatibility with existing skill/plugin ecosystems while enforcing governance boundaries:

- Skills execute within seat authority constraints
- All skill outputs pass through relevant gates (Yuheng for quality, Tianxuan for risk)
- Skill failures trigger appropriate escalation paths

---

## Production Considerations

### Resource Requirements

| Component | Typical Usage |
|-----------|---------------|
| Base runtime | 15-30 MB RAM |
| Per active seat | +5-10 MB RAM |
| Git worktree overhead | Minimal (reference to main repo) |
| Concurrent runs | Limited by available cores |

### Monitoring

The runtime exposes metrics via:
- Structured logs (JSON)
- Ledger entries (CSV)
- tmux pane status

### Security

- Each seat runs with minimal required permissions
- No seat can modify its own authority boundary
- Escalation paths require multi-seat consensus
- All authority exercises are logged immutably

---

## Comparison with Alternative Approaches

| Feature | DragonCore | Single-Process Python | Container-per-Agent |
|---------|-----------|---------------------|-------------------|
| Startup time | < 50ms | 500ms-2s | 1-5s |
| Memory overhead | Low | Medium | High (per container) |
| Isolation | Process-level | None | Container-level |
| Observability | High (tmux attach) | Medium | Medium (logs only) |
| Complexity | Medium | Low | High (orchestration) |

DragonCore occupies a middle ground: more robust than single-process systems, lighter than full container orchestration.

---

## Future Roadmap

### Near-term
- [ ] WebSocket interface for real-time seat communication
- [ ] Metrics export (Prometheus format)
- [ ] Distributed runs across multiple hosts

### Long-term
- [ ] WASM-based skill sandboxing
- [ ] Formal verification of governance state machines
- [ ] Hardware security module (HSM) integration for critical decisions
