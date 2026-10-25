# RelayForge foundation brief

This bounded brief prepares the foundation-contract slice. It summarizes the
owner's requirements and dated evidence; it does not select a product language
or claim an implementation. Read [the decision register](../00_brief/decisions.md)
for accepted decisions and [the roadmap](../roadmap.yaml) for admitted work.

## Outcome to optimize

Select an eligible model, native thinking effort, and worker profile that
completes the task reliably at the lowest measured total cost. Count parent
reasoning, worker attempts, tools, verification, failed recovery, repair, and
handoff. A higher-priced model can cost less when it avoids repeated failures.
Report completion rate alongside cost per accepted result and latency.

Compare exact model/effort/harness/instruction/tool/skill configurations.
Difficulty labels retain their initial estimate and provenance. Public
benchmarks and advertised prices support discovery but cannot certify local
coding performance. The Astra-effort versus Sol-max crossover is **unknown**;
no universal ordering or 100% reliability is established.

## Required boundaries

- An embeddable core and CLI serve Codex and Claude adapters first. Other
  harnesses, skill routing, MCP/tool narrowing, and execution gateways are later
  candidates whose added value must be measured.
- The foreground agent decomposes work and verifies completion. JEV answers
  bounded questions over registered eligible IDs; it does not generate code,
  invent profiles, or grant authority.
- Separate capability facts, observed outcomes, and routing policy. Observe the
  actual model and effort where exposed; missing runtime identity stays unknown.
- Record sessions, tasks, attempts, execution spans, tool outcomes, costs, and
  recovery links. Deliverable acceptance and operational failure are separate:
  eventual success does not erase earlier wrong calls or wasted spend.
- Several confirmed model-caused wrong tool calls constitute operational
  failure. The proposed threshold of three is unapproved and uncalibrated.
  Infrastructure faults and ambiguous causes retain separate attribution.
- Ordinary local event capture, counters, and consolidation require zero model
  calls. Use incremental records and cached statistics; call JEV only where
  unresolved routing or recovery decisions justify the cost.
- Persist during work. Reconcile at completion and next startup, deduplicate
  events, retain interrupted outcomes as unknown, and protect other active
  sessions from being falsely marked abandoned.
- Adapt rankings only within approved candidates and policy. Do not expand
  permissions, install tools, or change foreground settings through telemetry.
  Keep credentials, raw prompts/code/tool output, and private telemetry out of
  tracked artifacts.

## Observed gaps and unknowns

The 2026-10-25 audit found advisory JEV integrations with inconsistent candidate
families, no validated common model/effort registry, and written lane rules that
conflict with inspected foreground defaults. Claude had native subagents and
a JEV prompt hook; Codex's effective profile registration and dispatch were
unverified. Existing external JEV code contains useful inventory, routing, and
adapter contracts; presence on disk does not prove runtime wiring.

Both harnesses need verified event coverage, actual execution receipts, and
outcome attribution. Calibration, savings, cost accounting, allowed native
bindings, foreground mapping, and promotion thresholds remain unresolved.

## Additional research considerations

The [registry source review](registry-source-review-2026-10-25.md) separates
observed coding-agent PR outcomes from proposed JEV integrations. Consider these
details when defining the existing contracts; they do not admit new features:

- Separate technical verification, task acceptance, and PR integration outcomes.
  Abandonment, duplicates, interruption, and unwanted work are not automatically
  model reasoning failures. Keep full attempt/recovery history and denominators.
- Version JEV questions, criteria, candidate count/order, resolved model,
  thresholds, and calibration together. Provider Choice confidence depends on
  candidate count and is not an observed task-completion probability.
- Batch independent semantic questions over shared minimized state when useful;
  compute counts, prices, freshness, and failure rates in code. Do not make an
  extra model call for every observed event.
- Record relevant instruction/skill/tool/context configuration and actual
  recommendation adoption. A right suggestion does not ensure a right tool load.
- Any later semantic tool gate operates within existing deterministic permissions.
  Denied/deferred calls also belong in the cost/failure record. A generic MCP
  connection or prompt hint does not prove native call interception.

Read the detailed review only for those questions. The first slice remains the
bounded foundation contract; routing, gates, and skill selection are unimplemented.

The [Ten Levels source investigation](ten-levels-jev-source-review-2026-10-25.md)
adds concrete implementation limits: its router uses prose lanes, file filtering
makes one call per retained file, guard errors fail open, and compaction thresholds
advise the Pi agent rather than directly enforcing compaction. For the foundation,
retain attempt-level failure/cost records and explicit adapter capabilities;
compact answers do not prove lower total cost or correctness. Raw request retention
and arbitrary command execution are not default RelayForge telemetry features.
Read that source review only when designing the corresponding contract or later
experiment; the ten demonstration levels do not expand the admitted scope.

The [session handoff review](session-handoff-source-review-2026-10-25.md) adds one
measurement boundary: compact continuation views must preserve references while
leaving failure/cost accounting intact. Record handoff policy and adapter evidence
coverage so continuation changes are not misattributed to model/effort capability.

The [issue triage review](issue-triage-source-review-2026-10-25.md) separates task
category, readiness, difficulty and worker suitability. Good category prediction
does not validate readiness or model/effort success. Retain input coverage and
rubric provenance; evaluate each signal against independent outcomes and baselines.

The [seven-repository source review](jev-repository-roundup-2026-10-25.md) adds
route-adoption stages (suggested/applied/sent/confirmed), continuation pinning,
persist-before-dispatch and verify-before-complete ordering, and stable evidence
handles. Preserve unknown usage instead of zero defaults. Its small replay examples
do not validate a general adaptive router; compare fixed reasonable baselines.

## Foundation slice output

Specify the minimum contracts and compare at most two non-Python toolchain
options. Rust is a proposal; the owner has not selected a product language.
Python 3.11+/PyYAML run the retained template workflow only. Storage is also
undecided; SQLite is a candidate. Admit the disposable exact-toolchain canary
only after the relevant decision is recorded. No RelayForge runtime is built.

Use [phase-one spec](phase1-spec.md) for named contracts and R1–R37 acceptance
proposals; use [evidence brief](evidence-brief.md) for source provenance and
historical audit detail. Open relevant sections when needed, not both full
documents at every session start.
