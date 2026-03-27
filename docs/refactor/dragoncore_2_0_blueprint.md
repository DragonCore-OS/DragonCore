# DragonCore 2.0 Refactor Blueprint (Repo-Grounded)

## 1) Executive summary

This blueprint defines a **reviewable, reversible migration** from the current DragonCore runtime toward a DragonCore 2.0 architecture with explicit sovereign governance, explicit multi-brain arbitration, and an externalized tool/research layer boundary.

Key reality check from Phase 0 reconnaissance:
- `DragonCore` repository is present and contains a working Rust runtime + governance/event/state stack.
- `deer-flow` and `paperclip` repositories are **not present in the current workspace checkout**, so direct code-level inventory for those two cannot yet be completed.

Therefore:
- This document includes a complete, repo-grounded design for what can be validated now.
- All cross-repo boundaries are specified as contracts and marked with assumptions requiring confirmation once missing repos are available.
- The first implementation slice (PR-1 equivalent) is docs + boundary contract, with no runtime behavior change.

---

## 2) Current-state repo inventory

### 2.1 Repository availability

| Repository | Present in workspace | Evidence |
|---|---:|---|
| DragonCore | ✅ | `/workspace/DragonCore` exists with `.git` and Rust runtime tree |
| deer-flow | ❌ | No sibling checkout found under `/workspace` |
| paperclip | ❌ | No sibling checkout found under `/workspace` |

### 2.2 DragonCore inventory

#### Language stack
- Primary runtime: **Rust** (`runtime/Cargo.toml`, `runtime/src/**`).
- Ops scripts: **Bash** (`install.sh`, `runtime/launch.sh`, `scripts/launch.sh`).
- Config formats: **TOML** and **YAML** (runtime config samples and test specs).

#### Entrypoints
- Runtime binary CLI: `runtime/src/main.rs` (`dragoncore` CLI commands: `run`, `execute`, `veto`, `final-gate`, `archive`, `terminate`, `status`, `metrics`, `events`, `replay`, `meeting`, `entity`, etc.).
- Launcher scripts: `runtime/launch.sh`, top-level `scripts/launch.sh`.

#### Runtime boundaries (current)
- Governance/state machine: `runtime/src/governance/mod.rs`.
- Runtime orchestration: `runtime/src/runtime/mod.rs`.
- Model/provider abstraction and routing: `runtime/src/models/mod.rs`.
- Persistence: `runtime/src/persistence/mod.rs` (JSON-backed run store).
- Ledger: `runtime/src/ledger/mod.rs` (metrics and run-level accounting).
- Events/DIBL log stream: `runtime/src/events/mod.rs` (JSONL event store + channel/scope metadata).
- Worktree isolation: `runtime/src/worktree/mod.rs`.
- tmux process isolation: `runtime/src/tmux/mod.rs`.
- Meeting protocol: `runtime/src/meeting/mod.rs`.
- Entity subsystem: `runtime/src/entity/*`.

#### Config system
- Strongly typed runtime config in `runtime/src/config/mod.rs`.
- Current major fields:
  - `runtime`, `governance`, `providers`, `seat_models`, `execution`, `ledger`.
- Seat-model mapping exists (`seat_models`) but is provider-name based and still tightly coupled to runtime internals.

#### Test setup
- Unit tests exist in multiple modules:
  - `events`, `meeting`, `ledger`, `persistence`, `entity`.
- Rust test runner via `cargo test`.
- No dedicated multi-repo integration harness yet.

#### Build/deploy path
- Build: `cargo build --release` from `runtime/`.
- Local launch wrapper: `runtime/launch.sh`.
- Installer script compiles and places binary in local bin: `install.sh`.

#### Where model/provider logic currently lives
- `runtime/src/models/mod.rs`:
  - `ModelProvider` trait.
  - provider implementations (`Kimi`, `KimiCli`, `DeepSeek`, `Qwen`, `OpenAI-compatible`).
  - `ModelRouter` with seat mapping and default fallback.
- Runtime wires providers and seat mapping in `runtime/src/runtime/mod.rs`.

#### Where onboarding/UI logic currently lives
- No dedicated UI frontend repo present in workspace.
- Current onboarding is CLI/config oriented (`dragoncore init`, TOML setup).

#### Where research/tooling logic currently lives
- No explicit DeerFlow code in current checkout.
- Some orchestration/event/logging exists but no separate research-worker/tool-layer boundary yet.

### 2.3 “Spec vs repo reality” matrix

| Target requirement | Already exists | Partial | Missing | Notes |
|---|---|---|---|---|
| Sovereign Rust kernel as system of record | ✅ |  |  | Governance + persistence already Rust-native |
| Explicit seat abstraction | ✅ |  |  | `Seat` enum + authorities in governance module |
| Decree/command lifecycle |  | ✅ |  | Lifecycle exists across CLI/governance/events but terminology/protocol not unified as “decree contract” |
| Ledger/event log contract | ✅ |  |  | Ledger + DIBL event store already implemented |
| Identity continuity |  | ✅ |  | Run persistence exists; explicit dynasty/identity continuity contract not formalized |
| Multi-brain protocol (sovereign/advisory/tool/research workers) |  | ✅ |  | Seat/provider mapping exists, but authority protocol across actor types is not explicit |
| Provider registry/model registry |  | ✅ |  | Providers in config+router; no normalized registry schema with trace reasons |
| Structured arbitration trace (“why model selected”) |  | ✅ |  | Provider tagging in events exists; explicit selection reason trace schema missing |
| DeerFlow as tool layer, non-governing |  |  | ✅ | Repo absent + no adapter boundary in code yet |
| paperclip onboarding/control surface |  |  | ✅ | Repo absent + no UI boundary integration yet |
| Unified terminology across docs/config/logs |  | ✅ |  | Mixed modern corporate role labels and sovereign seat semantics coexist |
| Reversible migration slices | ✅ |  |  | Current architecture modular enough for phased migration |

### 2.4 What should not be merged

Given current evidence:
- **Do not merge governance kernel and external tooling runtime into one process by default.** Keep failure domains isolated.
- **Do not make any tool/research worker a hidden authority path.** Final decree authority must remain in DragonCore kernel.
- **Do not hard-couple UI state transitions directly to provider calls.** UI should call kernel APIs/contracts, not models directly.

---

## 3) Target architecture

## 3.1 Core architecture statement

DragonCore 2.0 uses a four-boundary architecture:

1. **Kernel Governance Boundary (DragonCore runtime, Rust)**
   - Canonical state, seat authority, decree lifecycle, identity continuity, ledger/event source of truth.
2. **Model Arbitration Boundary (inside kernel, explicit module/API)**
   - Provider/model registry + seat assignment + fallback/timeout/downgrade policy + structured selection traces.
3. **Tool Layer Boundary (DeerFlow adapter/workers)**
   - Research/search/crawl/python/RAG/MCP/report capabilities as non-sovereign tools.
4. **Operator/UI Boundary (paperclip or equivalent shell)**
   - Onboarding, control surface, decree drafting, status and replay visualization.

## 3.2 Kernel and governance contract (A)

### Canonical source of truth for state
- DragonCore governance + persistence modules remain canonical source.
- Canonical write path: `state transition -> durable write -> event emission`.

### Seat abstraction
- Preserve existing 19-seat authority map as stable governance primitive.
- Add overlay concept:
  - `sovereign_brain_seat` (defaults to Tianshu),
  - `advisory_brain_seats` (mapped to selected seat set).

### Decree/command lifecycle
Proposed lifecycle states:
1. `decree_drafted`
2. `decree_advisory_review`
3. `decree_challenged` (optional)
4. `decree_sovereign_decision`
5. `decree_committed`
6. `decree_executed` (optional)
7. `decree_archived`

### Ledger/event log contract
- Every decree lifecycle transition must emit structured event entries and ledger updates.
- Required event fields: run/decree id, actor type, seat, provider/model, action, outcome, error, trace id.

### Identity continuity rules
- Dynasty/system identity immutable once initialized (except explicit migration command).
- Run IDs remain stable across CLI invocations.
- Seat identity and authority history replayable from event stream.

### Failure/fallback behavior
- If advisory/model/tool fails: sovereign decision path still available with degraded mode.
- If persistence write fails: reject transition (no in-memory-only success).

## 3.3 Multi-brain protocol (B)

| Actor | Authority level | Allowed actions | Inputs | Outputs | Escalation | Logging | Replayability |
|---|---|---|---|---|---|---|---|
| Sovereign Brain | Final authority | approve/reject decree, final arbitration | decree draft, advisory reports, risk flags | final decree decision | n/a (top of chain) | mandatory structured event + ledger | full input/output + decision metadata |
| Advisory Brains | Non-final, high influence | plan, critique, propose, challenge | task/decree context, policy, tool results | advisory report + confidence + citations | escalate to sovereign with challenge code | mandatory per-turn event with provider/model | deterministic replay from stored prompts + responses where possible |
| Tool Agents | No governance authority | execute bounded tool calls | signed tool request from kernel | tool result/artifacts/errors | escalate errors back to requesting advisory/kernel | tool call log + timing + artifact refs | request/response payload archived |
| Research/Execution Workers | No governance authority | orchestrate research steps | mission request + constraints | research bundle/report | escalate to advisory or sovereign seat | workflow-level trace + step events | stepwise replay over stored plan |

## 3.4 Provider/model arbitration (C)

### Provider registry
- Registry object keyed by provider ID:
  - endpoint, auth ref, capability labels, cost tier, latency SLO, reliability score.

### Model registry
- Distinct model catalog per provider:
  - model id, context size, specialization tags, safety profile.

### Seat-to-model mapping
- Replace loose `seat -> provider` only mapping with:
  - `seat -> arbitration policy`, policy resolves to `(provider, model)`.

### Default/fallback behavior
- Default chain per seat: `primary -> fallback_1 -> fallback_2`.
- If no seat policy found, use sovereign default profile.

### Timeout/error downgrade rules
- Timeout: move to next candidate with downgrade reason `timeout`.
- 5xx/provider outage: quarantine provider for cooldown window.
- Validation failure: retry once if idempotent, else escalate.

### Structured traces for model choice
Each model selection emits:
- candidate list,
- seat policy id,
- selected provider/model,
- rejection reasons for skipped candidates,
- latency + error stats used for decision.

## 3.5 DeerFlow integration strategy (D)

**Decision (provisional until repo inspection): separate worker runtime via adapter boundary**.

Justification:
- Operational isolation: tool crashes must not crash kernel.
- Observability: clean boundary events (`tool.requested`, `tool.completed`, `tool.failed`).
- Testing cost: adapter contract can be mocked without full DeerFlow stack.
- Failure containment: network-heavy research isolated from governance state machine.
- Upgrade path: DeerFlow can evolve independently with contract versioning.

Integration options rejected for default path:
- In-process library import into kernel: weak failure isolation.
- Hidden direct invocation from UI: bypasses governance.

## 3.6 paperclip integration strategy (E)

Because paperclip repo is currently unavailable, decision is staged:
- **Interim role hypothesis:** onboarding + governance console shell, no direct authority.
- Final role (onboarding shell vs operator cockpit vs thin admin UI) must be confirmed after repo inventory.

Hard constraint:
- paperclip must integrate only through kernel API/contracts, never direct provider authority.

## 3.7 Onboarding redesign (F)

First-run flow (kernel-driven, UI-rendered):
1. **Dynasty Identity**: set system/dynasty name + immutable identity seed.
2. **Sovereign Brain Selection**: choose sovereign seat/model policy.
3. **Advisory Seat Selection**: assign advisory seats and specializations.
4. **Provider Credentials/Endpoints**: register providers and auth secrets.
5. **Tool Layer Enablement**: enable DeerFlow adapters by capability toggle.
6. **Governance Defaults**: strict mode, escalation timeout, fallback policy.
7. **Operator Confirmation**: show signed summary; require explicit commit.

## 3.8 Terminology contract (G)

| Canonical term | Meaning | Former/legacy aliases to deprecate |
|---|---|---|
| dynasty | persistent system identity | org/company/workspace |
| sovereign brain | final authority model/seat path | CEO-only model wording |
| advisory brain | non-final specialist model path | assistant/worker model |
| seat | governance authority position | role/persona |
| decree | governance command/decision unit | task/command (ambiguous) |
| ledger | immutable accountability record | metrics log only |
| archive/scroll | historical record artifact | generic report |
| operator / steward | human controller | admin/user |
| mission / expedition / worktree | execution unit/context | job/project/task |

---

## 3.9 Terminology Layering Contract (kernel-safe naming boundary)

This contract defines a strict three-layer terminology boundary to prevent UI/brand copy from destabilizing kernel schemas or machine-queryable logs.

### Layer A — Kernel / Config / API / Log terminology (engineering-stable)
- Scope: persistence schema, config keys, runtime structs, API fields, event/log payload fields.
- Rule: names must remain machine-safe, explicit, and stable across migrations.
- Current examples that must remain stable:
  - `provider_registry`
  - `model_registry`
  - `seat_policies`
  - `tool_adapters`
  - `evolution`

### Layer B — UI / Operator terminology (display labels)
- Scope: onboarding copy, labels, operator-facing navigation text, tooltip/help language.
- Rule: display labels can vary by product-language guidance, but must map to Layer A stable IDs/keys.
- DTO/UI contracts should carry both:
  - stable `id` / `machine_name` (Layer A)
  - localized/display `label` (Layer B)

### Layer C — Narrative / Brand terminology
- Scope: brand art direction, thematic copywriting, marketing narratives.
- Rule: narrative terms must not become canonical config/event/persistence names unless explicitly approved through schema governance.

### Allowed vs forbidden mapping examples

Allowed:
- UI label `Sovereign Brain` -> internal key `brains.sovereign_seat`
- UI label `Research Tools` -> internal key `tool_adapters`
- UI label `Primary Council Model` -> internal key `seat_policies.<seat>.primary_model`

Forbidden:
- Renaming `provider_registry` to `dragon_veins` in runtime config schema
- Renaming `model_registry` to `imperial_seal` in persisted event payloads
- Storing only display-copy field names in logs/events without stable machine fields

### Product-language source constraint
- Product-language source documents (including Kimi CLI source copy) constrain Layer B/C wording.
- They do **not** redefine Layer A kernel schema, API names, or log field contracts.

### Logging and replay rule
- Stored logs/events remain technically legible and queryable using stable machine fields.
- UI may render sovereign/product terminology on top, but replay/search must operate on Layer A field names.

---

## 4) Boundary contracts between repos

## 4.1 Kernel ↔ Model arbitration (internal contract)
- Kernel calls arbitration module with:
  - seat id, decree context, constraints.
- Arbitration returns:
  - selected provider/model,
  - trace metadata,
  - fallback chain state.

## 4.2 Kernel ↔ Tool layer (external adapter contract)
- Kernel sends signed tool request envelope:
  - request id, decree id, capability, input payload hash, timeout, authority token.
- Tool runtime returns:
  - status, output artifact refs, execution trace summary, error classification.
- Tool layer cannot mutate governance state directly.

## 4.3 Kernel ↔ UI/operator layer
- UI submits commands as decree drafts.
- Kernel validates authority/state and returns state transitions.
- UI renders projections/read models; no direct state mutation outside kernel commands.

## 4.4 Contract versioning
- Start with `v1alpha` schema for adapter endpoints/events.
- All boundary payloads include `contract_version` + `schema_hash`.

---

## 5) Migration strategy

## Phase 0 (current): reconnaissance + architecture contract
- Completed for available repo(s).
- Blocked on missing deer-flow/paperclip checkouts for full tri-repo inventory.

## Phase 1: docs-first PR slice (safe/no behavior change)
- Add DragonCore 2.0 blueprint and explicit boundary contracts.
- No runtime behavior changes.

## Phase 2: config schema normalization
- Introduce normalized `brains`, `provider_registry`, `model_registry`, `seat_policies` sections.
- Keep backward-compatible parser for existing `providers` + `seat_models`.

## Phase 3: provider/model arbitration abstraction
- Extract arbitration into module with trace objects.
- Replace ad-hoc router selection calls with policy-based selection API.

## Phase 4: onboarding skeleton
- Add UI-agnostic onboarding state machine + DTO contract in kernel.
- paperclip wiring deferred until repo available.

## Phase 5: DeerFlow adapter boundary stub
- Add adapter trait + noop/mock worker implementation + tracing.
- No authority path granted.

## Phase 6: integration tests + migration notes
- Add minimal end-to-end tests across:
  - config parse,
  - model arbitration fallback,
  - onboarding state machine,
  - adapter boundary behavior,
  - regression checks.

---

## 6) PR slicing plan

### PR-1 (this slice): repo inventory + docs + boundary contract
- Deliverable: this blueprint document.
- Risk: low (docs only).
- Reversible: full (delete/revert doc commit).

### PR-2: config schema normalization (backward compatible)
- Add new schema structs + loader migration layer.
- Add tests for legacy + new config parse paths.

### PR-3: provider/model arbitration abstraction
- Introduce arbitration module and structured trace events.
- Update runtime call points.
- Terminology layering in PR-3:
  - internal trace/config keys stay Layer A (`provider_registry`, `model_registry`, `seat_policies`, `tool_adapters`, `evolution`)
  - any sovereign/thematic wording is display-label-only metadata (Layer B)
  - arbitration traces remain queryable/searchable by stable technical fields, not narrative labels.

### PR-4: onboarding skeleton + terminology
- Kernel onboarding flow object model.
- Terminology normalization constants and log fields.

### PR-5: DeerFlow adapter boundary stub
- Tool adapter trait + stub worker + strict authority guardrails.

### PR-6: integration tests + migration notes
- Add regression/integration matrix and migration playbook.

---

## 7) Risk register

| Risk | Severity | Likelihood | Detection | Mitigation |
|---|---:|---:|---|---|
| Config breakage during schema change | High | Medium | parse tests + startup validation | dual-read parser + explicit migration warnings |
| State schema breakage | High | Medium | run replay tests | versioned persisted schema + migration layer |
| Provider routing regressions | High | Medium | arbitration tests + event trace inspection | deterministic selection policy tests |
| Hidden centralization (tool layer becomes oracle) | High | Medium | authority audit tests | enforce no-tool-final-decision rule in kernel |
| Duplicate logic across kernel/UI/tool | Medium | High | code ownership review | strict boundary ownership matrix |
| Fragile glue code across missing repos | Medium | High | contract tests with mocks | contract-first design + versioning |
| Missing repo assumptions wrong | High | High (current) | pending repo inventory | block high-impact code changes until repos available |

---

## 8) Validation plan

For each PR slice, minimum reproducible checks:

1. Boot/compile validation
- `cargo check` and `cargo test` in `runtime/`.

2. Config parse validation
- Legacy config fixtures parse successfully.
- New schema fixtures parse successfully (from PR-2 onward).

3. Provider selection validation
- Deterministic seat policy selection tests.
- Fallback and timeout downgrade tests.

4. Onboarding state-machine validation
- Transition tests for required steps and invalid transitions.

5. Adapter boundary validation
- Ensure tool adapter cannot issue governance finalization actions.

6. Regression status
- Existing runtime tests remain green.

---

## 9) Explicit assumptions requiring human confirmation

1. `deer-flow` and `paperclip` repositories are not present in current workspace; this must be corrected before PR-2+ that requires cross-repo implementation.
2. DragonCore remains the sovereign kernel and sole source of governance truth (non-negotiable design guardrail).
3. paperclip should not hold provider credentials directly unless mediated by kernel-issued credential references.
4. DeerFlow integration is expected to be worker/adapter style, pending repo inspection.
5. Terminology migration (CEO-style labels -> sovereign terminology) may require staged compatibility labels to avoid operator confusion.
6. Existing production runtime behavior must remain default-compatible until explicit migration flags are enabled.

---

## Appendix A: dependency and ownership map (provisional)

| Concern | Owner repo | Notes |
|---|---|---|
| Governance state machine, seat authority, decree lifecycle | DragonCore | Canonical kernel ownership |
| Provider/model arbitration policy and traces | DragonCore | Kernel module boundary |
| Research/search/crawl/python/RAG/MCP execution | deer-flow (adapter runtime) | Non-sovereign tool execution only |
| Onboarding/control UI and operator cockpit | paperclip | Reads/writes via kernel contracts |
| Cross-repo contracts and versioning | Shared (kernel-led) | Contract schema authored from kernel authority |

## Appendix B: immediate blockers

- Missing local checkouts for `deer-flow` and `paperclip` prevents completing mandatory tri-repo runtime/config/test-entrypoint inventory from source.

---

## PR-2 execution update (schema normalization slice)

### Workspace inventory re-check (2026-03-26 UTC)
- Re-checked workspace inventory before coding PR-2.
- `DragonCore` remains present.
- `/workspace/deer-flow` and `/workspace/paperclip` are still absent.
- This does **not** block PR-2 because PR-2 is kernel-local; cross-repo blocker remains applicable for PR-4+ and PR-5+.

### PR-2 schema decisions (kernel-local)
- Added normalized config domains:
  - `brains`
  - `provider_registry`
  - `model_registry`
  - `seat_policies`
  - `tool_adapters` (stub-only)
  - `evolution` (forge extension point; disabled by default)
- Added root `config_schema_version` and `state_schema` compatibility planning markers.
- Added explicit dual-read normalization strategy:
  - legacy: `[providers]` + `[seat_models]` -> normalized internal representation
  - normalized: new schema blocks -> same normalized internal representation

### Normalization rules (PR-2)
- Legacy and normalized schema families are parsed by one `Config` object but normalized by explicit mode selection.
- Guardrails reject mixed conflicting definitions (no silent coercion):
  - legacy `providers` mixed with normalized `provider_registry`
  - legacy `seat_models` mixed with normalized `seat_policies`
- Validation now rejects:
  - unsupported schema versions
  - model entries that reference unknown providers
  - seat policies referencing unknown models
  - invalid fallback chains (duplicates, missing references, primary repeated in fallback)

### Deferred to PR-3
- Full arbitration extraction from runtime into dedicated arbitration engine.
- Runtime-wide propagation of detailed arbitration trace emission at each selection point.
- Policy-driven candidate scoring, outage cooldown orchestration, and retry policy execution.

### Evolution/Forge extension point note
- `evolution` config is reserved and parseable in PR-2.
- Default is disabled.
- No scheduling, autonomous loop, or runtime behavior tied to this block yet.
